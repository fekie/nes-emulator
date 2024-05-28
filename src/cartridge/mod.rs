use crate::{
    bus::Interrupts,
    ines::{Header, Ines},
    ClockableMapper, Mapper,
};
const KB: usize = 1024;

pub struct Cartridge(Box<dyn ClockableMapper>);

impl From<Ines> for Cartridge {
    fn from(value: Ines) -> Self {
        let mapper = select_mapper(value.header.mapper_number);

        Self(mapper)
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

#[derive(Debug)]
struct NROM {
    program_rom: [u8; KB * 32],
    program_ram: [u8; KB * 8],
    character_rom: [u8; KB * 8],
}

impl NROM {
    pub fn new() -> Self {
        Self {
            program_rom: [0; KB * 32],
            program_ram: [0; KB * 8],
            character_rom: [0; KB * 8],
        }
    }
}

impl ClockableMapper for NROM {
    fn read(&self, address: u16) -> u8 {
        todo!();
        self.program_rom[address as usize - 0x8000]
    }

    fn write(&mut self, address: u16, byte: u8) {
        todo!();
        self.program_rom[address as usize - 0x8000] = byte;
    }

    fn clock(&mut self, interrupts: &Interrupts) {
        // NROM doesnt interact so we do nothing
    }
}

fn select_mapper(mapper_number: u8) -> Box<dyn ClockableMapper> {
    match mapper_number {
        0 => Box::new(NROM::new()),
        _ => panic!("Mapper not implemented"),
    }
}
