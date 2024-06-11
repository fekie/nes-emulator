use super::{
    absolute_read, absolute_x_read, absolute_y_read, handle_invalid_addressing_mode,
    immediate_read, indirect_x_read, indirect_y_read, zeropage_read, zeropage_x_read,
};
use super::{AddressingMode, CPU};
use crate::Bus;

impl CPU {
    pub(crate) fn instruction_and(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Immediate => {
                let value = immediate_read(low_byte) & self.accumulator;
                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);
                2
            }
            AddressingMode::Zeropage => {
                let value = zeropage_read(self, bus, low_byte) & self.accumulator;
                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);
                3
            }
            AddressingMode::ZeropageXIndexed => {
                let value = zeropage_x_read(self, bus, low_byte) & self.accumulator;
                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);
                4
            }
            AddressingMode::Absolute => {
                let value = absolute_read(self, bus, low_byte, high_byte) & self.accumulator;
                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);
                4
            }
            AddressingMode::AbsoluteXIndexed => {
                let (raw, page_changed) = absolute_x_read(self, bus, low_byte, high_byte);
                let value = raw & self.accumulator;

                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                match page_changed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::AbsoluteYIndexed => {
                let (raw, page_changed) = absolute_y_read(self, bus, low_byte, high_byte);
                let value = raw & self.accumulator;

                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                match page_changed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::IndirectXIndexed => {
                let value = indirect_x_read(self, bus, low_byte) & self.accumulator;

                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                6
            }
            AddressingMode::IndirectYIndexed => {
                let (raw, page_changed) = indirect_y_read(self, bus, low_byte);
                let value = raw & self.accumulator;

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

    pub(crate) fn instruction_eor(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Immediate => {
                let value = immediate_read(low_byte) ^ self.accumulator;
                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);
                2
            }
            AddressingMode::Zeropage => {
                let value = zeropage_read(self, bus, low_byte) ^ self.accumulator;
                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);
                3
            }
            AddressingMode::ZeropageXIndexed => {
                let value = zeropage_x_read(self, bus, low_byte) ^ self.accumulator;
                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);
                4
            }
            AddressingMode::Absolute => {
                let value = absolute_read(self, bus, low_byte, high_byte) ^ self.accumulator;
                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);
                4
            }
            AddressingMode::AbsoluteXIndexed => {
                let (raw, page_changed) = absolute_x_read(self, bus, low_byte, high_byte);
                let value = raw ^ self.accumulator;

                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                match page_changed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::AbsoluteYIndexed => {
                let (raw, page_changed) = absolute_y_read(self, bus, low_byte, high_byte);
                let value = raw ^ self.accumulator;

                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                match page_changed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::IndirectXIndexed => {
                let value = indirect_x_read(self, bus, low_byte) ^ self.accumulator;

                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                6
            }
            AddressingMode::IndirectYIndexed => {
                let (raw, page_changed) = indirect_y_read(self, bus, low_byte);
                let value = raw ^ self.accumulator;

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

    pub(crate) fn instruction_ora(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Immediate => {
                let value = immediate_read(low_byte) | self.accumulator;
                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);
                2
            }
            AddressingMode::Zeropage => {
                let value = zeropage_read(self, bus, low_byte) | self.accumulator;
                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);
                3
            }
            AddressingMode::ZeropageXIndexed => {
                let value = zeropage_x_read(self, bus, low_byte) | self.accumulator;
                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);
                4
            }
            AddressingMode::Absolute => {
                let value = absolute_read(self, bus, low_byte, high_byte) | self.accumulator;
                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);
                4
            }
            AddressingMode::AbsoluteXIndexed => {
                let (raw, page_changed) = absolute_x_read(self, bus, low_byte, high_byte);
                let value = raw | self.accumulator;

                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                match page_changed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::AbsoluteYIndexed => {
                let (raw, page_changed) = absolute_y_read(self, bus, low_byte, high_byte);
                let value = raw | self.accumulator;

                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                match page_changed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::IndirectXIndexed => {
                let value = indirect_x_read(self, bus, low_byte) | self.accumulator;

                self.accumulator = value;
                self.modify_negative_flag(value);
                self.modify_zero_flag(value);

                6
            }
            AddressingMode::IndirectYIndexed => {
                let (raw, page_changed) = indirect_y_read(self, bus, low_byte);
                let value = raw | self.accumulator;

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

    pub(crate) fn instruction_bit(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Zeropage => {
                let raw = zeropage_read(self, bus, low_byte);

                // check bit 7
                match (raw & 0b1000_0000) != 0 {
                    true => self.processor_status.set_negative_flag(),
                    false => self.processor_status.clear_negative_flag(),
                };

                // check bit 6
                match (raw & 0b0100_0000) != 0 {
                    true => self.processor_status.set_overflow_flag(),
                    false => self.processor_status.clear_overflow_flag(),
                };

                self.modify_zero_flag(raw & self.accumulator);

                3
            }
            AddressingMode::Absolute => {
                let raw = absolute_read(self, bus, low_byte, high_byte);

                // check bit 7
                match (raw & 0b1000_0000) != 0 {
                    true => self.processor_status.set_negative_flag(),
                    false => self.processor_status.clear_negative_flag(),
                };

                // check bit 6
                match (raw & 0b0100_0000) != 0 {
                    true => self.processor_status.set_overflow_flag(),
                    false => self.processor_status.clear_overflow_flag(),
                };

                self.modify_zero_flag(raw & self.accumulator);

                4
            }
            _ => handle_invalid_addressing_mode(),
        }
    }
}
