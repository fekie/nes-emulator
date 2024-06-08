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

fn handle_invalid_addressing_mode() -> ! {
    panic!("Invalid addressing mode")
}

pub(crate) fn pack_bytes(low_byte: u8, high_byte: u8) -> u16 {
    ((high_byte as u16) << 8) | low_byte as u16
}

pub(crate) fn pack_bytes_wrapped(low_byte: Option<u8>, high_byte: Option<u8>) -> u16 {
    ((high_byte.unwrap() as u16) << 8) | low_byte.unwrap() as u16
}

// rough and dirty addressing shortcuts
pub(crate) fn immediate(low_byte: Option<u8>) -> u8 {
    low_byte.unwrap()
}

pub(crate) fn zeropage(cpu: &CPU, bus: &Bus, low_byte: Option<u8>) -> u8 {
    let address = low_byte.unwrap() as u16;
    cpu.read(bus, address)
}

pub(crate) fn zeropage_x(cpu: &CPU, bus: &Bus, low_byte: Option<u8>) -> u8 {
    let address = low_byte.unwrap().wrapping_add(cpu.x) as u16;
    cpu.read(bus, address)
}

pub(crate) fn absolute(cpu: &CPU, bus: &Bus, low_byte: Option<u8>, high_byte: Option<u8>) -> u8 {
    let address = pack_bytes_wrapped(low_byte, high_byte);
    cpu.read(bus, address)
}

/// Returns the value and whether a page boundary was crossed.
pub(crate) fn absolute_x(
    cpu: &CPU,
    bus: &Bus,
    low_byte: Option<u8>,
    high_byte: Option<u8>,
) -> (u8, bool) {
    let pre_add_address = pack_bytes_wrapped(low_byte, high_byte);
    let address = pre_add_address.wrapping_add(cpu.x as u16);

    let page_changed = low_byte.unwrap().checked_add(cpu.x).is_none();

    (cpu.read(bus, address), page_changed)
}

/// Returns the value and whether a page boundary was crossed.
pub(crate) fn absolute_y(
    cpu: &CPU,
    bus: &Bus,
    low_byte: Option<u8>,
    high_byte: Option<u8>,
) -> (u8, bool) {
    let pre_add_address = pack_bytes_wrapped(low_byte, high_byte);
    let address = pre_add_address.wrapping_add(cpu.y as u16);

    let page_changed = low_byte.unwrap().checked_add(cpu.y).is_none();

    (cpu.read(bus, address), page_changed)
}

pub(crate) fn indirect_x(cpu: &CPU, bus: &Bus, low_byte: Option<u8>) -> u8 {
    let base_address = low_byte.unwrap().wrapping_add(cpu.x) as u16;

    let resolved_address = pack_bytes(cpu.read(bus, base_address), cpu.read(bus, base_address + 1));

    cpu.read(bus, resolved_address)
}

pub(crate) fn indirect_y(cpu: &CPU, bus: &Bus, low_byte: Option<u8>) -> (u8, bool) {
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
