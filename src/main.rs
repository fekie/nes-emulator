//! System info:
use apu::Apu;
use cartridge::Cartridge;
use clap::Parser;
use cpu::{CpuContainer, CpuDebugSnapshot};
use debug::{StartupInstructionTrace, Tile};
use ines::Ines;
/// - System Type: NTSC
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use ppu::Ppu;
use rgb::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::spawn;
use std::time::Instant;

const WIDTH: usize = 256;
const HEIGHT: usize = 240;
const DEBUG_PANEL_WIDTH: usize = 152;
const APP_WIDTH: usize = WIDTH + DEBUG_PANEL_WIDTH;
const SCALE: usize = 4;
const SCREEN_WIDTH: usize = APP_WIDTH * SCALE;
const SCREEN_HEIGHT: usize = HEIGHT * SCALE;

// Speeds taken from https://www.nesdev.org/wiki/CPU
const CORE_CLOCK_HZ: u64 = 21_441_960;
const CLOCK_DIVISOR: u64 = 12;
const CPU_HZ: u64 = CORE_CLOCK_HZ / CLOCK_DIVISOR;
const FRAME_INTERVAL_SECS: f64 = 1.0 / TARGET_FPS as f64;

const PPU_CLOCK_DIVISOR: u8 = 4;
const TARGET_FPS: usize = 60;

mod apu;
mod cartridge;
mod cpu;
mod debug;
mod ines;
mod ppu;

pub struct MapperType {}

#[derive(Clone, Copy, Debug, Default)]
struct ColorToggles {
    orange: bool,
    indigo: bool,
}

impl ColorToggles {
    fn frame_color(self) -> Rgb<u8> {
        match (self.orange, self.indigo) {
            (false, false) => Rgb {
                r: 255,
                g: 255,
                b: 0,
            },
            (true, false) => Rgb {
                r: 255,
                g: 108,
                b: 0,
            },
            (false, true) => Rgb {
                r: 70,
                g: 120,
                b: 255,
            },
            (true, true) => Rgb {
                r: 220,
                g: 70,
                b: 255,
            },
        }
    }
}

#[derive(Default, Debug)]
enum Keycode {
    #[default]
    Placeholder,
    ToggleOrange,
    ToggleIndigo,
}

#[derive(Debug)]
struct FrameFinishedSignal {
    /// The key that was pressed down just after the newly created frame.
    current_keycode: Keycode,
    /// The additional seconds that we need to run cycles for because a frame was delayed.
    delay_debt_s: f64,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path of the rom to load into the program.
    #[arg(short, long)]
    rom: String,
    /// Prints the CHR-ROM pattern table to the terminal.
    #[clap(short, long, default_value = None)]
    pattern_table: bool,
}

// Using an array instead of a vector will lead to a stackoverflow, even when Box'ing.
pub struct Pixels(Mutex<Vec<u32>>);

