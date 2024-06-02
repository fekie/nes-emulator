use instructions::{AddressingMode, FullOpcode, Instruction};

use super::debug;
use crate::bus::Bus;
use crate::cartridge::Cartridge;
use crate::ppu::PPU;
use crate::Mapper;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::Bytes;

const STACK_POINTER_STARTING_VALUE: u8 = 0xFD;
const NMI_VECTOR_ADDRESS: u16 = 0xFFFA;
const RESET_VECTOR_ADDRESS: u16 = 0xFFFC;
const IRQ_BRK_VECTOR_ADDRESS: u16 = 0xFFFE;

mod instructions;
#[derive(Debug, Clone, Copy, Default)]
pub struct ProcessorStatus(u8);

impl ProcessorStatus {
    pub fn new() -> Self {
        ProcessorStatus(0)
    }
    pub fn carry_flag(&self) -> bool {
        self.0 & 0b0000_0001 != 0
    }

    pub fn set_carry_flag(&mut self) {
        let mask = 0b0000_0001;
        self.0 |= mask;
    }

    pub fn clear_carry_flag(&mut self) {
        let mask = 0b1111_1110;
        self.0 &= mask;
    }

    //

    pub fn zero_flag(&self) -> bool {
        self.0 & 0b0000_0010 != 0
    }

    pub fn set_zero_flag(&mut self) {
        let mask = 0b0000_0010;
        self.0 |= mask;
    }

    pub fn clear_zero_flag(&mut self) {
        let mask = 0b1111_1101;
        self.0 &= mask;
        self.0 |= mask;
    }

    //

    pub fn interrupt_disable_flag(&self) -> bool {
        self.0 & 0b0000_0100 != 0
    }

    pub fn set_interrupt_disable_flag(&mut self) {
        let mask = 0b0000_0100;
        self.0 |= mask;
    }

    pub fn clear_interrupt_disable_flag(&mut self) {
        let mask = 0b1111_1011;
        self.0 &= mask;
        self.0 |= mask;
    }

    //

    pub fn decimal_flag(&self) -> bool {
        self.0 & 0b0000_1000 != 0
    }

    pub fn set_decimal_flag(&mut self) {
        let mask = 0b0000_1000;
        self.0 |= mask;
    }

    pub fn clear_decimal_flag(&mut self) {
        let mask = 0b1111_0111;
        self.0 &= mask;
    }

    //

    pub fn break_flag(&self) -> bool {
        self.0 & 0b0001_0000 != 0
    }

    pub fn set_break_flag(&mut self) {
        let mask = 0b0001_0000;
        self.0 |= mask;
    }

    pub fn clear_break_flag(&mut self) {
        let mask = 0b1110_1111;
        self.0 &= mask;
    }

    //

    pub fn overflow_flag(&self) -> bool {
        self.0 & 0b0100_0000 != 0
    }

    pub fn set_overflow_flag(&mut self) {
        let mask = 0b0100_0000;
        self.0 |= mask;
    }

    pub fn clear_overflow_flag(&mut self) {
        let mask = 0b1011_1111;
        self.0 &= mask;
    }

    //

    pub fn negative_flag(&self) -> bool {
        self.0 & 0b1000_0000 != 0
    }

    pub fn set_negative_flag(&mut self) {
        let mask = 0b1000_0000;
        self.0 |= mask;
    }

    pub fn clear_negative_flag(&mut self) {
        let mask = 0b0111_1111;
        self.0 &= mask;
    }
}

pub struct CPU {
    accumulator_register: u8,
    x_register: u8,
    y_register: u8,
    stack_pointer: u8,
    program_counter: u16,
    registers: [u8; 6],
    processor_status: ProcessorStatus,
    memory_mapper: CpuMemoryMapper,
}

