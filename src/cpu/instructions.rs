#![allow(non_snake_case)]

use super::Cpu;
use super::AddressingMode::{self, *};
pub enum Instructions { // fekie you can add some documentation here if you want, I aint even gonna bother
    ADC{adr: AddressingMode},
    AND{adr: AddressingMode},
    ASL{adr: AddressingMode},
    BCC{adr: AddressingMode},
    BCS{adr: AddressingMode},
    BEQ{adr: AddressingMode},
    BIT{adr: AddressingMode},
    BMI{adr: AddressingMode},
    BNE{adr: AddressingMode},
    BPL{adr: AddressingMode},
    BRK{adr: AddressingMode},
    BVC{adr: AddressingMode},
    BVS{adr: AddressingMode},
    CLC{adr: AddressingMode},
    CLD{adr: AddressingMode},
    CLI{adr: AddressingMode},
    CLV{adr: AddressingMode},
    CMP{adr: AddressingMode},
    CPX{adr: AddressingMode},
    CPY{adr: AddressingMode},
    DEC{adr: AddressingMode},
    DEx{adr: AddressingMode},
    DEY{adr: AddressingMode},
    EOR{adr: AddressingMode},
    INC{adr: AddressingMode},
    INX{adr: AddressingMode},
    INY{adr: AddressingMode},
    JMP{adr: AddressingMode},
    JSR{adr: AddressingMode},
    LDA{adr: AddressingMode},
    LDX{adr: AddressingMode},
    LDY{adr: AddressingMode},
    LSR{adr: AddressingMode},
    NOP{adr: AddressingMode},
    ORA{adr: AddressingMode},
    PHA{adr: AddressingMode},
    PHP{adr: AddressingMode},
    PLA{adr: AddressingMode},
    PLP{adr: AddressingMode},
    ROL{adr: AddressingMode},
    ROR{adr: AddressingMode},
    RTI{adr: AddressingMode},
    RTD{adr: AddressingMode},
    SBC{adr: AddressingMode},
    SEC{adr: AddressingMode},
    SED{adr: AddressingMode},
    SEI{adr: AddressingMode},
    STA{adr: AddressingMode},
    STX{adr: AddressingMode},
    STY{adr: AddressingMode},
    TAX{adr: AddressingMode},
    TAY{adr: AddressingMode},
    TSX{adr: AddressingMode},
    TXA{adr: AddressingMode},
    TXS{adr: AddressingMode},
    TYA{adr: AddressingMode},

}

use super::CPU;

impl CPU {
    pub fn ADC(&mut self, presumed_mem_byte: u8) {
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
    }
}
