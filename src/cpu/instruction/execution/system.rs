use super::{pack_bytes, unpack_bytes};
use super::{AddressingMode, CPU};
use crate::cpu::processor_status::ProcessorStatus;
use crate::cpu::IRQ_BRK_VECTOR_ADDRESS;
use crate::Bus;

impl CPU {
    pub(crate) fn instruction_brk(&mut self, bus: &Bus) -> u8 {
        let (pc_low, pc_high) = unpack_bytes(self.program_counter);

        self.push(bus, pc_high);
        self.push(bus, pc_low);

        self.processor_status.set_break_flag();

        self.push(bus, self.processor_status.0);

        self.program_counter = IRQ_BRK_VECTOR_ADDRESS;

        7
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
