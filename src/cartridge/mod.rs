use crate::{ines::Ines, Mapper};

pub struct Cartridge(Box<dyn Mapper>);

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
}

struct NROM {
    rom: [u8; 0x8000],
}

impl Mapper for NROM {
    fn read(&self, address: u16) -> u8 {
        self.rom[address as usize - 0x8000]
    }

    fn write(&mut self, address: u16, byte: u8) {
        self.rom[address as usize - 0x8000] = byte;
    }
}