impl Pixels {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(Mutex::new(vec![0; WIDTH * HEIGHT]))
    }

    pub fn read(&self, x: usize, y: usize) -> Rgb<u8> {
        let i = (y * WIDTH) + x;
        let raw: u32 = self.0.lock().unwrap()[i];

        let r = ((raw >> 16) & 0xF) as u8;
        let g = ((raw >> 8) & 0xF) as u8;
        let b = (raw & 0xF) as u8;

        Rgb { r, g, b }
    }

    pub fn write(&self, x: usize, y: usize, rgb: Rgb<u8>) {
        let i = (y * WIDTH) + x;
        self.0.lock().unwrap()[i] = ((rgb.r as u32) << 16) | ((rgb.g as u32) << 8) | (rgb.b as u32);
    }

    pub fn copy_to_buffer(&self, buffer: &mut [u32]) {
        buffer.copy_from_slice(self.0.lock().unwrap().as_slice())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let rom = Ines::parse(&std::fs::read(&args.rom)?);

    // If we need to run debug routines, run the routine and then exit.
    if check_and_run_debug(&args, &rom) {
        return Ok(());
    }

    let pixels = Arc::new(Pixels::new());
    let cpu_debug = Arc::new(Mutex::new(CpuDebugSnapshot::default()));
    let mut buffer = vec![0; APP_WIDTH * HEIGHT];

    let thread_1_pixels = pixels.clone();
    let thread_1_cpu_debug = cpu_debug.clone();

    let (tx, rx) = crossbeam_channel::unbounded::<FrameFinishedSignal>();

    // After every frame, we process the appropriate amount of clock cycles.
    let _ = spawn(move || {
        let cpu = Rc::new(RefCell::new(CpuContainer::new()));
        let ppu = Rc::new(RefCell::new(Ppu::new()));
        let apu = Rc::new(RefCell::new(Apu::new()));
        let cartridge = Rc::new(RefCell::new(Cartridge::new(rom)));

        cpu.borrow_mut()
            .initialize(ppu.clone(), apu.clone(), cartridge.clone());
        ppu.borrow_mut().initialize(cpu.clone());
        apu.borrow_mut().initialize();
        cartridge.borrow_mut().initialize(cpu.clone(), ppu.clone());

        assert!(cpu.borrow().initialized());
        assert!(ppu.borrow().initialized());
        assert!(apu.borrow().initialized());
        assert!(cartridge.borrow().initialized());

        // If we execute an instruction and it takes more cycles than we have available,
        // then we store the amount here (as a negative number) that we need to take off.
        let mut cpu_cycle_debt: i64 = 0;

        // A wrapping machine cycle counter that allows us to tell when we need to clock the PPU;
        let mut current_machine_cycles = 0;
        let mut debug_snapshot = CpuDebugSnapshot::default();
        let mut startup_instruction_trace =
            StartupInstructionTrace::new("startup_instruction_trace.txt");

        while let Ok(frame_finished_signal) = rx.recv() {
            if let Err(err) = startup_instruction_trace.save_if_complete() {
                eprintln!(
                    "Failed to save startup instruction trace to {}: {err}",
                    startup_instruction_trace.path().display()
                );
            }

            match frame_finished_signal.current_keycode {
                Keycode::Placeholder => {}
                Keycode::ToggleOrange | Keycode::ToggleIndigo => {}
            }

            // Do cycles for (FRAME_INTERVAL_SECS + delay_debt_s) * CPU_HZ
            let mut available_cpu_cycles =
                ((FRAME_INTERVAL_SECS + frame_finished_signal.delay_debt_s) * CPU_HZ as f64) as i64
                    + cpu_cycle_debt;

            // Each time we do a cpu instruction, find out how many clock cycles it
            // took, multiply by 12, and then run that many master clock cycles on the bus.
            loop {
                // run a cpu full instruction cycle and see how many cpu cycles were taken
                let cpu_cycles_taken = cpu.borrow_mut().cycle_debug(&mut debug_snapshot);

                if cpu_cycles_taken == 0 {
                    *thread_1_cpu_debug.lock().unwrap() = debug_snapshot.clone();
                    break;
                }

                if let Err(err) = startup_instruction_trace.record(&debug_snapshot) {
                    eprintln!(
                        "Failed to save startup instruction trace to {}: {err}",
                        startup_instruction_trace.path().display()
                    );
                }

                available_cpu_cycles -= cpu_cycles_taken as i64;

                let machine_cycles_taken = cpu_cycles_taken * CLOCK_DIVISOR as u8;

                for _ in 0..machine_cycles_taken {
                    // clock ppu every 4 cycles
                    if current_machine_cycles % PPU_CLOCK_DIVISOR == 0 {
                        ppu.borrow_mut().clock(&thread_1_pixels);
                    }

                    // clock cartridge every tick
                    cartridge.borrow_mut().clock();

                    current_machine_cycles = current_machine_cycles.wrapping_add(1);
                }

                // If we're out of cpu cycles, record how much we went over and then
                // stop running cycles until the next request
                if available_cpu_cycles <= 0 {
                    cpu_cycle_debt = available_cpu_cycles;
                    *thread_1_cpu_debug.lock().unwrap() = debug_snapshot.clone();
                    break;
                }
            }
        }
    });

    // Rendering

    let mut window = Window::new(
        "Test - ESC to exit",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    let mut previous_frame_stamp = Instant::now();
    let mut color_toggles = ColorToggles::default();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        /* for i in buffer.iter_mut() {
            *i = v; // write something more funny here!
            v += 1;
            v += v.ilog(4);
        } */

        let mut current_keycode = Keycode::Placeholder;
        if window.is_key_pressed(Key::O, KeyRepeat::No) {
            color_toggles.orange = !color_toggles.orange;
            current_keycode = Keycode::ToggleOrange;
        }
        if window.is_key_pressed(Key::I, KeyRepeat::No) {
            color_toggles.indigo = !color_toggles.indigo;
            current_keycode = Keycode::ToggleIndigo;
        }

        let frame_color = color_toggles.frame_color();
        for i in 0..WIDTH * HEIGHT {
            let y = i / WIDTH;
            let x = i % WIDTH;
            pixels.write(x, y, frame_color);
        }

        draw_app_frame(
            &mut buffer,
            &pixels,
            &cpu_debug.lock().unwrap(),
            color_toggles,
        );
        window
            .update_with_buffer(&buffer, APP_WIDTH, HEIGHT)
            .unwrap();

        let delay_debt_s = previous_frame_stamp.elapsed().as_secs_f64() - FRAME_INTERVAL_SECS;

        tx.send(FrameFinishedSignal {
            current_keycode,
            delay_debt_s,
        })
        .unwrap();

        // Don't know why this works better below the tx.send but it does,
        // even though normally it should be *right* after the frame technically.
        // Move it back if it has issues.
        previous_frame_stamp = Instant::now();
    }

    Ok(())
}

