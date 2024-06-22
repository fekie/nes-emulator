use super::{
    absolute_read, absolute_write, absolute_x_read, absolute_x_write, absolute_y_read,
    handle_invalid_addressing_mode, immediate_read, indirect_x_read, indirect_y_read,
    zeropage_read, zeropage_write, zeropage_x_read, zeropage_x_write,
};
use super::{AddressingMode, CPU};
use crate::Bus;

impl CPU {
    pub(crate) fn instruction_adc(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Immediate => {
                let value = immediate_read(low_byte);

                // If the sign bits are the same, then we need to check if they
                // are different later because that is an overflow.
                // If the sign bits are the same, we keep the sign in Some(), otherwise
                // it is None and we don't need to check for overflow
                let shared_sign = match (self.accumulator >> 7) == (value >> 7) {
                    true => Some(self.accumulator >> 7),
                    false => None,
                };

                // store whether we need a carry before modifying the values
                let carry_needed = (value as u16
                    + self.accumulator as u16
                    + self.processor_status.carry_flag() as u16)
                    > 255;

                self.accumulator = self.accumulator.wrapping_add(value);
                self.accumulator = self
                    .accumulator
                    .wrapping_add(self.processor_status.carry_flag() as u8);

                // Modify the carry flag
                match carry_needed {
                    true => self.processor_status.set_carry_flag(),
                    false => self.processor_status.clear_carry_flag(),
                };

                // Modify the overflow flag
                // If the signs were the same before the operation, they need to
                // have the same sign as the result
                match shared_sign {
                    Some(x) => match (self.accumulator >> 7) == (x) {
                        true => self.processor_status.clear_overflow_flag(),
                        false => self.processor_status.set_overflow_flag(),
                    },
                    None => self.processor_status.clear_overflow_flag(),
                }

                // Modify zero and negative flag
                self.modify_zero_flag(self.accumulator);
                self.modify_negative_flag(self.accumulator);

                2
            }
            AddressingMode::Zeropage => todo!(),
            AddressingMode::ZeropageXIndexed => todo!(),
            AddressingMode::Absolute => todo!(),
            AddressingMode::AbsoluteXIndexed => todo!(),
            AddressingMode::AbsoluteYIndexed => todo!(),
            AddressingMode::IndirectXIndexed => todo!(),
            AddressingMode::IndirectYIndexed => todo!(),
            _ => handle_invalid_addressing_mode(),
        }
    }

    pub(crate) fn instruction_sbc(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(crate) fn instruction_cmp(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(crate) fn instruction_cpx(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(crate) fn instruction_cpy(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }
}
