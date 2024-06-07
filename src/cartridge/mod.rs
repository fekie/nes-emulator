use crate::{
    bus::{Bus, Interrupts},
    ines::Ines,
    ClockableMapper,
};

const KB: usize = 1024;

pub struct Cartridge(Box<dyn ClockableMapper>);

impl From<Ines> for Cartridge {
    fn from(ines: Ines) -> Self {
        dbg!(&ines.header);
        let mapper = select_mapper(ines);

        Self(mapper)
    }
}

impl Cartridge {
    /// Initialize the APU.
    pub fn initialize(&mut self) {
        self.0.initialize();
    }

    pub fn initialized(&self) -> bool {
        self.0.initialized()
    }

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