fn draw_app_frame(
    buffer: &mut [u32],
    pixels: &Pixels,
    cpu_debug: &CpuDebugSnapshot,
    color_toggles: ColorToggles,
) {
    buffer.fill(0x111318);

    let pixels = pixels.0.lock().unwrap();
    for y in 0..HEIGHT {
        let source_row = y * WIDTH;
        let target_row = y * APP_WIDTH;
        buffer[target_row..target_row + WIDTH]
            .copy_from_slice(&pixels[source_row..source_row + WIDTH]);
    }

    draw_debug_panel(buffer, cpu_debug, color_toggles);
}

fn draw_debug_panel(buffer: &mut [u32], cpu_debug: &CpuDebugSnapshot, color_toggles: ColorToggles) {
    let left = WIDTH;

    for y in 0..HEIGHT {
        buffer[y * APP_WIDTH + left] = 0x2A2F3A;
    }

    draw_text(buffer, left + 10, 10, "CPU DEBUG", 0xE6EDF3);
    draw_text(
        buffer,
        left + 10,
        28,
        &format!("PC  ${:04X}", cpu_debug.program_counter),
        0xC9D1D9,
    );
    draw_text(
        buffer,
        left + 10,
        40,
        &format!(
            "A {:02X} X {:02X} Y {:02X}",
            cpu_debug.accumulator, cpu_debug.x, cpu_debug.y
        ),
        0xC9D1D9,
    );
    draw_text(
        buffer,
        left + 10,
        52,
        &format!(
            "SP {:02X} P {:02X}",
            cpu_debug.stack_pointer, cpu_debug.processor_status
        ),
        0xC9D1D9,
    );
    draw_text(
        buffer,
        left + 10,
        70,
        &format!("CYC {}", cpu_debug.total_cpu_cycles),
        0xA5D6FF,
    );
    draw_text(
        buffer,
        left + 10,
        82,
        &format!("INS {}", cpu_debug.instruction_count),
        0xA5D6FF,
    );
    draw_text(
        buffer,
        left + 10,
        100,
        if cpu_debug.last_instruction_success {
            "STATUS OK"
        } else {
            "STATUS WAIT"
        },
        if cpu_debug.last_instruction_success {
            0x7EE787
        } else {
            0xF2CC60
        },
    );
    draw_text(
        buffer,
        left + 10,
        112,
        &format!(
            "O {} I {}",
            toggle_label(color_toggles.orange),
            toggle_label(color_toggles.indigo)
        ),
        0xFFA657,
    );

    draw_text(buffer, left + 10, 132, "CURRENT", 0xE6EDF3);
    for (index, line) in wrap_debug_text(&cpu_debug.current_instruction, 21)
        .iter()
        .take(8)
        .enumerate()
    {
        draw_text(buffer, left + 10, 146 + (index * 12), line, 0xC9D1D9);
    }
}

