use crate::cartridge::{self, Cartridge};

#[derive(Debug, Default)]
pub struct PPU {
    pub registers: [u8; 8],
}

impl PPU {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self, cartridge: &Cartridge) {
        // do nothing right now
    }
}
