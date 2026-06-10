use crate::cartridge::Cartridge;
use crate::cpu::{CpuContainer, CpuDebugSnapshot};
use crate::debug::StartupInstructionTrace;
use crate::display::{Pixels, HEIGHT};
use crate::graphical_debug::{draw_app_frame, ColorToggles, APP_WIDTH};
use crate::ppu::{self, Ppu, PpuDebugSnapshot};
use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use std::time::Instant;

const MASTER_CLOCK_HZ: f64 = 21_477_272.0;
const CLOCK_DIVISOR: u64 = 12;
const CPU_HZ: f64 = MASTER_CLOCK_HZ / CLOCK_DIVISOR as f64;
const FRAME_INTERVAL_SECS: f64 = ppu::CPU_CYCLES_PER_FRAME / CPU_HZ;
const PPU_CLOCK_DIVISOR: u8 = 4;

#[derive(Default, Debug)]
enum Keycode {
    #[default]
    Placeholder,
    ToggleOrange,
    ToggleIndigo,
}

#[derive(Debug)]
struct FrameFinishedSignal {
    current_keycode: Keycode,
    delay_debt_s: f64,
}

struct SharedDebug {
    cpu: Arc<Mutex<CpuDebugSnapshot>>,
    ppu: Arc<Mutex<PpuDebugSnapshot>>,
}

impl SharedDebug {
    fn new() -> Self {
        Self {
            cpu: Arc::new(Mutex::new(CpuDebugSnapshot::default())),
            ppu: Arc::new(Mutex::new(PpuDebugSnapshot::default())),
        }
    }
}

pub fn run<F>(create_emulator: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnOnce() -> Emulator + Send + 'static,
{
    let pixels = Arc::new(Pixels::new());
    let shared_debug = SharedDebug::new();
    let mut buffer = vec![0; APP_WIDTH * HEIGHT];
    let (tx, rx) = crossbeam_channel::unbounded::<FrameFinishedSignal>();

    spawn_emulator(create_emulator, rx, pixels.clone(), &shared_debug);
    run_render_loop(pixels, shared_debug, &mut buffer, tx)?;

    Ok(())
}

fn spawn_emulator<F>(
    create_emulator: F,
    rx: crossbeam_channel::Receiver<FrameFinishedSignal>,
    pixels: Arc<Pixels>,
    shared_debug: &SharedDebug,
) where
    F: FnOnce() -> Emulator + Send + 'static,
{
    let cpu_debug = shared_debug.cpu.clone();
    let ppu_debug = shared_debug.ppu.clone();

    spawn(move || {
        let mut emulator = create_emulator();
        let mut runner = EmulatorRunner::new();

        while let Ok(frame_finished_signal) = rx.recv() {
            runner.run_frame(
                &mut emulator,
                &pixels,
                &cpu_debug,
                &ppu_debug,
                frame_finished_signal,
            );
        }
    });
}

fn run_render_loop(
    pixels: Arc<Pixels>,
    shared_debug: SharedDebug,
    buffer: &mut [u32],
    tx: crossbeam_channel::Sender<FrameFinishedSignal>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new(
        "Test - ESC to exit",
        APP_WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X4,
            ..WindowOptions::default()
        },
    )?;

    window.set_target_fps(60);

    let mut previous_frame_stamp = Instant::now();
    let mut color_toggles = ColorToggles::default();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let current_keycode = process_input(&window, &mut color_toggles);

        draw_app_frame(
            buffer,
            &pixels,
            &shared_debug.cpu.lock().unwrap(),
            &shared_debug.ppu.lock().unwrap(),
            color_toggles,
        );
        window.update_with_buffer(buffer, APP_WIDTH, HEIGHT)?;

        let delay_debt_s = previous_frame_stamp.elapsed().as_secs_f64() - FRAME_INTERVAL_SECS;
        tx.send(FrameFinishedSignal {
            current_keycode,
            delay_debt_s,
        })?;

        previous_frame_stamp = Instant::now();
    }

    Ok(())
}

fn process_input(window: &Window, color_toggles: &mut ColorToggles) -> Keycode {
    let mut current_keycode = Keycode::Placeholder;

    if window.is_key_pressed(Key::O, KeyRepeat::No) {
        color_toggles.orange = !color_toggles.orange;
        current_keycode = Keycode::ToggleOrange;
    }

    if window.is_key_pressed(Key::I, KeyRepeat::No) {
        color_toggles.indigo = !color_toggles.indigo;
        current_keycode = Keycode::ToggleIndigo;
    }

    current_keycode
}

