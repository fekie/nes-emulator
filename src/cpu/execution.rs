use super::helper::{
    absolute, absolute_x, absolute_y, immediate, indirect_x, indirect_y, zeropage, zeropage_x,
};
use super::{instruction::AddressingMode, CPU};
use crate::Bus;

impl CPU {
    // make placeholder functions for all of these instructions
    // ADC | AND | ASL | BCC | BCS | BEQ | BIT | BMI | BNE | BPL | BRK | BVC | BVS | CLC | CLD | CLI | CLV | CMP | CPX | CPY | DEC | DEX | DEY | EOR | INC | INX | INY | JMP | JSR | LDA | LDX | LDY | LSR | NOP | ORA | PHA | PHP | PLA | PLP | ROL | ROR | RTI | RTS | SBC | SEC | SED | SEI | STA | STX | STY | TAX | TAY | TSX | TXA | TXS | TYA |

    pub(super) fn instruction_adc(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_and(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_asl(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_bcc(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_bcs(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_beq(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_bit(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_bmi(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_bne(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_bpl(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_brk(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_bvc(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_bvs(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_clc(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_cld(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        self.processor_status.clear_decimal_flag();
        2
    }

    pub(super) fn instruction_cli(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_clv(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_cmp(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_cpx(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_cpy(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_dec(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_dex(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_dey(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_eor(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_inc(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_inx(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_iny(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_jmp(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_jsr(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

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
            _ => handle_invalid_addressing_mode(),
        }
    }

    pub(super) fn instruction_ldx(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Immediate => {
                dbg!(low_byte);
                self.x = immediate(low_byte);
                println!("{}", self.x);
                2
            }
            AddressingMode::Zeropage => {
                self.x = zeropage(self, bus, low_byte);
                3
            }
            AddressingMode::ZeropageXIndexed => {
                self.x = zeropage_x(self, bus, low_byte);
                4
            }
            AddressingMode::Absolute => {
                self.x = absolute(self, bus, low_byte, high_byte);
                4
            }
            AddressingMode::AbsoluteYIndexed => {
                let (value, page_changed) = absolute_y(self, bus, low_byte, high_byte);

                self.x = value;

                match page_changed {
                    true => 5,
                    false => 4,
                }
            }
            _ => handle_invalid_addressing_mode(),
        }
    }

    pub(super) fn instruction_ldy(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_lsr(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_nop(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_ora(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_pha(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_php(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_pla(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_plp(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_rol(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_ror(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_rti(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_rts(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_sbc(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_sec(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_sed(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_sei(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        self.processor_status.set_interrupt_disable_flag();
        2
    }

    pub(super) fn instruction_sta(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_stx(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_sty(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_tax(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_tay(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_tsx(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_txa(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_txs(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }

    pub(super) fn instruction_tya(
        &mut self,
        bus: &Bus,
        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        todo!()
    }
}

fn handle_invalid_addressing_mode() -> ! {
    panic!("Invalid addressing mode")
}

#[cfg(test)]
mod test {
    use std::borrow::Borrow;

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

    /// Creates and initializes a system, loads the program into memory, and
    /// begins to execute it. Returns the bus so that we can inspect the state.
    fn simulate_execution(instruction_count: usize, program: Vec<u8>) -> Bus {
        let cartridge = create_cartridge(program);
        let mut bus = Bus::new(cartridge);
        bus.initialize_test_mode(0x8000);

        for _ in 0..instruction_count {
            dbg!(bus.cpu.borrow_mut().program_counter);
            bus.clock_cpu();
        }

        bus
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
        //let cartridge = create_cartridge(vec![]);
        //todo!()
    }

    // not fully tested
    #[test]
    fn test_ldx() {
        // program:
        // LDX #$55
        let instruction_count = 1;
        let program = vec![0xA2, 0x55];

        let bus = simulate_execution(instruction_count, program);

        assert_eq!(bus.cpu.borrow_mut().x, 0x55);
    }
}
