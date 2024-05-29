#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]

use super::CPU;

//https://emudev.de/nes-emulator/opcodes-and-addressing-modes-the-6502/   <-- good stuff
//https://blogs.oregonstate.edu/ericmorgan/2022/01/21/6502-addressing-modes/  <--- also this too
#[derive(Clone, Copy)]
pub enum AddressingMode {
    Relative,
    Accumulator,
    Immediate,
    Implied,
    Zeropage,
    ZeropageXIndexed,
    ZeropageYIndexed,
    IndirectXIndexed,
    IndirectYIndexed,
    Absolute,
    AbsoluteXIndexed,
    AbsoluteYIndexed,
}

impl AddressingMode {
    /// Each instruction will require 1-3 bytes in total.
    /// This includes the opcode byte.
    pub fn bytes_required(&self) -> u16 {
        todo!()
    }
}

pub enum Opcode {
    /// High Nibble | Low Nibble | Addressing Mode
    /// 0x6 | 0x1 | indirect, x-indexed
    /// 0x6 | 0x5 | zero-page
    /// 0x6 | 0x9 | immediate
    /// 0x6 | 0xD | absolute
    /// 0x7 | 0x1 | indirect, y-indexed
    /// 0x7 | 0x5 | zero-page, x-indexed
    /// 0x7 | 0x9 | absolute, y-indexed
    /// 0x7 | 0xD | absolute, x-indexed
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTD,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

/// Includes both the opcode and the addressing mode from
/// the opcode byte.
pub struct FullOpcode {
    pub opcode: Opcode,
    pub addressing_mode: AddressingMode,
}

/* impl Opcode {
    pub fn addressing_mode(&self) -> AddressingMode {
        // forgive me lord, for i have sinned
        // I didn't want to leave the interface I had
        // for `Opcode` though
        // I couldn't think of a better way to do this because
        // the instruction is tied directly to the addressing mode
        // (as in the opcode byte is the instruction and the addressing mode,
        // but in a non-linear fashion).
        match self {
            Self::ADC(mode)
            | Self::AND(mode)
            | Self::ASL(mode)
            | Self::BCC(mode)
            | Self::BCS(mode)
            | Self::BEQ(mode)
            | Self::BIT(mode)
            | Self::BMI(mode)
            | Self::BNE(mode)
            | Self::BPL(mode)
            | Self::BRK(mode)
            | Self::BVC(mode)
            | Self::BVS(mode)
            | Self::CLC(mode)
            | Self::CLD(mode)
            | Self::CLI(mode)
            | Self::CLV(mode)
            | Self::CMP(mode)
            | Self::CPX(mode)
            | Self::CPY(mode)
            | Self::DEC(mode)
            | Self::DEX(mode)
            | Self::DEY(mode)
            | Self::EOR(mode)
            | Self::INC(mode)
            | Self::INX(mode)
            | Self::INY(mode)
            | Self::JMP(mode)
            | Self::JSR(mode)
            | Self::LDA(mode)
            | Self::LDX(mode)
            | Self::LDY(mode)
            | Self::LSR(mode)
            | Self::NOP(mode)
            | Self::ORA(mode)
            | Self::PHA(mode)
            | Self::PHP(mode)
            | Self::PLA(mode)
            | Self::PLP(mode)
            | Self::ROL(mode)
            | Self::ROR(mode)
            | Self::RTI(mode)
            | Self::RTD(mode)
            | Self::SBC(mode)
            | Self::SEC(mode)
            | Self::SED(mode)
            | Self::SEI(mode)
            | Self::STA(mode)
            | Self::STX(mode)
            | Self::STY(mode)
            | Self::TAX(mode)
            | Self::TAY(mode)
            | Self::TSX(mode)
            | Self::TXA(mode)
            | Self::TXS(mode)
            | Self::TYA(mode) => *mode,
        }
    }
}
 */
impl FullOpcode {
    pub fn new(byte: u8) -> Self {
        todo!()
    }
}

pub struct Instruction {
    pub opcode: Opcode,
    pub addressing_mode: AddressingMode,
    pub low_byte: Option<u8>,
    pub high_byte: Option<u8>,
}

impl CPU {
    /* pub fn ADC(&mut self, presumed_mem_byte: u8) {
        //Add with Carry
        let wrapped_sum = self.accumulator_register.wrapping_add(presumed_mem_byte);

        let overflow_ocurred = self
            .accumulator_register
            .checked_add(presumed_mem_byte)
            .is_none();

        self.accumulator_register = wrapped_sum;
        if overflow_ocurred {
            // crazy overhead, am i right?
            self.processor_status.set_carry_flag()
        }
    }

    pub fn AND(&mut self, presumed_mem_byte: u8) {
        let _ = self.accumulator_register & presumed_mem_byte;
    }

    pub fn ASL(&mut self, presumed_mem_byte: u8) {}

    pub fn JMP(&mut self, presumed_argument: u8) {
        self.program_counter = presumed_argument.into();
    }

    pub fn BCC(&mut self) {
        if !self.processor_status.carry_flag() {
            self.program_counter;
        }
    }

    // all the instructions starting with T are implied addressing
    pub fn TAX(&mut self) {
        self.x_register = self.accumulator_register;
    }
    pub fn TAY(&mut self) {
        self.y_register = self.accumulator_register;
    }
    pub fn TSX(&mut self) {
        self.x_register = self.stack_pointer;
    }
    pub fn TXA(&mut self) {
        self.accumulator_register = self.x_register;
    }
    pub fn TXS(&mut self) {
        self.stack_pointer = self.x_register;
    }
    pub fn TYA(&mut self) {
        self.accumulator_register = self.y_register;
    }

    pub fn CLC(&mut self) {
        self.processor_status.clear_carry_flag();
    }

    pub fn CLD(&mut self) {
        self.processor_status.clear_decimal_flag();
    }

    pub fn CLI(&mut self) {
        self.processor_status.clear_interrupt_disable_flag();
    }

    pub fn CLV(&mut self) {
        self.processor_status.clear_overflow_flag();
    }

    pub fn SEC(&mut self) {
        self.processor_status.set_carry_flag();
    }

    pub fn SED(&mut self) {
        self.processor_status.set_decimal_flag();
    }

    pub fn SEI(&mut self) {
        self.processor_status.set_interrupt_disable_flag();
    } */
}