fn toggle_label(enabled: bool) -> &'static str {
    if enabled {
        "ON"
    } else {
        "OFF"
    }
}

fn wrap_debug_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        if !current.is_empty() && current.len() + word.len() + 1 > width {
            lines.push(current);
            current = String::new();
        }

        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
}

fn draw_text(buffer: &mut [u32], x: usize, y: usize, text: &str, color: u32) {
    for (index, character) in text.chars().enumerate() {
        draw_character(buffer, x + index * 6, y, character, color);
    }
}

fn draw_character(buffer: &mut [u32], x: usize, y: usize, character: char, color: u32) {
    for (row, bits) in glyph(character).iter().enumerate() {
        for col in 0..5 {
            if bits & (1 << (4 - col)) != 0 {
                let target_x = x + col;
                let target_y = y + row;
                if target_x < APP_WIDTH && target_y < HEIGHT {
                    buffer[target_y * APP_WIDTH + target_x] = color;
                }
            }
        }
    }
}

fn glyph(character: char) -> [u8; 7] {
    match character.to_ascii_uppercase() {
        'A' => [0x0E, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'B' => [0x1E, 0x11, 0x11, 0x1E, 0x11, 0x11, 0x1E],
        'C' => [0x0E, 0x11, 0x10, 0x10, 0x10, 0x11, 0x0E],
        'D' => [0x1E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1E],
        'E' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x1F],
        'F' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x10],
        'G' => [0x0E, 0x11, 0x10, 0x17, 0x11, 0x11, 0x0E],
        'H' => [0x11, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'I' => [0x1F, 0x04, 0x04, 0x04, 0x04, 0x04, 0x1F],
        'J' => [0x01, 0x01, 0x01, 0x01, 0x11, 0x11, 0x0E],
        'K' => [0x11, 0x12, 0x14, 0x18, 0x14, 0x12, 0x11],
        'L' => [0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1F],
        'M' => [0x11, 0x1B, 0x15, 0x15, 0x11, 0x11, 0x11],
        'N' => [0x11, 0x19, 0x15, 0x13, 0x11, 0x11, 0x11],
        'O' => [0x0E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'P' => [0x1E, 0x11, 0x11, 0x1E, 0x10, 0x10, 0x10],
        'Q' => [0x0E, 0x11, 0x11, 0x11, 0x15, 0x12, 0x0D],
        'R' => [0x1E, 0x11, 0x11, 0x1E, 0x14, 0x12, 0x11],
        'S' => [0x0F, 0x10, 0x10, 0x0E, 0x01, 0x01, 0x1E],
        'T' => [0x1F, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04],
        'U' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'V' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x0A, 0x04],
        'W' => [0x11, 0x11, 0x11, 0x15, 0x15, 0x15, 0x0A],
        'X' => [0x11, 0x11, 0x0A, 0x04, 0x0A, 0x11, 0x11],
        'Y' => [0x11, 0x11, 0x0A, 0x04, 0x04, 0x04, 0x04],
        'Z' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x10, 0x1F],
        '0' => [0x0E, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0E],
        '1' => [0x04, 0x0C, 0x04, 0x04, 0x04, 0x04, 0x0E],
        '2' => [0x0E, 0x11, 0x01, 0x02, 0x04, 0x08, 0x1F],
        '3' => [0x1E, 0x01, 0x01, 0x0E, 0x01, 0x01, 0x1E],
        '4' => [0x02, 0x06, 0x0A, 0x12, 0x1F, 0x02, 0x02],
        '5' => [0x1F, 0x10, 0x10, 0x1E, 0x01, 0x01, 0x1E],
        '6' => [0x0E, 0x10, 0x10, 0x1E, 0x11, 0x11, 0x0E],
        '7' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08],
        '8' => [0x0E, 0x11, 0x11, 0x0E, 0x11, 0x11, 0x0E],
        '9' => [0x0E, 0x11, 0x11, 0x0F, 0x01, 0x01, 0x0E],
        '$' => [0x04, 0x0F, 0x14, 0x0E, 0x05, 0x1E, 0x04],
        ':' => [0x00, 0x04, 0x04, 0x00, 0x04, 0x04, 0x00],
        ',' => [0x00, 0x00, 0x00, 0x00, 0x04, 0x04, 0x08],
        '_' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1F],
        '(' => [0x02, 0x04, 0x08, 0x08, 0x08, 0x04, 0x02],
        ')' => [0x08, 0x04, 0x02, 0x02, 0x02, 0x04, 0x08],
        ' ' => [0x00; 7],
        _ => [0x1F, 0x01, 0x02, 0x04, 0x04, 0x00, 0x04],
    }
}

