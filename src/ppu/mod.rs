use crate::bus::BusPointer;
use crate::{bus::Bus, cartridge::Cartridge};
use std::{cell::RefCell, rc::Rc};

#[allow(clippy::upper_case_acronyms)]
pub struct Ppu {
    pub registers: [u8; 8],
    pub bus: BusPointer,
    pub initialized: bool,
}

impl Ppu {
    /// Creates the PPU but does not initialize it. Please run [`Initialize`] to
    /// initialize the PPU.
    #[allow(clippy::new_without_default)]
    pub fn new(bus: BusPointer) -> Self {
        Self {
            registers: [0; 8],
            bus,
            initialized: false,
        }
    }

    /// Initialize the PPU.
    pub fn initialize(&mut self) {
        self.initialized = true;
    }

    /// Returns the state of initialization.
    pub fn initialized(&self) -> bool {
        self.initialized
    }

    pub fn tick(&mut self) {
        // do nothing right now
    }
}