impl CPU {
    /// Creates a new CPU and initializes it to its startup state.
    pub fn new() -> Self {
        let mut processor_status = ProcessorStatus(0);
        processor_status.set_interrupt_disable_flag();

        Self {
            accumulator_register: 0,
            x_register: 0,
            y_register: 0,
            stack_pointer: STACK_POINTER_STARTING_VALUE,
            program_counter: NMI_VECTOR_ADDRESS,
            registers: [0; 6],
            processor_status,
            memory_mapper: CpuMemoryMapper::new(),
        }
    }

    /// Runs a full instruction cycle. Returns the amount of
    /// machine cycles taken.
    pub fn cycle(&mut self, bus: &Bus) -> u8 {
        // fetch + decode
        let instruction = self.fetch(bus);
        dbg!(instruction);
        //debug::hex_print_byte(byte);
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
            0x2000..=0x3FFF => bus.ppu.borrow_mut().registers[((address - 0x2000) % 8) as usize],
            // Saved for APU
            0x4000..=0x4017 => unimplemented!(),
            // Disabled
            0x4018..=0x401F => unimplemented!(),
            // Route to cartridge mapper
            0x4020..=0xFFFF => bus.cartridge.borrow().read(address),
        }
    }

    fn write(&mut self, bus: &Bus, address: u16, byte: u8) {
        todo!()
    }
}

