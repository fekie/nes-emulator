use super::{AddressingMode, Bus, CPU};
use crate::cpu::ProcessorStatus;

impl CPU {
    pub(crate) fn instruction_tsx(&mut self) -> u8 {
        self.x = self.stack_pointer;
        2
    }

    pub(crate) fn instruction_txs(&mut self) -> u8 {
        self.stack_pointer = self.x;
        2
    }

    pub(crate) fn instruction_pha(&mut self, bus: &Bus) -> u8 {
        self.push(bus, self.accumulator);
        3
    }

    pub(crate) fn instruction_php(&mut self, bus: &Bus) -> u8 {
        self.push(bus, self.processor_status.0);

        3
    }

    pub(crate) fn instruction_pla(&mut self, bus: &Bus) -> u8 {
        self.accumulator = self.pop(bus);

        4
    }

    pub(crate) fn instruction_plp(&mut self, bus: &Bus) -> u8 {
        self.processor_status = ProcessorStatus(self.pop(bus));

        4
    }
}
