//! System info:
use lazy_static::lazy_static;
/// - System Type: NTSC
use minifb::{Key, Window, WindowOptions};
use spin_sleep::{SpinSleeper, SpinStrategy};
use std::sync::mpsc::channel;
use std::thread::{self, spawn};
use std::time::{Duration, Instant};

const WIDTH: usize = 256;
const HEIGHT: usize = 240;
const SCREEN_WIDTH: usize = WIDTH * 4;
const SCREEN_HEIGHT: usize = HEIGHT * 4;

// Speeds taken from https://www.nesdev.org/wiki/CPU
const CORE_CLOCK_HZ: u64 = 21_441_960;
const CLOCK_DIVISOR: u64 = 12;
const CPU_HZ: u64 = CORE_CLOCK_HZ / CLOCK_DIVISOR;
const FRAME_INTERVAL_SECS: f64 = 1.0 / TARGET_FPS as f64;
const MACHINE_CYCLES_PER_FRAME: u64 = (CPU_HZ as f64 * FRAME_INTERVAL_SECS) as u64;

const TARGET_FPS: usize = 60;

#[derive(Default, Debug)]
enum Keycode {
    #[default]
    Placeholder,
}

#[derive(Debug)]
struct FrameFinishedSignal {
    /// The key that was pressed down just after the newly created frame.
    current_keycode: Keycode,
}

fn main() {
    let (tx, rx) = channel::<FrameFinishedSignal>();

    // After every frame, we process the appropriate amount of clock cycles.
    let _ = spawn(move || {
        let mut cycle_debt = 0;
        let mut last_instruction: Option<u8> = None;

        //let mut frames = 0;
        //let mut frame_60_time = Instant::now();

        while let Ok(_frame_finished_signal) = rx.recv() {
            /* frames += 1;
            if frames % 60 == 0 {
                dbg!(frame_60_time.elapsed());
                frame_60_time = Instant::now();
            } */
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
    window.set_target_fps(TARGET_FPS);

    let mut v = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = v; // write something more funny here!
            v += 1;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        tx.send(FrameFinishedSignal {
            current_keycode: Keycode::Placeholder,
        })
        .unwrap()
    }
}
