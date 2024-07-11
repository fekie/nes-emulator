use crate::cpu::{self, CpuContainer};
use crate::ppu;
use crate::{ines::Ines, ppu::Ppu};
use std::cell::RefCell;
use std::rc::Rc;

const KB: usize = 1024;

pub trait ClockableMapper {
    type Cpu;
    type Ppu;

    fn read(&self, address: u16) -> u8;

    fn write(&mut self, address: u16, byte: u8);

    fn clock(&mut self);

    /// Initialize the APU.
    fn initialize(&mut self, cpu: Self::Cpu, ppu: Self::Ppu);

    /// Returns the state of initialization.
    fn initialized(&self) -> bool;
}

pub struct Cartridge {
    mapper: Box<dyn ClockableMapper<Cpu = Rc<RefCell<CpuContainer>>, Ppu = Rc<RefCell<Ppu>>>>,
}

impl Cartridge {
    /// Creates a new cartridge from a ROM in the INES format.
    pub fn new(rom: Ines) -> Self {
        let mapper = select_mapper(rom);

        Self { mapper }
    }

    pub fn initialize(&mut self, cpu: Rc<RefCell<CpuContainer>>, ppu: Rc<RefCell<Ppu>>) {
        self.mapper.initialize(cpu, ppu);
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
        self.mapper.clock();
    }
}

struct Nrom {
    program_rom: [u8; KB * 32],
    character_rom: [u8; KB * 8],
    cpu: Option<Rc<RefCell<CpuContainer>>>,
    ppu: Option<Rc<RefCell<Ppu>>>,
    initialized: bool,
}

impl Nrom {
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
            cpu: None,
            ppu: None,
            initialized: false,
        }
    }
}

impl ClockableMapper for Nrom {
    type Cpu = Rc<RefCell<CpuContainer>>;
    type Ppu = Rc<RefCell<Ppu>>;

    fn read(&self, address: u16) -> u8 {
        self.program_rom[address as usize - 0x8000]
    }

    fn write(&mut self, address: u16, byte: u8) {
        self.program_rom[address as usize - 0x8000] = byte;
    }

    fn clock(&mut self) {
        // NROM doesnt interact so we do nothing
    }

    fn initialize(&mut self, cpu: Self::Cpu, ppu: Self::Ppu) {
        self.cpu = Some(cpu);
        self.ppu = Some(ppu);
        self.initialized = true;
    }

    fn initialized(&self) -> bool {
        self.initialized
    }
}

fn select_mapper(
    ines: Ines,
) -> Box<dyn ClockableMapper<Cpu = Rc<RefCell<CpuContainer>>, Ppu = Rc<RefCell<Ppu>>>> {
    match ines.header.mapper_number {
        0 => Box::new(Nrom::new(ines)),
        _ => panic!("Mapper not implemented"),
    }
}
