use super::{
    absolute_read, absolute_x_read, absolute_y_read, handle_invalid_addressing_mode,
    immediate_read, indirect_x_read, indirect_y_read, zeropage_read, zeropage_x_read,
};
use super::{AddressingMode, CPU};
use crate::Bus;

impl CPU {
    pub(crate) fn instruction_lda(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Immediate => {
                let value = immediate_read(low_byte);

                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                2
            }
            AddressingMode::Zeropage => {
                let value = zeropage_read(self, bus, low_byte);

                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                3
            }
            AddressingMode::ZeropageXIndexed => {
                let value = zeropage_x_read(self, bus, low_byte);

                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                4
            }
            AddressingMode::Absolute => {
                let value = absolute_read(self, bus, low_byte, high_byte);

                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                4
            }
            AddressingMode::AbsoluteXIndexed => {
                let (value, page_changed) = absolute_x_read(self, bus, low_byte, high_byte);

                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                match page_changed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::AbsoluteYIndexed => {
                let (value, page_changed) = absolute_y_read(self, bus, low_byte, high_byte);

                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                match page_changed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::IndirectXIndexed => {
                let value = indirect_x_read(self, bus, low_byte);

                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                6
            }
            AddressingMode::IndirectYIndexed => {
                let (value, page_changed) = indirect_y_read(self, bus, low_byte);

                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                match page_changed {
                    true => 6,
                    false => 5,
                }
            }
            _ => handle_invalid_addressing_mode(),
        }
    }

    pub(crate) fn instruction_ldx(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Immediate => {
                let value = immediate_read(low_byte);

                self.x = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                2
            }
            AddressingMode::Zeropage => {
                let value = zeropage_read(self, bus, low_byte);

                self.x = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                3
            }
            AddressingMode::ZeropageXIndexed => {
                let value = zeropage_x_read(self, bus, low_byte);

                self.x = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                4
            }
            AddressingMode::Absolute => {
                let value = absolute_read(self, bus, low_byte, high_byte);

                self.x = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                4
            }
            AddressingMode::AbsoluteYIndexed => {
                let (value, page_changed) = absolute_y_read(self, bus, low_byte, high_byte);

                self.x = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                match page_changed {
                    true => 5,
                    false => 4,
                }
            }
            _ => handle_invalid_addressing_mode(),
        }
    }

    pub(crate) fn instruction_ldy(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Immediate => {
                let value = immediate_read(low_byte);

                self.y = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                2
            }
            AddressingMode::Zeropage => {
                let value = zeropage_read(self, bus, low_byte);

                self.y = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                3
            }
            AddressingMode::ZeropageXIndexed => {
                let value = zeropage_x_read(self, bus, low_byte);

                self.y = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                4
            }
            AddressingMode::Absolute => {
                let value = absolute_read(self, bus, low_byte, high_byte);

                self.y = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                4
            }
            AddressingMode::AbsoluteXIndexed => {
                let (value, page_changed) = absolute_x_read(self, bus, low_byte, high_byte);

                self.y = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                match page_changed {
                    true => 5,
                    false => 4,
                }
            }
            _ => handle_invalid_addressing_mode(),
        }
    }

    pub(crate) fn instruction_sta(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Zeropage => {
                todo!()
            }
            AddressingMode::ZeropageXIndexed => todo!(),
            AddressingMode::Absolute => todo!(),
            AddressingMode::AbsoluteXIndexed => todo!(),
            AddressingMode::AbsoluteYIndexed => todo!(),
            AddressingMode::IndirectXIndexed => todo!(),
            AddressingMode::IndirectYIndexed => todo!(),
            _ => handle_invalid_addressing_mode(),
        }
    }

    pub(crate) fn instruction_stx(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(crate) fn instruction_sty(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }
}
