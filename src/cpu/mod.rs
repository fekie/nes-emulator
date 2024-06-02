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

/* pub fn decodeOPandADR(opcode: u8) -> Instructions {
    // just some sketches, can change later on
    match opcode {
        0x69 => Instructions::ADC {
            adr: AddressingMode::Implied,
        },
        _ => unimplemented!(),
        /* 0x65 => Instructions::ADC { adr: AddressingMode::Zeropage { adr: () } }, // ok for stuff that needs actual operands, we can manually get them by looking at the next mem locations ofter the opcode
        0x75 => Instructions::ADC { adr: AddressingMode::AbsoluteXIndex { adr: (), X: () } },
        0x6D => Instructions::ADC { adr: AddressingMode::Absolute { adr: () } },
        0x7D => Instructions::ADC { adr: AddressingMode::AbsoluteXIndex { adr: (), X: () } }
        0x79 => Instructions::ADC { adr: AddressingMode::AbsoluteYIndex { adr: (), Y: () } }
        0x61 => Instructions::ADC { adr: AddressingMode::IndirectXIndex { adr: (), X: () } }
        0x71 => Instructions::ADC { adr: AddressingMode::IndirectYIndex { adr: (), Y: () } }
        0x29 => Instructions::AND { adr: AddressingMode::Immediate { adr: () } }
        0x25 => Instructions::AND { adr: AddressingMode::Zeropage { adr: () } }
        0x35 => Instructions::AND { adr: AddressingMode::ZeropageXIndex { adr: (), X: () } }
        0x2D => Instructions::AND { adr: AddressingMode::Absolute { adr: () } }
        0x3D => Instructions::AND { adr: AddressingMode::AbsoluteXIndex { adr: (), X: () } }
        0x39 => Instructions::AND { adr: AddressingMode::AbsoluteYIndex { adr: (), Y: () } }
        0x21 => Instructions::AND { adr: AddressingMode::IndirectXIndex { adr: (), X: () } }
        0x31 => Instructions::AND { adr: AddressingMode::AbsoluteYIndex { adr: (), Y: () } }
        0x0A => Instructions::ASL { adr: AddressingMode::Accumulator { accumulator: () } }
        0x06 => Instructions::ASL { adr: AddressingMode::Zeropage { adr: () } }
        0x16 => Instructions::ASL { adr: AddressingMode::ZeropageXIndex { adr: (), X: () } }
        0x0E => Instructions::ASL { adr: AddressingMode::Absolute { adr: () } }
        0x1E => Instructions::ASL { adr: AddressingMode::AbsoluteXIndex { adr: (), X: () } }
        0x90 => Instructions::BCC { adr: AddressingMode::Relative { offset: () } }
        0xB0 => Instructions::BCS { adr: AddressingMode::Relative { offset: () } }
        0xF0 => Instructions::BEQ { adr: AddressingMode::Relative { offset: () } }
        0x24 => Instructions::BIT { adr: AddressingMode::Zeropage { adr: () } }
        0x2C => Instructions::BIT { adr: AddressingMode::Absolute { adr: () } }
        0x30 => Instructions::BMI { adr: AddressingMode::Relative { offset: () } }
        0xD0 => Instructions::BNE { adr: AddressingMode::Relative { offset: () } }
        0x10 => Instructions::BPL { adr: AddressingMode::Relative { offset: () } }
        0x00 => Instructions::BRK { adr: AddressingMode::Implied }
        0x50 => Instructions::BVC { adr: AddressingMode::Relative { offset: () } }
        0x70 => Instructions::BVS { adr: AddressingMode::Relative { offset: () } }
        0x18 => Instructions::CLC { adr: AddressingMode::Implied }
        0xD8 => Instructions::CLD { adr: AddressingMode::Implied }
        0x58 => Instructions::CLI { adr: AddressingMode::Implied }
        0xB8 => Instructions::CLV { adr: AddressingMode::Implied }
        0xC9 => Instructions::CMP { adr: AddressingMode::Immediate { adr: () } }
        0xC5 => Instructions::CMP { adr: AddressingMode::Zeropage { adr: () }}
        0xD5 => Instructions::CMP { adr: AddressingMode::ZeropageXIndex { adr: (), X: () } }
        0xCD => Instructions::CMP { adr: AddressingMode::Absolute { adr: () } }
        0xDD => Instructions::CMP { adr: AddressingMode::AbsoluteXIndex { adr: (), X: () }}
        0xD9 => Instructions::CMP { adr: AddressingMode::AbsoluteYIndex { adr: (), Y: () }}
        0xC1 => Instructions::CMP { adr: AddressingMode::IndirectXIndex { adr: (), X: () }}
        0xD1 => Instructions::CMP { adr: AddressingMode::IndirectYIndex { adr: (), Y: () }}
        0xC0 => Instructions::CPY { adr: AddressingMode::Immediate { adr: () } }
        0xC4 => Instructions::CPY { adr: AddressingMode::Zeropage { adr: () }}
        0xCC => Instructions::CPY { adr: AddressingMode::Absolute { adr: () }}
        0xC6 => Instructions::DEC { adr: AddressingMode::Zeropage { adr: () } }
        0xD6 => Instructions::DEC { adr: AddressingMode::ZeropageXIndex { adr: (), X: () }}
        0xCE => Instructions::DEC { adr: AddressingMode::Absolute { adr: () } }
        0xDE => Instructions::DEC { adr: AddressingMode::AbsoluteXIndex { adr: (), X: () }}
        0xCA => Instructions::DEx { adr: AddressingMode::Implied }
        0x88 => Instructions::DEY { adr: AddressingMode::Implied }
        0x49 => Instructions::EOR { adr: AddressingMode::Immediate { adr: () } }
        0x45 => Instructions::EOR { adr: AddressingMode::Zeropage { adr: () }}
        0x55 => Instructions::EOR { adr: AddressingMode::ZeropageXIndex { adr: (), X: () }}
        0x4D => Instructions::EOR { adr: AddressingMode::Absolute { adr: () }}
        0x5D => Instructions::EOR { adr: AddressingMode::AbsoluteXIndex { adr: (), X: () }}
        0x59 => Instructions::EOR { adr: AddressingMode::AbsoluteYIndex { adr: (), Y: () }}
        0x41 => Instructions::EOR { adr: AddressingMode::IndirectXIndex { adr: (), X: () }}
        0x51 => Instructions::EOR { adr: AddressingMode::IndirectYIndex { adr: (), Y: () }} */
    }
} */

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
