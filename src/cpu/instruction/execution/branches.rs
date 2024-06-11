use super::{
    absolute_read, absolute_x_read, absolute_y_read, handle_invalid_addressing_mode,
    immediate_read, indirect_x_read, indirect_y_read, twos_compliment_to_signed, zeropage_read,
    zeropage_x_read,
};
use super::{AddressingMode, CPU};
use crate::Bus;

impl CPU {
    pub(crate) fn instruction_bcc(&mut self, low_byte: Option<u8>) -> u8 {
        let value = twos_compliment_to_signed(low_byte.unwrap());
        let original_page = self.program_counter >> 8;
        let needs_branch = !self.processor_status.carry_flag();

        if needs_branch {
            match value.is_positive() {
                true => self.program_counter - value as u16,
                false => self.program_counter - (-value) as u16,
            };
        }

        let new_page = self.program_counter >> 8;
        let page_crossed = original_page != new_page;

        match needs_branch {
            true => match page_crossed {
                true => 4,
                false => 3,
            },
            false => 2,
        }
    }

    pub(crate) fn instruction_bcs(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(crate) fn instruction_beq(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(crate) fn instruction_bmi(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(crate) fn instruction_bne(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(crate) fn instruction_bpl(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(crate) fn instruction_bvc(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(crate) fn instruction_bvs(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }
}
