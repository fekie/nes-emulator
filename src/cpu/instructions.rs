use super::Cpu;

impl Cpu {
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
    pub fn ASL(&mut self, presumed_mem_byte: u8) {

    }

    pub fn JMP(&mut self, presumed_argument: u8) {
        self.program_counter = presumed_argument.into();
    }
    


    pub fn CLC (&mut self) {self.processor_status.clear_carry_flag();}
    pub fn CLD (&mut self) {self.processor_status.clear_deci_flag();}
    pub fn CLI (&mut self) {self.processor_status.clear_inter_flag();}
    pub fn CLV (&mut self) {self.processor_status.clear_over_flag();}
    pub fn SEC (&mut self) {self.processor_status.set_carry_flag();}
    pub fn SED (&mut self) {self.processor_status.set_deci_flag();}
    pub fn SEI (&mut self) {self.processor_status.set_inter_flag();}
}
