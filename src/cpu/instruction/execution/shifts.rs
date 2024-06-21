use super::{
    absolute_read, absolute_write, absolute_x_read, absolute_x_write, absolute_y_read,
    handle_invalid_addressing_mode, immediate_read, indirect_x_read, indirect_y_read,
    zeropage_read, zeropage_write, zeropage_x_read, zeropage_x_write,
};
use super::{AddressingMode, CPU};
use crate::Bus;

impl CPU {
    pub(crate) fn instruction_asl(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Accumulator => {
                match (self.accumulator >> 7) != 0 {
                    true => self.processor_status.set_carry_flag(),
                    false => self.processor_status.clear_carry_flag(),
                };

                self.accumulator <<= 1;

                self.modify_zero_flag(self.accumulator);

                2
            }
            AddressingMode::Zeropage => {
                let mut value = zeropage_read(self, bus, low_byte);

                match (value >> 7) != 0 {
                    true => self.processor_status.set_carry_flag(),
                    false => self.processor_status.clear_carry_flag(),
                };

                value <<= 1;

                self.modify_zero_flag(value);

                zeropage_write(self, bus, low_byte, value);

                5
            }
            AddressingMode::ZeropageXIndexed => {
                let mut value = zeropage_x_read(self, bus, low_byte);

                match (value >> 7) != 0 {
                    true => self.processor_status.set_carry_flag(),
                    false => self.processor_status.clear_carry_flag(),
                };

                value <<= 1;

                self.modify_zero_flag(value);

                zeropage_x_write(self, bus, low_byte, value);

                6
            }
            AddressingMode::Absolute => {
                let mut value = absolute_read(self, bus, low_byte, high_byte);

                match (value >> 7) != 0 {
                    true => self.processor_status.set_carry_flag(),
                    false => self.processor_status.clear_carry_flag(),
                };

                value <<= 1;

                self.modify_zero_flag(value);

                absolute_write(self, bus, low_byte, high_byte, value);

                6
            }
            AddressingMode::AbsoluteXIndexed => {
                let (mut value, _) = absolute_x_read(self, bus, low_byte, high_byte);

                match (value >> 7) != 0 {
                    true => self.processor_status.set_carry_flag(),
                    false => self.processor_status.clear_carry_flag(),
                };

                value <<= 1;

                self.modify_zero_flag(value);

                absolute_x_write(self, bus, low_byte, high_byte, value);

                7
            }
            _ => handle_invalid_addressing_mode(),
        }
    }

    pub(crate) fn instruction_lsr(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(crate) fn instruction_rol(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(crate) fn instruction_ror(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }
}
