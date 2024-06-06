use crate::{bus::Bus, cartridge::Cartridge};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct PPU {
    pub registers: [u8; 8],
    initialized: bool,
}

impl PPU {
    /// Creates the PPU but does not initialize it. Please run [`Initialize`] to
    /// initialize the PPU.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            registers: [0; 8],
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

    pub fn tick(&mut self, cartridge: &Cartridge) {
        // do nothing right now
    }
}
