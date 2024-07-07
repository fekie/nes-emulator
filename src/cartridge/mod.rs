use crate::{
    bus::{BusPointer, Interrupts},
    ines::Ines,
    ClockableMapper,
};

const KB: usize = 1024;

pub struct Cartridge {
    mapper: Box<dyn ClockableMapper>,
    bus: BusPointer,
}

impl Cartridge {
    /// Creates a new cartridge from a ROM in the INES format.
    pub fn new(rom: Ines, bus: BusPointer) -> Self {
        let mapper = select_mapper(rom);

        Self { mapper, bus }
    }

    pub fn initialize(&mut self) {
        self.mapper.initialize();
    }

    pub fn initialized(&self) -> bool {
        self.mapper.initialized()
    }

    pub fn read(&self, address: u16) -> u8 {
        self.mapper.read(address)
    }

    pub fn write(&mut self, address: u16, byte: u8) {
        self.mapper.write(address, byte)
    }

    pub fn clock(&mut self) {
        self.mapper
            .clock(&self.bus.borrow().interrupts.0.as_ref().unwrap().borrow());
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
struct NROM {
    program_rom: [u8; KB * 32],
    character_rom: [u8; KB * 8],
    initialized: bool,
}

impl NROM {
    pub fn new(mut ines: Ines) -> Self {
        let mut program_rom = [0; KB * 32];
        let mut character_rom = [0; KB * 8];
        character_rom.copy_from_slice(&ines.character_rom);
        let is_mirrored = program_rom.len() != ines.program_rom.len();

        Self {
            program_rom: match is_mirrored {
                true => {
                    let copy = ines.program_rom.clone();
                    ines.program_rom.extend(copy);
                    program_rom.clone_from_slice(&ines.program_rom);
                    program_rom
                }
                false => {
                    program_rom.clone_from_slice(&ines.program_rom);
                    program_rom
                }
            },
            character_rom,
            initialized: false,
        }
    }
}

impl ClockableMapper for NROM {
    fn read(&self, address: u16) -> u8 {
        self.program_rom[address as usize - 0x8000]
    }

    fn write(&mut self, address: u16, byte: u8) {
        self.program_rom[address as usize - 0x8000] = byte;
    }

    fn clock(&mut self, _interrupts: &Interrupts) {
        // NROM doesnt interact so we do nothing
    }

    fn initialize(&mut self) {
        self.initialized = true;
    }

    fn initialized(&self) -> bool {
        self.initialized
    }
}

fn select_mapper(ines: Ines) -> Box<dyn ClockableMapper> {
    match ines.header.mapper_number {
        0 => Box::new(NROM::new(ines)),
        _ => panic!("Mapper not implemented"),
    }
}