pub struct Emulator {
    cpu: Rc<RefCell<CpuContainer>>,
    ppu: Rc<RefCell<Ppu>>,
    cartridge: Rc<RefCell<Cartridge>>,
}

impl Emulator {
    pub fn from_initialized(
        cpu: Rc<RefCell<CpuContainer>>,
        ppu: Rc<RefCell<Ppu>>,
        cartridge: Rc<RefCell<Cartridge>>,
    ) -> Self {
        Self {
            cpu,
            ppu,
            cartridge,
        }
    }
}

struct EmulatorRunner {
    cpu_cycle_debt: i64,
    current_machine_cycles: u8,
    cpu_snapshot: CpuDebugSnapshot,
    startup_instruction_trace: StartupInstructionTrace,
}

impl EmulatorRunner {
    fn new() -> Self {
        Self {
            cpu_cycle_debt: 0,
            current_machine_cycles: 0,
            cpu_snapshot: CpuDebugSnapshot::default(),
            startup_instruction_trace: StartupInstructionTrace::new(
                "startup_instruction_trace.txt",
            ),
        }
    }

    fn run_frame(
        &mut self,
        emulator: &mut Emulator,
        pixels: &Pixels,
        cpu_debug: &Mutex<CpuDebugSnapshot>,
        ppu_debug: &Mutex<PpuDebugSnapshot>,
        frame_finished_signal: FrameFinishedSignal,
    ) {
        self.save_completed_startup_trace();
        handle_keycode(frame_finished_signal.current_keycode);

        let mut available_cpu_cycles = ((FRAME_INTERVAL_SECS + frame_finished_signal.delay_debt_s)
            * CPU_HZ) as i64
            + self.cpu_cycle_debt;

        loop {
            let cpu_cycles_taken = emulator
                .cpu
                .borrow_mut()
                .cycle_debug(&mut self.cpu_snapshot);

            if cpu_cycles_taken == 0 {
                publish_debug_snapshots(cpu_debug, ppu_debug, &self.cpu_snapshot, &emulator.ppu);
                break;
            }

            available_cpu_cycles -= cpu_cycles_taken as i64;
            self.clock_bus(emulator, pixels, cpu_cycles_taken);

            let ppu_snapshot = emulator.ppu.borrow().debug_snapshot();
            self.record_startup_trace(&ppu_snapshot);

            if available_cpu_cycles <= 0 {
                self.cpu_cycle_debt = available_cpu_cycles;
                *cpu_debug.lock().unwrap() = self.cpu_snapshot.clone();
                *ppu_debug.lock().unwrap() = ppu_snapshot;
                break;
            }
        }
    }

    fn clock_bus(&mut self, emulator: &mut Emulator, pixels: &Pixels, cpu_cycles_taken: u8) {
        let machine_cycles_taken = cpu_cycles_taken * CLOCK_DIVISOR as u8;

        for _ in 0..machine_cycles_taken {
            if self.current_machine_cycles % PPU_CLOCK_DIVISOR == 0 {
                emulator.ppu.borrow_mut().clock(pixels);
            }

            emulator.cartridge.borrow_mut().clock();
            self.current_machine_cycles = self.current_machine_cycles.wrapping_add(1);
        }
    }

    fn save_completed_startup_trace(&mut self) {
        if let Err(err) = self.startup_instruction_trace.save_if_complete() {
            eprintln!(
                "Failed to save startup instruction trace to {}: {err}",
                self.startup_instruction_trace.path().display()
            );
        }
    }

    fn record_startup_trace(&mut self, ppu_snapshot: &PpuDebugSnapshot) {
        if let Err(err) = self
            .startup_instruction_trace
            .record(&self.cpu_snapshot, ppu_snapshot)
        {
            eprintln!(
                "Failed to save startup instruction trace to {}: {err}",
                self.startup_instruction_trace.path().display()
            );
        }
    }
}

fn publish_debug_snapshots(
    cpu_debug: &Mutex<CpuDebugSnapshot>,
    ppu_debug: &Mutex<PpuDebugSnapshot>,
    cpu_snapshot: &CpuDebugSnapshot,
    ppu: &RefCell<Ppu>,
) {
    *cpu_debug.lock().unwrap() = cpu_snapshot.clone();
    *ppu_debug.lock().unwrap() = ppu.borrow().debug_snapshot();
}

fn handle_keycode(keycode: Keycode) {
    match keycode {
        Keycode::Placeholder => {}
        Keycode::ToggleOrange | Keycode::ToggleIndigo => {}
    }
}
