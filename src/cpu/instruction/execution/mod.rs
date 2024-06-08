use super::{AddressingMode, CPU};
use crate::Bus;

// We organize the instructions using modules according to the
// categories used on https://www.nesdev.org/obelisk-6502-guide/instructions.html
mod arithmetic;
mod branches;
mod incr_decr;
mod jumps_calls;
mod load_store;
mod logical;
mod register_transfers;
mod shifts;
mod stack;
mod status_flags;
mod system;

impl CPU {
    /// Sets the zero flag if the given byte is 0.
    fn modify_zero_flag(&mut self, byte: u8) {
        match byte == 0 {
            true => self.processor_status.set_zero_flag(),
            false => self.processor_status.clear_zero_flag(),
        }
    }

    /// Sets the negative flag given byte is negative (in two's compliment)
    fn modify_negative_flag(&mut self, byte: u8) {
        match byte >> 7 != 0 {
            true => self.processor_status.set_negative_flag(),
            false => self.processor_status.clear_negative_flag(),
        }
    }

    /// Sets the overflow flag if an overflow ocurred.
    fn modify_overflow_flag(&mut self, op1: u8, op2: u8) {
        let op1_sign = op1 >> 7;
        let op2_sign = op2 >> 7;

        let result = op1 + op2;

        // If the signs were the same and are different from the result,
        // we have an overflow.
        match op1_sign == op2_sign {
            true => match op1_sign == result >> 7 {
                true => self.processor_status.clear_overflow_flag(),
                false => self.processor_status.set_overflow_flag(),
            },
            false => self.processor_status.clear_overflow_flag(),
        }
    }

    /// Sets the carry flag if a carry out ocurred.
    fn modify_carry_flag(&mut self, op1: u8, op2: u8) {
        match op1.checked_add(op2).is_none() {
            true => self.processor_status.set_carry_flag(),
            false => self.processor_status.clear_carry_flag(),
        }
    }
}

fn handle_invalid_addressing_mode() -> ! {
    panic!("Invalid addressing mode")
}

fn pack_bytes(low_byte: u8, high_byte: u8) -> u16 {
    ((high_byte as u16) << 8) | low_byte as u16
}

fn pack_bytes_wrapped(low_byte: Option<u8>, high_byte: Option<u8>) -> u16 {
    ((high_byte.unwrap() as u16) << 8) | low_byte.unwrap() as u16
}

// rough and dirty addressing shortcuts
fn immediate(low_byte: Option<u8>) -> u8 {
    low_byte.unwrap()
}

fn zeropage(cpu: &CPU, bus: &Bus, low_byte: Option<u8>) -> u8 {
    let address = low_byte.unwrap() as u16;
    cpu.read(bus, address)
}

fn zeropage_x(cpu: &CPU, bus: &Bus, low_byte: Option<u8>) -> u8 {
    let address = low_byte.unwrap().wrapping_add(cpu.x) as u16;
    cpu.read(bus, address)
}

fn absolute(cpu: &CPU, bus: &Bus, low_byte: Option<u8>, high_byte: Option<u8>) -> u8 {
    let address = pack_bytes_wrapped(low_byte, high_byte);
    cpu.read(bus, address)
}

/// Returns the value and whether a page boundary was crossed.
fn absolute_x(cpu: &CPU, bus: &Bus, low_byte: Option<u8>, high_byte: Option<u8>) -> (u8, bool) {
    let pre_add_address = pack_bytes_wrapped(low_byte, high_byte);
    let address = pre_add_address.wrapping_add(cpu.x as u16);

    let page_changed = low_byte.unwrap().checked_add(cpu.x).is_none();

    (cpu.read(bus, address), page_changed)
}

/// Returns the value and whether a page boundary was crossed.
fn absolute_y(cpu: &CPU, bus: &Bus, low_byte: Option<u8>, high_byte: Option<u8>) -> (u8, bool) {
    let pre_add_address = pack_bytes_wrapped(low_byte, high_byte);
    let address = pre_add_address.wrapping_add(cpu.y as u16);

    let page_changed = low_byte.unwrap().checked_add(cpu.y).is_none();

    (cpu.read(bus, address), page_changed)
}

fn indirect_x(cpu: &CPU, bus: &Bus, low_byte: Option<u8>) -> u8 {
    let base_address = low_byte.unwrap().wrapping_add(cpu.x) as u16;

    let resolved_address = pack_bytes(cpu.read(bus, base_address), cpu.read(bus, base_address + 1));

    cpu.read(bus, resolved_address)
}

fn indirect_y(cpu: &CPU, bus: &Bus, low_byte: Option<u8>) -> (u8, bool) {
    let low_base_address = low_byte.unwrap() as u16;
    let high_base_address = low_byte.unwrap().wrapping_add(1) as u16;

    let page_changed = low_base_address > high_base_address;

    let resolved_address = pack_bytes(
        cpu.read(bus, low_base_address),
        cpu.read(bus, high_base_address),
    ) + cpu.y as u16;

    (cpu.read(bus, resolved_address), page_changed)
}

#[cfg(test)]
mod test {
    use std::borrow::Borrow;

    use crate::*;
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
