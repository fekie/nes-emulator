use crate::{bus::Interrupts, ines::Ines, ClockableMapper, Mapper};

pub struct Cartridge(Box<dyn ClockableMapper>);

impl From<Ines> for Cartridge {
    fn from(value: Ines) -> Self {
        todo!()
    }
}

impl Cartridge {
    pub fn read(&self, address: u16) -> u8 {
        self.0.read(address)
    }

    pub fn write(&mut self, address: u16, byte: u8) {
        self.0.write(address, byte)
    }

    pub fn clock(&mut self, interrupts: &Interrupts) {
        self.0.clock(interrupts);
    }
}

struct NROM {
    rom: [u8; 0x8000],
}

impl ClockableMapper for NROM {
    fn read(&self, address: u16) -> u8 {
        self.rom[address as usize - 0x8000]
    }

    fn write(&mut self, address: u16, byte: u8) {
        self.rom[address as usize - 0x8000] = byte;
    }

    fn clock(&mut self, interrupts: &Interrupts) {
        // NROM doesnt interact so we do nothing
    }
}
