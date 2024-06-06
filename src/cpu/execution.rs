use super::{
    instruction::{AddressingMode, Instruction},
    CPU,
};
use crate::Bus;

impl CPU {
    /// Executes instruction [`Instruction::ADC`]. Returns amount of instructions taken.
    pub(super) fn instruction_adc(&mut self, instruction: Instruction, bus: &Bus) -> u8 {
        todo!()
    }

    /// Executes instruction [`Instruction::ADC`]. Returns true if a page boundary was crossed.
    pub(super) fn instruction_sei(&mut self, bus: &Bus, addressing_mode: AddressingMode) -> u8 {
        self.processor_status.set_interrupt_disable_flag();
        todo!()
    }
}
