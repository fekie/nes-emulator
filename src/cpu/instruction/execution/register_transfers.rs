use super::CPU;

impl CPU {
    pub(crate) fn instruction_tax(&mut self) -> u8 {
        self.x = self.accumulator;
        self.modify_negative_flag(self.x);
        self.modify_zero_flag(self.x);
        2
    }

    pub(crate) fn instruction_tay(&mut self) -> u8 {
        self.y = self.accumulator;
        self.modify_negative_flag(self.y);
        self.modify_zero_flag(self.y);
        2
    }

    pub(crate) fn instruction_txa(&mut self) -> u8 {
        self.accumulator = self.x;
        self.modify_negative_flag(self.accumulator);
        self.modify_zero_flag(self.accumulator);
        2
    }

    pub(crate) fn instruction_tya(&mut self) -> u8 {
        self.accumulator = self.y;
        self.modify_negative_flag(self.accumulator);
        self.modify_zero_flag(self.accumulator);
        2
    }
}
