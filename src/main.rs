//! System info:
use apu::Apu;
use cartridge::Cartridge;
use clap::Parser;
use cpu::CpuContainer;
use debug::Tile;
use image::{DynamicImage, GrayImage, RgbImage};
use ines::Ines;
use lazy_static::lazy_static;
/// - System Type: NTSC
use minifb::{Key, Window, WindowOptions};
use ppu::Ppu;
use rgb::*;
use std::cell::RefCell;
use std::option;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::spawn;
use std::time::Instant;

const WIDTH: usize = 256;
const HEIGHT: usize = 240;
const SCREEN_WIDTH: usize = WIDTH * 4;
const SCREEN_HEIGHT: usize = HEIGHT * 4;

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

#[derive(Default, Debug)]
enum Keycode {
    #[default]
    Placeholder,
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
    let mut buffer = vec![0; WIDTH * HEIGHT];

    let thread_1_pixels = pixels.clone();

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

        while let Ok(frame_finished_signal) = rx.recv() {
            // Do cycles for (FRAME_INTERVAL_SECS + delay_debt_s) * CPU_HZ
            let mut available_cpu_cycles =
                ((FRAME_INTERVAL_SECS + frame_finished_signal.delay_debt_s) * CPU_HZ as f64) as i64
                    + cpu_cycle_debt;

            // Each time we do a cpu instruction, find out how many clock cycles it
            // took, multiply by 12, and then run that many master clock cycles on the bus.
            loop {
                // run a cpu full instruction cycle and see how many cpu cycles were taken
                let cpu_cycles_taken = cpu.borrow_mut().cycle();

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

    let mut v = 0;

    let mut previous_frame_stamp = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        /* for i in buffer.iter_mut() {
            *i = v; // write something more funny here!
            v += 1;
            v += v.ilog(4);
        } */

        for i in 0..WIDTH * HEIGHT {
            let y = i / WIDTH;
            let x = i % WIDTH;
            pixels.write(
                x,
                y,
                Rgb {
                    r: 255,
                    g: 255,
                    b: 0,
                },
            );
        }

        pixels.copy_to_buffer(&mut buffer);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        let delay_debt_s = previous_frame_stamp.elapsed().as_secs_f64() - FRAME_INTERVAL_SECS;

        tx.send(FrameFinishedSignal {
            current_keycode: Keycode::Placeholder,
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
