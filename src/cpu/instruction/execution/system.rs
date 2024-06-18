use super::pack_bytes;
use super::{AddressingMode, CPU};
use crate::cpu::processor_status::ProcessorStatus;
use crate::Bus;

impl CPU {
    pub(crate) fn instruction_brk(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(crate) fn instruction_nop(&mut self) -> u8 {
        2
    }

    pub(crate) fn instruction_rti(&mut self, bus: &Bus) -> u8 {
        // ignore the new break flag and bit 5
        self.processor_status = ProcessorStatus(
            (self.pop(bus) & 0b1100_1111) | (self.processor_status.0 & 0b0011_0000),
        );

        let pc_low = self.pop(bus);
        let pc_high: u8 = self.pop(bus);

        self.program_counter = pack_bytes(pc_low, pc_high);

        6
    }
}
