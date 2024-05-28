//! System info:
use cartridge::Cartridge;
use clap::Parser;
use cpu::CPU;
use ines::Ines;
/// - System Type: NTSC
use minifb::{Key, Window, WindowOptions};
use ppu::PPU;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;
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

const TARGET_FPS: usize = 60;

pub mod cartridge;
pub mod cpu;
pub mod ines;
pub mod ppu;

pub trait Mapper {
    fn read(&self, address: u16) -> u8;

    fn write(&mut self, address: u16, byte: u8);
}

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
    #[arg(short, long)]
    rom: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let rom = Ines::parse(&std::fs::read(args.rom)?);

    let (tx, rx) = crossbeam_channel::unbounded::<FrameFinishedSignal>();

    // After every frame, we process the appropriate amount of clock cycles.
    let _ = spawn(move || {
        let cartridge = Rc::new(RefCell::new(rom.into()));

        let ppu = Rc::new(RefCell::new(PPU::new()));
        let mut cpu = CPU::new(cartridge, ppu);

        // If we execute an instruction and it takes more cycles than we have available,
        // then we store the amount here (as a negative number) that we need to take off.
        let mut cycle_debt: i64 = 0;

        while let Ok(frame_finished_signal) = rx.recv() {
            // Do cycles for (FRAME_INTERVAL_SECS + delay_debt_s) * CPU_HZ
            let mut available_cycles = ((FRAME_INTERVAL_SECS + frame_finished_signal.delay_debt_s)
                * CPU_HZ as f64) as i64
                + cycle_debt;

            // Each tick cycle, do 3 PPU cycles, and
            loop {
                let cycles_taken = cpu.cycle();

                available_cycles -= cycles_taken as i64;

                if available_cycles <= 0 {
                    cycle_debt = available_cycles;
                    break;
                }
            }
        }
    });

    // Rendering
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

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
        for i in buffer.iter_mut() {
            *i = v; // write something more funny here!
            v += 1;
            v += v.ilog(4);
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
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
