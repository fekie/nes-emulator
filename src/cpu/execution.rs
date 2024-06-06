use super::{instruction::AddressingMode, CPU};
use crate::Bus;

impl CPU {
    pub(super) fn instruction_lda(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Immediate => {
                self.accumulator = low_byte.unwrap();
                2
            }
            AddressingMode::Zeropage => {
                let address = low_byte.unwrap() as u16;
                self.accumulator = self.read(bus, address);
                3
            }
            AddressingMode::ZeropageXIndexed => {
                let address = low_byte.unwrap().wrapping_add(self.accumulator) as u16;
                self.accumulator = self.read(bus, address);
                4
            }
            AddressingMode::Absolute => {
                let address = pack_bytes_wrapped(low_byte, high_byte);
                self.accumulator = self.read(bus, address);
                4
            }
            AddressingMode::AbsoluteXIndexed => {
                let pre_add_address = pack_bytes_wrapped(low_byte, high_byte);
                let address = pre_add_address.wrapping_add(self.x as u16);

                let page_changed = low_byte.unwrap().checked_add(self.x).is_none();

                self.accumulator = self.read(bus, address);

                match page_changed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::AbsoluteYIndexed => {
                let pre_add_address = pack_bytes_wrapped(low_byte, high_byte);
                let address = pre_add_address.wrapping_add(self.y as u16);

                let page_changed = low_byte.unwrap().checked_add(self.y).is_none();

                self.accumulator = self.read(bus, address);

                match page_changed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::IndirectXIndexed => {
                let base_address = low_byte.unwrap().wrapping_add(self.x) as u16;

                let resolved_address = pack_bytes(
                    self.read(bus, base_address),
                    self.read(bus, base_address + 1),
                );

                self.accumulator = self.read(bus, resolved_address);

                6
            }
            AddressingMode::IndirectYIndexed => {
                let low_base_address = low_byte.unwrap() as u16;
                let high_base_address = low_byte.unwrap().wrapping_add(1) as u16;

                let page_changed = low_base_address > high_base_address;

                let resolved_address = pack_bytes(
                    self.read(bus, low_base_address),
                    self.read(bus, high_base_address),
                );

                self.accumulator = self.read(bus, resolved_address);

                match page_changed {
                    true => 6,
                    false => 5,
                }
            }
            _ => panic!("Invalid addressing mode"),
        }
    }

    pub(super) fn instruction_sei(&mut self) -> u8 {
        self.processor_status.set_interrupt_disable_flag();
        2
    }

    pub(super) fn instruction_cld(&mut self) -> u8 {
        self.processor_status.clear_decimal_flag();
        2
    }
}

fn pack_bytes(low_byte: u8, high_byte: u8) -> u16 {
    ((high_byte as u16) << 8) | low_byte as u16
}

fn pack_bytes_wrapped(low_byte: Option<u8>, high_byte: Option<u8>) -> u16 {
    ((high_byte.unwrap() as u16) << 8) | low_byte.unwrap() as u16
}
