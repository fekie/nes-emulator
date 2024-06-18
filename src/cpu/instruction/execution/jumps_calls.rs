use super::{handle_invalid_addressing_mode, pack_bytes, pack_bytes_wrapped, unpack_bytes};
use super::{AddressingMode, CPU};
use crate::Bus;

impl CPU {
    pub(crate) fn instruction_jmp(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Absolute => {
                self.program_counter = pack_bytes_wrapped(low_byte, high_byte);
                3
            }
            AddressingMode::Indirect => {
                // We do an indrect read here. We do not have a general function
                // as JMP is the only instruction that uses it
                let base_address = pack_bytes_wrapped(low_byte, high_byte);
                self.program_counter = pack_bytes(
                    self.read(bus, base_address),
                    self.read(bus, base_address + 1),
                );

                5
            }
            _ => handle_invalid_addressing_mode(),
        }
    }

    pub(crate) fn instruction_jsr(
        &mut self,
        bus: &Bus,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        let subroutine_address = pack_bytes_wrapped(low_byte, high_byte);

        // for whatever reason, https://www.nesdev.org/obelisk-6502-guide/reference.html#JSR says that
        // we only push pc - 1 to the stack instead of PC
        let (pc_low, pc_high) = unpack_bytes(self.program_counter - 1);

        self.push(bus, pc_high);
        self.push(bus, pc_low);

        self.program_counter = subroutine_address;

        6
    }

    pub(crate) fn instruction_rts(&mut self, bus: &Bus) -> u8 {
        let pc_low = self.pop(bus);
        let pc_high: u8 = self.pop(bus);

        // we have to add one because we subtracted 1 earlier, not sure why we have to do this exactly
        self.program_counter = pack_bytes(pc_low, pc_high) + 1;

        6
    }
}
