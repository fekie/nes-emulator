use crate::cartridge::{self, Cartridge};
use crate::ppu::PPU;
use crate::CPU;
use std::cell::RefCell;
use std::default;
use std::rc::Rc;

const PPU_CLOCK_DIVISOR: u8 = 4;

#[derive(Copy, Clone, Default)]
pub enum Request {
    Active,
    #[default]
    Inactive,
}

/// Wraps over internally mutatable interrupt states.
#[derive(Default)]
pub struct Interrupts {
    interrupt: Rc<RefCell<Request>>,
    non_maskable_interrupt: Rc<RefCell<Request>>,
}

impl Interrupts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn interrupt_state(&self) -> Request {
        *self.interrupt.borrow()
    }

    pub fn set_interrupt_state(&self, new_state: Request) {
        *self.interrupt.borrow_mut() = new_state;
    }

    pub fn non_maskable_interrupt_state(&self) -> Request {
        *self.non_maskable_interrupt.borrow()
    }

    pub fn set_non_maskable_interrupt_state(&self, new_state: Request) {
        *self.non_maskable_interrupt.borrow_mut() = new_state;
    }
}

pub struct Bus {
    cpu: CPU,
    ppu: PPU,
    // todo: add APU later
    // Cartridge can sometimes interact with the cpu and apu
    // and it needs to be clocked just like
    cartridge: Cartridge,
    interrupts: Interrupts,
    /// A counter for our current cycles that wraps around. We mostly need it
    /// to be able to keep the PPU in sync as it needs to tick on every
    /// 4th machine cycle.
    current_machine_cycles: u8,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        let cpu = CPU::new();
        let ppu = PPU::new();
        let interrupts = Interrupts::new();
        let current_machine_cycles = 0;

        Self {
            cpu,
            ppu,
            cartridge,
            interrupts,
            current_machine_cycles,
        }
    }
}

impl Bus {
    /// Runs one cycle of the cpu and returns how many cpu cycles it took.
    /// CPU cycles can be converted to clock cycles by multiplying by 12.
    pub fn clock_cpu(&mut self) -> u8 {
        self.cpu.cycle(&mut self.ppu)
    }

    /// Clocks all the devices on the bus, such as the PPU, APU, and Cartridge
    pub fn clock_bus(&mut self, additional_machine_cycles: u8) {
        for _ in 0..additional_machine_cycles {
            // clock ppu every 4 cycles
            if self.current_machine_cycles % PPU_CLOCK_DIVISOR == 0 {
                self.ppu.tick(&self.cartridge);
            }

            // clock cartridge every tick
            self.cartridge.clock(&self.interrupts);

            self.current_machine_cycles = self.current_machine_cycles.wrapping_add(1);
        }
    }
}
