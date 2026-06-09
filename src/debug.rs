use crate::cpu::CpuDebugSnapshot;
use crate::ppu::PpuDebugSnapshot;
use image::{Rgb, RgbImage};
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub struct Tile(pub [[Rgb<u8>; 8]; 8]);

const STARTUP_INSTRUCTION_LIMIT: usize = 10_000;
const STARTUP_TRACE_WINDOW: Duration = Duration::from_secs(3);
const STARTUP_TRACE_START_ADDRESS: u16 = 0x8000;

pub struct StartupInstructionTrace {
    started_at: Option<Instant>,
    path: PathBuf,
    entries: Vec<String>,
    saved: bool,
    observed_vblank_starts: u64,
    was_in_vblank: bool,
    entry_keys: Vec<Option<TraceRepeatKey>>,
    pending_loop: Option<PendingLoop>,
}

struct PendingLoop {
    body_keys: Vec<TraceRepeatKey>,
    cursor: usize,
    skipped_iterations: usize,
    current_iteration: Vec<(TraceRepeatKey, String)>,
    last_iteration: Vec<(TraceRepeatKey, String)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TraceRepeatKey {
    instruction_address: u16,
    program_counter: u16,
    instruction: String,
}

impl StartupInstructionTrace {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            started_at: None,
            path: path.into(),
            entries: Vec::with_capacity(STARTUP_INSTRUCTION_LIMIT),
            saved: false,
            observed_vblank_starts: 0,
            was_in_vblank: false,
            entry_keys: Vec::with_capacity(STARTUP_INSTRUCTION_LIMIT),
            pending_loop: None,
        }
    }

    pub fn record(
        &mut self,
        cpu_snapshot: &CpuDebugSnapshot,
        ppu_snapshot: &PpuDebugSnapshot,
    ) -> io::Result<()> {
        if self.saved {
            return Ok(());
        }

        self.observe_ppu(ppu_snapshot);

        if self.started_at.is_none()
            && cpu_snapshot.instruction_address == STARTUP_TRACE_START_ADDRESS
        {
            self.started_at = Some(Instant::now());
        }

        if self.should_record_next_entry() {
            let entry = format_trace_entry(cpu_snapshot, ppu_snapshot);
            let key = TraceRepeatKey::from(cpu_snapshot);
            self.push_entry(key, entry);
        }

        self.save_if_complete()
    }

    pub fn save_if_complete(&mut self) -> io::Result<()> {
        if let Some(started_at) = self.started_at {
            if !self.saved
                && (self.entries.len() >= STARTUP_INSTRUCTION_LIMIT
                    || started_at.elapsed() >= STARTUP_TRACE_WINDOW)
            {
                self.flush_pending_loop();
                self.save(started_at)?;
                self.saved = true;
            }
        }

        Ok(())
    }

    fn observe_ppu(&mut self, snapshot: &PpuDebugSnapshot) {
        if snapshot.in_vblank && !self.was_in_vblank {
            self.observed_vblank_starts += 1;
        }

        self.was_in_vblank = snapshot.in_vblank;
    }

    fn should_record_next_entry(&self) -> bool {
        self.started_at
            .map(|started_at| {
                self.entries.len() < STARTUP_INSTRUCTION_LIMIT
                    && started_at.elapsed() <= STARTUP_TRACE_WINDOW
            })
            .unwrap_or(false)
    }

    fn push_entry(&mut self, key: TraceRepeatKey, entry: String) {
        if let Some(pending_loop) = &mut self.pending_loop {
            if pending_loop.body_keys[pending_loop.cursor] == key {
                pending_loop.current_iteration.push((key.clone(), entry));
                pending_loop.cursor += 1;

                if pending_loop.cursor == pending_loop.body_keys.len() {
                    pending_loop.skipped_iterations += 1;
                    pending_loop.last_iteration =
                        std::mem::take(&mut pending_loop.current_iteration);
                    pending_loop.cursor = 0;
                }

                return;
            }

            self.flush_pending_loop();
        }

        self.entry_keys.push(Some(key.clone()));
        self.entries.push(entry);
        self.arm_loop_compaction(key);
    }

    fn arm_loop_compaction(&mut self, key: TraceRepeatKey) {
        if key.program_counter > key.instruction_address {
            return;
        }

        let Some(loop_start_index) = self.entry_keys.iter().rposition(|entry_key| {
            entry_key
                .as_ref()
                .map(|entry_key| entry_key.instruction_address == key.program_counter)
                .unwrap_or(false)
        }) else {
            return;
        };

        let body_keys = self.entry_keys[loop_start_index..]
            .iter()
            .filter_map(Clone::clone)
            .collect::<Vec<_>>();

        if body_keys.is_empty() || body_keys.len() > 16 {
            return;
        }

        self.pending_loop = Some(PendingLoop {
            body_keys,
            cursor: 0,
            skipped_iterations: 0,
            current_iteration: Vec::new(),
            last_iteration: Vec::new(),
        });
    }

    fn flush_pending_loop(&mut self) {
        let Some(pending_loop) = self.pending_loop.take() else {
            return;
        };

        if pending_loop.skipped_iterations > 0 {
            self.entries.push(format!(
                "... repeated previous loop body {} more times; middle removed ...",
                pending_loop.skipped_iterations
            ));
            self.entry_keys.push(None);
        }

        let bridge_entries = if pending_loop.current_iteration.is_empty() {
            pending_loop.last_iteration
        } else {
            pending_loop.current_iteration
        };

        if pending_loop.skipped_iterations == 0 {
            return;
        }

        for (key, entry) in bridge_entries {
            self.entry_keys.push(Some(key));
            self.entries.push(entry);
        }
    }

    fn save(&self, started_at: Instant) -> io::Result<()> {
        let file = File::create(&self.path)?;
        let mut writer = BufWriter::new(file);

        writeln!(
            writer,
            "Startup instruction trace: {} entries captured in {:.3}s from IP=${:04X} after {} vblank starts",
            self.entries.len(),
            started_at.elapsed().as_secs_f64(),
            STARTUP_TRACE_START_ADDRESS,
            self.observed_vblank_starts
        )?;

        for entry in &self.entries {
            writeln!(writer, "{entry}")?;
        }

        Ok(())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

fn format_trace_entry(cpu_snapshot: &CpuDebugSnapshot, ppu_snapshot: &PpuDebugSnapshot) -> String {
    format!(
        "#{:04} IP=${:04X} PC=${:04X} A=${:02X} X=${:02X} Y=${:02X} SP=${:02X} P=${:02X} CYC={} PPU=F{} S{} D{} VB={} OK={} {}",
        cpu_snapshot.instruction_count,
        cpu_snapshot.instruction_address,
        cpu_snapshot.program_counter,
        cpu_snapshot.accumulator,
        cpu_snapshot.x,
        cpu_snapshot.y,
        cpu_snapshot.stack_pointer,
        cpu_snapshot.processor_status,
        cpu_snapshot.total_cpu_cycles,
        ppu_snapshot.frame,
        ppu_snapshot.scanline,
        ppu_snapshot.dot,
        ppu_snapshot.in_vblank,
        cpu_snapshot.last_instruction_success,
        cpu_snapshot.current_instruction
    )
}

impl From<&CpuDebugSnapshot> for TraceRepeatKey {
    fn from(snapshot: &CpuDebugSnapshot) -> Self {
        Self {
            instruction_address: snapshot.instruction_address,
            program_counter: snapshot.program_counter,
            instruction: snapshot.current_instruction.clone(),
        }
    }
}

#[allow(dead_code)]
pub fn hex_print_word(word: u16) {
    println!("0x{:04X}", word);
}

#[allow(dead_code)]
pub fn hex_print_byte(byte: u8) {
    println!("0x{:02X}", byte);
}

const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
const DARK_GRAY: Rgb<u8> = Rgb([70, 70, 70]);
const GRAY: Rgb<u8> = Rgb([140, 140, 140]);
const LIGHT_GRAY: Rgb<u8> = Rgb([210, 210, 210]);

/// Takes 16 bytes and decodes them into an 8x8 rgb tile
pub fn deinterlace_tile_bytes(tile_bytes: &[u8]) -> Tile {
    assert_eq!(tile_bytes.len(), 16);

    let plane = tile_bytes.split_at(8);
    let mut tile_raw = [[Rgb([0, 0, 0]); 8]; 8];
    //let mut rgb: RgbImage = RgbImage::new(8, 8);

    #[allow(clippy::needless_range_loop)]
    for y in 0..8 {
        for x in 0..8 {
            let bitmask = 0b1000_0000 >> x;
            let bits = (plane.0[y] & bitmask != 0, plane.1[y] & bitmask != 0);

            let color_index = u8::from(bits.0) + (u8::from(bits.1) << 1);

            tile_raw[y][x] = match color_index {
                0 => BLACK,
                1 => DARK_GRAY,
                2 => GRAY,
                3 => LIGHT_GRAY,
                _ => panic!("Invalid color index."),
            }
        }
    }

    Tile(tile_raw)
}

/// Stitches together 512 tiles into two 16x16 tile grids, which then attach
/// to each other side by side, where each sequential 16 tiles is a row.
/// Turns the stitched result into an [`RgbImage`]
pub fn stitch_tiles(tiles: &[Tile]) -> RgbImage {
    let mut img = RgbImage::new(256, 128);

    assert_eq!(tiles.len(), 512);

    let left_tiles = &tiles[0..256];

    for (logical_row, tiles_on_row) in left_tiles.chunks(16).enumerate() {
        for subrow in 0..8 {
            let pixels = tiles_on_row
                .iter()
                .flat_map(|tile| tile.0[subrow])
                .collect::<Vec<Rgb<u8>>>();

            let y = (logical_row * 8) + subrow;
            for x in 0..(8 * 16) {
                *img.get_pixel_mut(x, y as u32) = pixels[x as usize];
            }
        }
    }

    let right_tiles = &tiles[256..512];

    for (logical_row, tiles_on_row) in right_tiles.chunks(16).enumerate() {
        for subrow in 0..8 {
            let pixels = tiles_on_row
                .iter()
                .flat_map(|tile| tile.0[subrow])
                .collect::<Vec<Rgb<u8>>>();

            let y = (logical_row * 8) + subrow;
            for x in 0..(8 * 16) {
                *img.get_pixel_mut(x + 128, y as u32) = pixels[x as usize];
            }
        }
    }

    img
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cpu_snapshot(
        instruction_count: u64,
        instruction_address: u16,
        program_counter: u16,
    ) -> CpuDebugSnapshot {
        CpuDebugSnapshot {
            instruction_address,
            program_counter,
            accumulator: 0x01,
            x: 0x02,
            y: 0x03,
            stack_pointer: 0xFD,
            processor_status: 0x24,
            current_instruction: "Instruction { opcode: NOP }".to_string(),
            last_instruction_success: true,
            total_cpu_cycles: instruction_count * 2,
            instruction_count,
        }
    }

    fn ppu_snapshot(in_vblank: bool) -> PpuDebugSnapshot {
        PpuDebugSnapshot {
            scanline: if in_vblank { 241 } else { 0 },
            dot: 0,
            frame: 0,
            in_vblank,
        }
    }

    #[test]
    fn startup_trace_starts_at_8000() {
        let mut trace = StartupInstructionTrace::new("unused_startup_trace.txt");

        trace
            .record(&cpu_snapshot(1, 0x90D8, 0x90DA), &ppu_snapshot(false))
            .unwrap();

        assert!(trace.started_at.is_none());
        assert!(trace.entries.is_empty());

        trace
            .record(&cpu_snapshot(2, 0x8000, 0x8002), &ppu_snapshot(true))
            .unwrap();

        assert!(trace.started_at.is_some());
        assert_eq!(trace.entries.len(), 1);
        assert!(trace.entries[0].contains("IP=$8000 PC=$8002"));
    }

    #[test]
    fn startup_trace_entries_include_ppu_position() {
        let entry = format_trace_entry(&cpu_snapshot(7, 0x8000, 0x8002), &ppu_snapshot(true));

        assert!(entry.contains("IP=$8000 PC=$8002"));
        assert!(entry.contains("PPU=F0 S241 D0 VB=true"));
    }

    #[test]
    fn startup_trace_compacts_small_backward_loops_with_exit_bridge() {
        let mut trace = StartupInstructionTrace::new("unused_startup_trace.txt");

        trace
            .record(&cpu_snapshot(1, 0x8000, 0x8002), &ppu_snapshot(false))
            .unwrap();
        trace
            .record(&cpu_snapshot(2, 0x800A, 0x800D), &ppu_snapshot(false))
            .unwrap();
        trace
            .record(&cpu_snapshot(3, 0x800D, 0x800A), &ppu_snapshot(false))
            .unwrap();
        trace
            .record(&cpu_snapshot(4, 0x800A, 0x800D), &ppu_snapshot(false))
            .unwrap();
        trace
            .record(&cpu_snapshot(5, 0x800D, 0x800A), &ppu_snapshot(false))
            .unwrap();
        trace
            .record(&cpu_snapshot(6, 0x800A, 0x800D), &ppu_snapshot(false))
            .unwrap();
        trace
            .record(&cpu_snapshot(7, 0x800D, 0x800F), &ppu_snapshot(false))
            .unwrap();

        assert_eq!(trace.entries.len(), 6);
        assert!(trace.entries[3].contains("repeated previous loop body 1 more times"));
        assert!(trace.entries[4].contains("#0006 IP=$800A PC=$800D"));
        assert!(trace.entries[5].contains("#0007 IP=$800D PC=$800F"));
    }
}
