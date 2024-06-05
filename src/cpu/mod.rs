use instructions::{FullOpcode, Instruction};

use crate::bus::Bus;
use crate::Mapper;
use processor_status::ProcessorStatus;

const STACK_POINTER_STARTING_VALUE: u8 = 0xFD;
pub const NMI_VECTOR_ADDRESS: u16 = 0xFFFA;
pub const RESET_VECTOR_ADDRESS: u16 = 0xFFFC;
pub const IRQ_BRK_VECTOR_ADDRESS: u16 = 0xFFFE;

mod instructions;
mod processor_status;

pub struct CPU {
    accumulator_register: u8,
    x_register: u8,
    y_register: u8,
    stack_pointer: u8,
    program_counter: u16,
    registers: [u8; 6],
    processor_status: ProcessorStatus,
    memory_mapper: CpuMemoryMapper,
    initialized: bool,
}

impl CPU {
    /// Creates a new CPU but does not initialize it as it needs to be connected
    /// to the bus to initialize. You can initialize it with [`Self::initialize`].
    pub fn new() -> Self {
        Self {
            accumulator_register: 0,
            x_register: 0,
            y_register: 0,
            stack_pointer: STACK_POINTER_STARTING_VALUE,
            program_counter: 0,
            registers: [0; 6],
            processor_status: ProcessorStatus::new(),
            memory_mapper: CpuMemoryMapper::new(),
            initialized: false,
        }
    }

    /// Initializes the CPU to a state ready to run instructions.
    pub fn initialize(&mut self, bus: &Bus) {
        self.processor_status.clear_carry_flag();
        self.processor_status.clear_zero_flag();
        self.processor_status.set_interrupt_disable_flag();
        self.processor_status.clear_decimal_flag();
        self.processor_status.clear_overflow_flag();
        self.processor_status.clear_negative_flag();
        self.processor_status.clear_break_flag();

        self.stack_pointer = STACK_POINTER_STARTING_VALUE;

        self.program_counter = {
            let low_byte = self.memory_mapper.read(bus, RESET_VECTOR_ADDRESS) as u16;
            let high_byte = self.memory_mapper.read(bus, RESET_VECTOR_ADDRESS + 1) as u16;
            (high_byte << 8) + low_byte
        };

        self.initialized = true;
    }

    pub fn initialized(&self) -> bool {
        self.initialized
    }

    /// Runs a full instruction cycle. Returns the amount of
    /// machine cycles taken.
    pub fn cycle(&mut self, bus: &Bus) -> u8 {
        // fetch + decode
        let instruction = self.fetch(bus);
        dbg!(instruction);
        // execute
        let machine_cycles_taken = todo!();
        machine_cycles_taken
    }

    /// Fetches the next instruction and updates the program counter.
    pub fn fetch(&mut self, bus: &Bus) -> Instruction {
        let full_opcode = FullOpcode::new(self.memory_mapper.read(bus, self.program_counter));

        let bytes_required = full_opcode.addressing_mode.bytes_required();

        // Low byte comes first as words are in little-endian
        let (low_byte, high_byte) = match bytes_required {
            1 => (None, None),
            2 => (
                Some(self.memory_mapper.read(bus, self.program_counter + 1)),
                None,
            ),
            3 => (
                Some(self.memory_mapper.read(bus, self.program_counter + 1)),
                Some(self.memory_mapper.read(bus, self.program_counter + 2)),
            ),
            _ => unreachable!(),
        };

        // Decide how much we need to increment the PC
        self.program_counter += bytes_required;

        Instruction {
            opcode: full_opcode.opcode,
            addressing_mode: full_opcode.addressing_mode,
            low_byte,
            high_byte,
        }
    }

    /// Executes the instruction and returns the amount of machine cycles that it took.
    pub fn execute(&mut self, instruction: Instruction) -> u8 {
        todo!()
    }
}

// We use 2KB of work ram.
#[derive(Debug)]
pub struct WorkRAM([u8; 0x800]);

/// Memory Map:
///
/// | Address range |  Size  |                                  Device                                  |   |   |
/// |:-------------:|:------:|:------------------------------------------------------------------------:|---|---|
/// | $0000–$07FF   | $0800  | 2 KB internal RAM                                                        |   |   |
/// | $0800–$0FFF   | $0800  | Mirror  of $0000–$07FF                                                   |   |   |
/// | $1000–$17FF   | $0800  | Mirror  of $0000–$07FF                                                   |   |   |
/// | $1800–$1FFF   | $0800  | Mirror  of $0000–$07FF                                                   |   |   |
/// | $2000–$2007   | $0008  | NES PPU registers                                                        |   |   |
/// | $2008–$3FFF   | $1FF8  | Mirrors of $2000–$2007 (repeats every 8 bytes)                           |   |   |
/// | $4000–$4017   | $0018  | NES APU and I/O registers                                                |   |   |
/// | $4018–$401F   | $0008  | APU and I/O functionality that is normally disabled. See CPU Test Mode.  |   |   |
/// | $4020–$FFFF   | $BFE0  | Unmapped. Available for cartridge use.                                   |   |   |
/// | *$6000-$7FFF  | $2000  | Usually cartridge RAM, when present.                                     |   |   |
/// | *$8000-$FFFF  | $8000  | Usually cartridge ROM and mapper registers.                              |   |   |
#[derive(Debug)]
pub struct CpuMemoryMapper {
    work_ram: WorkRAM,
}

impl CpuMemoryMapper {
    pub fn new() -> Self {
        Self {
            work_ram: WorkRAM([0; 2048]),
        }
    }
}

impl Mapper for CpuMemoryMapper {
    fn read(&self, bus: &Bus, address: u16) -> u8 {
        match address {
            // Handle the work RAM and the mirrors.
            0x0000..=0x1FFF => self.work_ram.0[address as usize % 0x0800],
            // Handle PPU registers and the mirrors.
            0x2000..=0x3FFF => bus.ppu.borrow().registers[((address - 0x2000) % 8) as usize],
            // Saved for APU
            0x4000..=0x4017 => unimplemented!(),
            // Disabled
            0x4018..=0x401F => unimplemented!(),
            // Route to cartridge mapper
            0x4020..=0xFFFF => bus.cartridge.borrow().read(address),
        }
    }

    fn write(&mut self, bus: &Bus, address: u16, byte: u8) {
        match address {
            // Handle the work RAM and the mirrors.
            0x0000..=0x1FFF => self.work_ram.0[address as usize % 0x0800] = byte,
            // Handle PPU registers and the mirrors.
            0x2000..=0x3FFF => {
                bus.ppu.borrow_mut().registers[((address - 0x2000) % 8) as usize] = byte
            }
            // Saved for APU
            0x4000..=0x4017 => unimplemented!(),
            // Disabled
            0x4018..=0x401F => unimplemented!(),
            // Route to cartridge mapper
            0x4020..=0xFFFF => bus.cartridge.borrow_mut().write(address, byte),
        }
    }
}