pub fn decodeOPandADR(opcode: u8) -> FullOpcode {
    // just some sketches, can change later on
    match opcode {
        0x69 => FullOpcode {
            opcode: instructions::Opcode::ADC,
            addressing_mode: AddressingMode::Immediate,
        },
        0x65 => FullOpcode {
            opcode: instructions::Opcode::ADC,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x75 => FullOpcode {
            opcode: instructions::Opcode::ADC,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x6D => FullOpcode {
            opcode: instructions::Opcode::ADC,
            addressing_mode: AddressingMode::Absolute,
        },
        0x7D => FullOpcode {
            opcode: instructions::Opcode::ADC,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x79 => FullOpcode {
            opcode: instructions::Opcode::ADC,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0x61 => FullOpcode {
            opcode: instructions::Opcode::ADC,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0x71 => FullOpcode {
            opcode: instructions::Opcode::ADC,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0x29 => FullOpcode {
            opcode: instructions::Opcode::AND,
            addressing_mode: AddressingMode::Immediate,
        },
        0x25 => FullOpcode {
            opcode: instructions::Opcode::AND,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x35 => FullOpcode {
            opcode: instructions::Opcode::AND,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x2D => FullOpcode {
            opcode: instructions::Opcode::AND,
            addressing_mode: AddressingMode::Absolute,
        },
        0x3D => FullOpcode {
            opcode: instructions::Opcode::AND,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x39 => FullOpcode {
            opcode: instructions::Opcode::AND,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0x21 => FullOpcode {
            opcode: instructions::Opcode::AND,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0x31 => FullOpcode {
            opcode: instructions::Opcode::AND,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0x0A => FullOpcode {
            opcode: instructions::Opcode::ASL,
            addressing_mode: AddressingMode::Accumulator,
        },
        0x06 => FullOpcode {
            opcode: instructions::Opcode::ASL,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x16 => FullOpcode {
            opcode: instructions::Opcode::ASL,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x0E => FullOpcode {
            opcode: instructions::Opcode::ASL,
            addressing_mode: AddressingMode::Absolute,
        },
        0x1E => FullOpcode {
            opcode: instructions::Opcode::ASL,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x90 => FullOpcode {
            opcode: instructions::Opcode::BCC,
            addressing_mode: AddressingMode::Relative,
        },
        0xB0 => FullOpcode {
            opcode: instructions::Opcode::BCS,
            addressing_mode: AddressingMode::Relative,
        },
        0xF0 => FullOpcode {
            opcode: instructions::Opcode::BEQ,
            addressing_mode: AddressingMode::Relative,
        },
        0x24 => FullOpcode {
            opcode: instructions::Opcode::BIT,
            addressing_mode: AddressingMode::Absolute,
        },
        0x2C => FullOpcode {
            opcode: instructions::Opcode::BIT,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x30 => FullOpcode {
            opcode: instructions::Opcode::BMI,
            addressing_mode: AddressingMode::Relative,
        },
        0xD0 => FullOpcode {
            opcode: instructions::Opcode::BNE,
            addressing_mode: AddressingMode::Relative,
        },
        0x10 => FullOpcode {
            opcode: instructions::Opcode::BPL,
            addressing_mode: AddressingMode::Relative,
        },
        0x00 => FullOpcode {
            opcode: instructions::Opcode::BRK,
            addressing_mode: AddressingMode::Implied,
        },
        0x50 => FullOpcode {
            opcode: instructions::Opcode::BVC,
            addressing_mode: AddressingMode::Relative,
        },
        0x70 => FullOpcode {
            opcode: instructions::Opcode::BVS,
            addressing_mode: AddressingMode::Relative,
        },
        0x18 => FullOpcode {
            opcode: instructions::Opcode::CLC,
            addressing_mode: AddressingMode::Implied,
        },
        0xD8 => FullOpcode {
            opcode: instructions::Opcode::CLD,
            addressing_mode: AddressingMode::Implied,
        },
        0x58 => FullOpcode {
            opcode: instructions::Opcode::CLI,
            addressing_mode: AddressingMode::Implied,
        },
        0xB8 => FullOpcode {
            opcode: instructions::Opcode::CLV,
            addressing_mode: AddressingMode::Implied,
        },
        0xC9 => FullOpcode {
            opcode: instructions::Opcode::CMP,
            addressing_mode: AddressingMode::Immediate,
        },
        0xC5 => FullOpcode {
            opcode: instructions::Opcode::CMP,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xD5 => FullOpcode {
            opcode: instructions::Opcode::CMP,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xCD => FullOpcode {
            opcode: instructions::Opcode::CMP,
            addressing_mode: AddressingMode::Absolute,
        },
        0xDD => FullOpcode {
            opcode: instructions::Opcode::CMP,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0xD9 => FullOpcode {
            opcode: instructions::Opcode::CMP,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0xC1 => FullOpcode {
            opcode: instructions::Opcode::CMP,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0xD1 => FullOpcode {
            opcode: instructions::Opcode::CMP,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0xE0 => FullOpcode {
            opcode: instructions::Opcode::CPX,
            addressing_mode: AddressingMode::Immediate,
        },
        0xE4 => FullOpcode {
            opcode: instructions::Opcode::CPX,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xEC => FullOpcode {
            opcode: instructions::Opcode::CPX,
            addressing_mode: AddressingMode::Absolute,
        },
        0xC0 => FullOpcode {
            opcode: instructions::Opcode::CPY,
            addressing_mode: AddressingMode::Immediate,
        },
        0xC4 => FullOpcode {
            opcode: instructions::Opcode::CPY,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xCC => FullOpcode {
            opcode: instructions::Opcode::CPY,
            addressing_mode: AddressingMode::Absolute,
        },
        0xC6 => FullOpcode {
            opcode: instructions::Opcode::DEC,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xD6 => FullOpcode {
            opcode: instructions::Opcode::DEC,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xCE => FullOpcode {
            opcode: instructions::Opcode::DEC,
            addressing_mode: AddressingMode::Absolute,
        },
        0xDE => FullOpcode {
            opcode: instructions::Opcode::DEC,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0xCA => FullOpcode {
            opcode: instructions::Opcode::DEX,
            addressing_mode: AddressingMode::Implied,
        },
        0x88 => FullOpcode {
            opcode: instructions::Opcode::DEY,
            addressing_mode: AddressingMode::Implied,
        },
        0x49 => FullOpcode {
            opcode: instructions::Opcode::EOR,
            addressing_mode: AddressingMode::Immediate,
        },
        0x45 => FullOpcode {
            opcode: instructions::Opcode::EOR,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x55 => FullOpcode {
            opcode: instructions::Opcode::EOR,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x4D => FullOpcode {
            opcode: instructions::Opcode::EOR,
            addressing_mode: AddressingMode::Absolute,
        },
        0x5D => FullOpcode {
            opcode: instructions::Opcode::EOR,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x59 => FullOpcode {
            opcode: instructions::Opcode::EOR,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0x41 => FullOpcode {
            opcode: instructions::Opcode::EOR,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0x51 => FullOpcode {
            opcode: instructions::Opcode::EOR,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0xE6 => FullOpcode {
            opcode: instructions::Opcode::INC,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xF6 => FullOpcode {
            opcode: instructions::Opcode::INC,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xEE => FullOpcode {
            opcode: instructions::Opcode::INC,
            addressing_mode: AddressingMode::Absolute,
        },
        0xFE => FullOpcode {
            opcode: instructions::Opcode::INC,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0xE8 => FullOpcode {
            opcode: instructions::Opcode::INX,
            addressing_mode: AddressingMode::Implied,
        },
        0xC8 => FullOpcode {
            opcode: instructions::Opcode::INY,
            addressing_mode: AddressingMode::Implied,
        },
        0x4C => FullOpcode {
            opcode: instructions::Opcode::JMP,
            addressing_mode: AddressingMode::Absolute,
        },
        0x6C => FullOpcode {
            opcode: instructions::Opcode::JMP,
            addressing_mode: AddressingMode::Indirect,
        },
        0x20 => FullOpcode {
            opcode: instructions::Opcode::JSR,
            addressing_mode: AddressingMode::Absolute,
        },
        0xA9 => FullOpcode {
            opcode: instructions::Opcode::LDA,
            addressing_mode: AddressingMode::Immediate,
        },
        0xA5 => FullOpcode {
            opcode: instructions::Opcode::LDA,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xB5 => FullOpcode {
            opcode: instructions::Opcode::LDA,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xAD => FullOpcode {
            opcode: instructions::Opcode::LDA,
            addressing_mode: AddressingMode::Absolute,
        },
        0xBD => FullOpcode {
            opcode: instructions::Opcode::LDA,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0xB9 => FullOpcode {
            opcode: instructions::Opcode::LDA,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0xA1 => FullOpcode {
            opcode: instructions::Opcode::LDA,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0xB1 => FullOpcode {
            opcode: instructions::Opcode::LDA,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0xA2 => FullOpcode {
            opcode: instructions::Opcode::LDX,
            addressing_mode: AddressingMode::Immediate,
        },
        0xA6 => FullOpcode {
            opcode: instructions::Opcode::LDX,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xB6 => FullOpcode {
            opcode: instructions::Opcode::LDX,
            addressing_mode: AddressingMode::ZeropageYIndexed,
        },
        0xAE => FullOpcode {
            opcode: instructions::Opcode::LDX,
            addressing_mode: AddressingMode::Absolute,
        },
        0xBE => FullOpcode {
            opcode: instructions::Opcode::LDX,
            addressing_mode: AddressingMode::ZeropageYIndexed,
        },
        0xA0 => FullOpcode {
            opcode: instructions::Opcode::LDY,
            addressing_mode: AddressingMode::Immediate,
        },
        0xA4 => FullOpcode {
            opcode: instructions::Opcode::LDY,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xB4 => FullOpcode {
            opcode: instructions::Opcode::LDY,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xAC => FullOpcode {
            opcode: instructions::Opcode::LDY,
            addressing_mode: AddressingMode::Absolute,
        },
        0xBC => FullOpcode {
            opcode: instructions::Opcode::LDY,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x4A => FullOpcode {
            opcode: instructions::Opcode::LSR,
            addressing_mode: AddressingMode::Accumulator,
        },
        0x46 => FullOpcode {
            opcode: instructions::Opcode::LSR,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x56 => FullOpcode {
            opcode: instructions::Opcode::LSR,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x4E => FullOpcode {
            opcode: instructions::Opcode::LSR,
            addressing_mode: AddressingMode::Absolute,
        },
        0x5E => FullOpcode {
            opcode: instructions::Opcode::LSR,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0xEA => FullOpcode {
            opcode: instructions::Opcode::NOP,
            addressing_mode: AddressingMode::Implied,
        },
        0x09 => FullOpcode {
            opcode: instructions::Opcode::ORA,
            addressing_mode: AddressingMode::Immediate,
        },
        0x05 => FullOpcode {
            opcode: instructions::Opcode::ORA,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x15 => FullOpcode {
            opcode: instructions::Opcode::ORA,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x0D => FullOpcode {
            opcode: instructions::Opcode::ORA,
            addressing_mode: AddressingMode::Absolute,
        },
        0x1D => FullOpcode {
            opcode: instructions::Opcode::ORA,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x19 => FullOpcode {
            opcode: instructions::Opcode::ORA,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0x01 => FullOpcode {
            opcode: instructions::Opcode::ORA,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0x11 => FullOpcode {
            opcode: instructions::Opcode::ORA,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0x48 => FullOpcode {
            opcode: instructions::Opcode::PHA,
            addressing_mode: AddressingMode::Implied,
        },
        0x08 => FullOpcode {
            opcode: instructions::Opcode::PHP,
            addressing_mode: AddressingMode::Implied,
        },
        0x68 => FullOpcode {
            opcode: instructions::Opcode::PLA,
            addressing_mode: AddressingMode::Implied,
        },
        0x28 => FullOpcode {
            opcode: instructions::Opcode::PLP,
            addressing_mode: AddressingMode::Implied,
        },
        0x2A => FullOpcode {
            opcode: instructions::Opcode::ROL,
            addressing_mode: AddressingMode::Accumulator,
        },
        0x26 => FullOpcode {
            opcode: instructions::Opcode::ROL,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x36 => FullOpcode {
            opcode: instructions::Opcode::ROL,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x2E => FullOpcode {
            opcode: instructions::Opcode::ROL,
            addressing_mode: AddressingMode::Absolute,
        },
        0x3E => FullOpcode {
            opcode: instructions::Opcode::ROL,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x6A => FullOpcode {
            opcode: instructions::Opcode::ROR,
            addressing_mode: AddressingMode::Accumulator,
        },
        0x66 => FullOpcode {
            opcode: instructions::Opcode::ROR,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x76 => FullOpcode {
            opcode: instructions::Opcode::ROR,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x6E => FullOpcode {
            opcode: instructions::Opcode::ROR,
            addressing_mode: AddressingMode::Absolute,
        },
        0x7E => FullOpcode {
            opcode: instructions::Opcode::ROR,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x40 => FullOpcode {
            opcode: instructions::Opcode::RTI,
            addressing_mode: AddressingMode::Implied,
        },
        0x60 => FullOpcode {
            opcode: instructions::Opcode::RTS,
            addressing_mode: AddressingMode::Implied,
        },
        0xE9 => FullOpcode {
            opcode: instructions::Opcode::SBC,
            addressing_mode: AddressingMode::Immediate,
        },
        0xE5 => FullOpcode {
            opcode: instructions::Opcode::SBC,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xF5 => FullOpcode {
            opcode: instructions::Opcode::SBC,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xED => FullOpcode {
            opcode: instructions::Opcode::SBC,
            addressing_mode: AddressingMode::Absolute,
        },
        0xFD => FullOpcode {
            opcode: instructions::Opcode::SBC,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0xF9 => FullOpcode {
            opcode: instructions::Opcode::SBC,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0xE1 => FullOpcode {
            opcode: instructions::Opcode::SBC,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0xF1 => FullOpcode {
            opcode: instructions::Opcode::SBC,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0x38 => FullOpcode {
            opcode: instructions::Opcode::SEC,
            addressing_mode: AddressingMode::Implied,
        },
        0xF8 => FullOpcode {
            opcode: instructions::Opcode::SED,
            addressing_mode: AddressingMode::Implied,
        },
        0x78 => FullOpcode {
            opcode: instructions::Opcode::SEI,
            addressing_mode: AddressingMode::Implied,
        },
        0x85 => FullOpcode {
            opcode: instructions::Opcode::STA,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x95 => FullOpcode {
            opcode: instructions::Opcode::STA,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x8D => FullOpcode {
            opcode: instructions::Opcode::STA,
            addressing_mode: AddressingMode::Absolute,
        },
        0x9D => FullOpcode {
            opcode: instructions::Opcode::STA,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x99 => FullOpcode {
            opcode: instructions::Opcode::STA,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0x81 => FullOpcode {
            opcode: instructions::Opcode::STA,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0x91 => FullOpcode {
            opcode: instructions::Opcode::STA,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0x86 => FullOpcode {
            opcode: instructions::Opcode::STX,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x96 => FullOpcode {
            opcode: instructions::Opcode::STX,
            addressing_mode: AddressingMode::ZeropageYIndexed,
        },
        0x8E => FullOpcode {
            opcode: instructions::Opcode::STX,
            addressing_mode: AddressingMode::Absolute,
        },
        0x84 => FullOpcode {
            opcode: instructions::Opcode::STY,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x94 => FullOpcode {
            opcode: instructions::Opcode::STY,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x8C => FullOpcode {
            opcode: instructions::Opcode::STY,
            addressing_mode: AddressingMode::Absolute,
        },
        0xAA => FullOpcode {
            opcode: instructions::Opcode::TAX,
            addressing_mode: AddressingMode::Implied,
        },
        0xA8 => FullOpcode {
            opcode: instructions::Opcode::TAX,
            addressing_mode: AddressingMode::Implied,
        },
        0xBA => FullOpcode {
            opcode: instructions::Opcode::TAX,
            addressing_mode: AddressingMode::Implied,
        },
        0x8A => FullOpcode {
            opcode: instructions::Opcode::TAX,
            addressing_mode: AddressingMode::Implied,
        },
        0x9A => FullOpcode {
            opcode: instructions::Opcode::TAX,
            addressing_mode: AddressingMode::Implied,
        },
        0x98 => FullOpcode {
            opcode: instructions::Opcode::TAX,
            addressing_mode: AddressingMode::Implied,
        },
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_carry() {
        let mut flag_reg = ProcessorStatus::default();
        assert!(!flag_reg.carry_flag());

        flag_reg.set_carry_flag();
        assert!(flag_reg.carry_flag());

        flag_reg.clear_carry_flag();

        assert!(!flag_reg.carry_flag());
    }

    #[test]
    fn test_zero() {
        let mut flag_reg = ProcessorStatus::default();
        assert!(!flag_reg.zero_flag());

        flag_reg.set_zero_flag();
        assert!(flag_reg.zero_flag());

        flag_reg.clear_zero_flag();

        assert!(!flag_reg.zero_flag());
    }

    #[test]
    fn test_interrupt_disable() {
        let mut flag_reg = ProcessorStatus::default();
        assert!(!flag_reg.interrupt_disable_flag());

        flag_reg.set_interrupt_disable_flag();
        assert!(flag_reg.interrupt_disable_flag());

        flag_reg.clear_interrupt_disable_flag();
        assert!(!flag_reg.interrupt_disable_flag());
    }

    #[test]
    fn test_decimal() {
        let mut flag_reg = ProcessorStatus::default();
        assert!(!flag_reg.decimal_flag());

        flag_reg.set_decimal_flag();
        assert!(flag_reg.decimal_flag());

        flag_reg.clear_decimal_flag();
        assert!(!flag_reg.decimal_flag());
    }

    #[test]
    fn test_overflow() {
        let mut flag_reg = ProcessorStatus::default();
        assert!(!flag_reg.overflow_flag());

        flag_reg.set_overflow_flag();
        assert!(flag_reg.overflow_flag());

        flag_reg.clear_overflow_flag();
        assert!(!flag_reg.overflow_flag());
    }

    #[test]
    fn test_neg() {
        let mut flag_reg = ProcessorStatus::default();
        assert!(!flag_reg.negative_flag());

        flag_reg.set_negative_flag();
        assert!(flag_reg.negative_flag());

        flag_reg.clear_negative_flag();
        assert!(!flag_reg.negative_flag());
    }
}
