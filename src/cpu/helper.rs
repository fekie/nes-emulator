//! A module for quick and dirty helper functions.
//! Not the prettiest, but it's the better option
use super::CPU;
use crate::Bus;

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
