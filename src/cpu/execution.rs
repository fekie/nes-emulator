use super::helper::{
    absolute, absolute_x, absolute_y, immediate, indirect_x, indirect_y, zeropage, zeropage_x,
};
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
                self.accumulator = immediate(low_byte);
                2
            }
            AddressingMode::Zeropage => {
                self.accumulator = zeropage(self, bus, low_byte);
                3
            }
            AddressingMode::ZeropageXIndexed => {
                self.accumulator = zeropage_x(self, bus, low_byte);
                4
            }
            AddressingMode::Absolute => {
                self.accumulator = absolute(self, bus, low_byte, high_byte);
                4
            }
            AddressingMode::AbsoluteXIndexed => {
                let (value, page_changed) = absolute_x(self, bus, low_byte, high_byte);

                self.accumulator = value;

                match page_changed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::AbsoluteYIndexed => {
                let (value, page_changed) = absolute_y(self, bus, low_byte, high_byte);

                self.accumulator = value;

                match page_changed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::IndirectXIndexed => {
                self.accumulator = indirect_x(self, bus, low_byte);
                6
            }
            AddressingMode::IndirectYIndexed => {
                let (value, page_changed) = indirect_y(self, bus, low_byte);

                self.accumulator = value;

                match page_changed {
                    true => 6,
                    false => 5,
                }
            }
            _ => panic!("Invalid addressing mode"),
        }
    }

    pub(super) fn instruction_ldx(&mut self) -> u8 {
        todo!()
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        cartridge::{self, Cartridge},
        ines::{Header, Ines},
    };

    fn create_cartridge(program: Vec<u8>) -> Cartridge {
        let mut rom = Ines::default();

        for (i, byte) in program.into_iter().enumerate() {
            rom.program_rom[i] = byte;
        }

        rom.into()
    }

    #[test]
    fn test_lda() {
        // program:
        // LDA #$55
        // LDA $44
        // LDA $33,X
        // LDA $0122
        // LDA $0111,X
        // LDA $0299,Y
        // LDA ($03,X)
        // LDA ($02),Y
        let cartridge = create_cartridge(vec![]);
        todo!()
    }
}
