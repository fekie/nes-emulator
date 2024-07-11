use nes6502::Cpu;

use crate::cpu::CpuContainer;
use std::{cell::RefCell, rc::Rc};

#[allow(clippy::upper_case_acronyms)]
pub struct Ppu {
    pub registers: [u8; 8],
    pub cpu: Option<Rc<RefCell<CpuContainer>>>,
    pub initialized: bool,
}

impl Ppu {
    /// Creates the PPU but does not initialize it. Please run [`Initialize`] to
    /// initialize the PPU.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            registers: [0; 8],
            cpu: None,
            initialized: false,
        }
    }

    /// Initialize the PPU.
    pub fn initialize(&mut self, cpu: Rc<RefCell<CpuContainer>>) {
        self.cpu = Some(cpu);
        self.initialized = true;
    }

    /// Returns the state of initialization.
    pub fn initialized(&self) -> bool {
        self.initialized
    }

    pub fn clock(&mut self) {
        // do nothing right now
    }
}