/// Checks if we need any debug routines to run, such as saving
/// pattern table data to a .png.
///
/// Returns true if we ran any debug routine.
fn check_and_run_debug(args: &Args, rom: &Ines) -> bool {
    if args.pattern_table {
        print_pattern_tables(args, rom);
        return true;
    }

    false
}

fn print_pattern_tables(args: &Args, rom: &Ines) {
    let pattern_bytes = &rom.character_rom[0..=0x1FFF];
    let tiles = pattern_bytes
        .chunks(16)
        .map(debug::deinterlace_tile_bytes)
        .collect::<Vec<Tile>>();

    let img = debug::stitch_tiles(&tiles);
    img.save("pattern_table.png").unwrap();
    println!("Saved pattern table to pattern_table.png");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_panel_expands_frame_and_draws_snapshot() {
        let pixels = Pixels::new();
        pixels.write(
            0,
            0,
            Rgb {
                r: 0xAA,
                g: 0x55,
                b: 0x11,
            },
        );

        let snapshot = CpuDebugSnapshot {
            program_counter: 0xC000,
            accumulator: 0x01,
            x: 0x02,
            y: 0x03,
            stack_pointer: 0xFD,
            processor_status: 0x24,
            current_instruction: "Instruction { opcode: LDA }".to_string(),
            last_instruction_success: true,
            total_cpu_cycles: 123,
            instruction_count: 45,
        };

        let mut buffer = vec![0; APP_WIDTH * HEIGHT];
        draw_app_frame(&mut buffer, &pixels, &snapshot, ColorToggles::default());

        assert_eq!(APP_WIDTH, WIDTH + DEBUG_PANEL_WIDTH);
        assert_eq!(buffer[0], 0xAA5511);
        assert_eq!(buffer[WIDTH], 0x2A2F3A);
        let text_row_start = 10 * APP_WIDTH;
        assert!(
            buffer[text_row_start + WIDTH + 10..text_row_start + APP_WIDTH]
                .iter()
                .any(|pixel| *pixel != 0x111318)
        );
    }

    #[test]
    fn color_toggles_change_frame_color() {
        assert_eq!(
            ColorToggles::default().frame_color(),
            Rgb {
                r: 255,
                g: 255,
                b: 0
            }
        );
        assert_eq!(
            ColorToggles {
                orange: true,
                indigo: false
            }
            .frame_color(),
            Rgb {
                r: 255,
                g: 108,
                b: 0
            }
        );
        assert_eq!(
            ColorToggles {
                orange: false,
                indigo: true
            }
            .frame_color(),
            Rgb {
                r: 70,
                g: 120,
                b: 255
            }
        );
        assert_eq!(
            ColorToggles {
                orange: true,
                indigo: true
            }
            .frame_color(),
            Rgb {
                r: 220,
                g: 70,
                b: 255
            }
        );
    }
}
