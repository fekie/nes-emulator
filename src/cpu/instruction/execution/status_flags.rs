use super::CPU;

impl CPU {
    pub(crate) fn instruction_clc(&mut self) -> u8 {
        self.processor_status.clear_carry_flag();
        2
    }

    pub(crate) fn instruction_cld(&mut self) -> u8 {
        self.processor_status.clear_decimal_flag();
        2
    }

    pub(crate) fn instruction_cli(&mut self) -> u8 {
        self.processor_status.clear_interrupt_disable_flag();
        2
    }

    pub(crate) fn instruction_clv(&mut self) -> u8 {
        self.processor_status.clear_overflow_flag();
        2
    }

    pub(crate) fn instruction_sec(&mut self) -> u8 {
        self.processor_status.set_carry_flag();
        2
    }

    pub(crate) fn instruction_sed(&mut self) -> u8 {
        self.processor_status.set_decimal_flag();
        2
    }

    pub(crate) fn instruction_sei(&mut self) -> u8 {
        self.processor_status.set_interrupt_disable_flag();
        2
    }
}
