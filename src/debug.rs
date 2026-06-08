use crate::cpu::CpuDebugSnapshot;
use image::{Rgb, RgbImage};
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub struct Tile(pub [[Rgb<u8>; 8]; 8]);

const STARTUP_INSTRUCTION_LIMIT: usize = 1000;
const STARTUP_TRACE_WINDOW: Duration = Duration::from_secs(3);

pub struct StartupInstructionTrace {
    started_at: Instant,
    path: PathBuf,
    entries: Vec<String>,
    saved: bool,
}

impl StartupInstructionTrace {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            started_at: Instant::now(),
            path: path.into(),
            entries: Vec::with_capacity(STARTUP_INSTRUCTION_LIMIT),
            saved: false,
        }
    }

    pub fn record(&mut self, snapshot: &CpuDebugSnapshot) -> io::Result<()> {
        if self.saved {
            return Ok(());
        }

        if self.entries.len() < STARTUP_INSTRUCTION_LIMIT
            && self.started_at.elapsed() <= STARTUP_TRACE_WINDOW
        {
            self.entries.push(format_trace_entry(snapshot));
        }

        self.save_if_complete()
    }

    pub fn save_if_complete(&mut self) -> io::Result<()> {
        if !self.saved
            && (self.entries.len() >= STARTUP_INSTRUCTION_LIMIT
                || self.started_at.elapsed() >= STARTUP_TRACE_WINDOW)
        {
            self.save()?;
            self.saved = true;
        }

        Ok(())
    }

    fn save(&self) -> io::Result<()> {
        let file = File::create(&self.path)?;
        let mut writer = BufWriter::new(file);

        writeln!(
            writer,
            "Startup instruction trace: {} instructions captured in {:.3}s",
            self.entries.len(),
            self.started_at.elapsed().as_secs_f64()
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

fn format_trace_entry(snapshot: &CpuDebugSnapshot) -> String {
    format!(
        "#{:04} PC=${:04X} A=${:02X} X=${:02X} Y=${:02X} SP=${:02X} P=${:02X} CYC={} OK={} {}",
        snapshot.instruction_count,
        snapshot.program_counter,
        snapshot.accumulator,
        snapshot.x,
        snapshot.y,
        snapshot.stack_pointer,
        snapshot.processor_status,
        snapshot.total_cpu_cycles,
        snapshot.last_instruction_success,
        snapshot.current_instruction
    )
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
