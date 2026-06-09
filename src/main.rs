//! System info:
use apu::Apu;
use cartridge::Cartridge;
use clap::Parser;
use cpu::{CpuContainer, CpuDebugSnapshot};
use debug::{StartupInstructionTrace, Tile};
use graphical_debug::{draw_app_frame, ColorToggles, APP_WIDTH};
use ines::Ines;
/// - System Type: NTSC
use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
use ppu::{Ppu, PpuDebugSnapshot};
use rgb::Rgb;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::spawn;
use std::time::Instant;

const WIDTH: usize = ppu::VISIBLE_DOTS;
const HEIGHT: usize = ppu::VISIBLE_SCANLINES;

const MASTER_CLOCK_HZ: f64 = 21_477_272.0;
const CLOCK_DIVISOR: u64 = 12;
const CPU_HZ: f64 = MASTER_CLOCK_HZ / CLOCK_DIVISOR as f64;
const FRAME_INTERVAL_SECS: f64 = ppu::CPU_CYCLES_PER_FRAME / CPU_HZ;

const PPU_CLOCK_DIVISOR: u8 = 4;

mod apu;
mod cartridge;
mod cpu;
mod debug;
mod graphical_debug;
mod ines;
mod ppu;

pub struct MapperType {}

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
    //#[arg(short, long)]
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

    pub fn copy_to_app_buffer(&self, buffer: &mut [u32], app_width: usize) {
        let pixels = self.0.lock().unwrap();
        for y in 0..HEIGHT {
            let source_row = y * WIDTH;
            let target_row = y * app_width;
            buffer[target_row..target_row + WIDTH]
                .copy_from_slice(&pixels[source_row..source_row + WIDTH]);
        }
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
    let ppu_debug = Arc::new(Mutex::new(PpuDebugSnapshot::default()));
    let mut buffer = vec![0; APP_WIDTH * HEIGHT];

    let thread_1_pixels = pixels.clone();
    let thread_1_cpu_debug = cpu_debug.clone();
    let thread_1_ppu_debug = ppu_debug.clone();

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
                ((FRAME_INTERVAL_SECS + frame_finished_signal.delay_debt_s) * CPU_HZ) as i64
                    + cpu_cycle_debt;

            // Each time we do a cpu instruction, find out how many clock cycles it
            // took, multiply by 12, and then run that many master clock cycles on the bus.
            loop {
                // run a cpu full instruction cycle and see how many cpu cycles were taken
                let cpu_cycles_taken = cpu.borrow_mut().cycle_debug(&mut debug_snapshot);

                if cpu_cycles_taken == 0 {
                    *thread_1_cpu_debug.lock().unwrap() = debug_snapshot.clone();
                    *thread_1_ppu_debug.lock().unwrap() = ppu.borrow().debug_snapshot();
                    break;
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

                let ppu_snapshot = ppu.borrow().debug_snapshot();
                if let Err(err) = startup_instruction_trace.record(&debug_snapshot, &ppu_snapshot) {
                    eprintln!(
                        "Failed to save startup instruction trace to {}: {err}",
                        startup_instruction_trace.path().display()
                    );
                }

                // If we're out of cpu cycles, record how much we went over and then
                // stop running cycles until the next request
                if available_cpu_cycles <= 0 {
                    cpu_cycle_debt = available_cpu_cycles;
                    *thread_1_cpu_debug.lock().unwrap() = debug_snapshot.clone();
                    *thread_1_ppu_debug.lock().unwrap() = ppu_snapshot;
                    break;
                }
            }
        }
    });

    // Rendering

    let mut window = Window::new(
        "Test - ESC to exit",
        APP_WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X4,
            ..WindowOptions::default()
        },
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

        draw_app_frame(
            &mut buffer,
            &pixels,
            &cpu_debug.lock().unwrap(),
            &ppu_debug.lock().unwrap(),
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

fn print_pattern_tables(_args: &Args, rom: &Ines) {
    let pattern_bytes = &rom.character_rom[0..=0x1FFF];
    let tiles = pattern_bytes
        .chunks(16)
        .map(debug::deinterlace_tile_bytes)
        .collect::<Vec<Tile>>();

    let img = debug::stitch_tiles(&tiles);
    img.save("pattern_table.png").unwrap();
    println!("Saved pattern table to pattern_table.png");
}
