use super::{AddressingMode, CPU};
use crate::Bus;

// We organize the instructions using modules according to the
// categories used on https://www.nesdev.org/obelisk-6502-guide/instructions.html
mod arithmetic;
mod branches; // completed
mod incr_decr; // completed
mod jumps_calls; // completed
mod load_store; // completed
mod logical; // completed
mod register_transfers; // completed
mod shifts; // completed
mod stack; // completed
mod status_flags; // completed
mod system; // completed

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

    // Pushes a value from the stack
    fn push(&mut self, bus: &Bus, byte: u8) {
        self.write(bus, 0x0100 | self.stack_pointer as u16, byte);

        self.stack_pointer = match self.stack_pointer.checked_sub(1) {
            Some(x) => x,
            None => panic!("CPU stack overflow"),
        };
    }

    // Pops a value from the stack
    fn pop(&mut self, bus: &Bus) -> u8 {
        self.stack_pointer = match self.stack_pointer.checked_add(1) {
            Some(x) => x,
            None => panic!("CPU stack underflow"),
        };

        let byte = self.read(bus, 0x0100 | self.stack_pointer as u16);

        byte
    }
}

fn handle_invalid_addressing_mode() -> ! {
    panic!("Invalid addressing mode")
}

fn unpack_bytes(packed: u16) -> (u8, u8) {
    ((packed & 0xFF) as u8, ((packed >> 8) & 0xFF) as u8)
}

fn pack_bytes(low_byte: u8, high_byte: u8) -> u16 {
    ((high_byte as u16) << 8) | low_byte as u16
}

fn pack_bytes_wrapped(low_byte: Option<u8>, high_byte: Option<u8>) -> u16 {
    ((high_byte.unwrap() as u16) << 8) | low_byte.unwrap() as u16
}

fn twos_compliment_to_signed(value: u8) -> i8 {
    match (value >> 7) != 0 {
        true => {
            let negative = (!value).wrapping_add(1);

            // we check for the case that we had -128, which wouldnt be converted
            match (negative == 0b1000_0000) {
                true => -128,
                false => -(negative as i8),
            }
        }
        false => value as i8,
    }
}

// rough and dirty addressing shortcuts
fn immediate_read(low_byte: Option<u8>) -> u8 {
    low_byte.unwrap()
}

fn zeropage_read(cpu: &CPU, bus: &Bus, low_byte: Option<u8>) -> u8 {
    let address = low_byte.unwrap() as u16;
    cpu.read(bus, address)
}

// value is the value written to memory
fn zeropage_write(cpu: &mut CPU, bus: &Bus, low_byte: Option<u8>, value: u8) {
    let address = low_byte.unwrap() as u16;
    cpu.write(bus, address, value);
}

fn zeropage_x_read(cpu: &CPU, bus: &Bus, low_byte: Option<u8>) -> u8 {
    let address = low_byte.unwrap().wrapping_add(cpu.x) as u16;
    cpu.read(bus, address)
}

fn zeropage_x_write(cpu: &mut CPU, bus: &Bus, low_byte: Option<u8>, value: u8) {
    let address = low_byte.unwrap().wrapping_add(cpu.x) as u16;
    cpu.write(bus, address, value);
}

fn zeropage_y_read(cpu: &CPU, bus: &Bus, low_byte: Option<u8>) -> u8 {
    let address = low_byte.unwrap().wrapping_add(cpu.y) as u16;
    cpu.read(bus, address)
}

fn zeropage_y_write(cpu: &mut CPU, bus: &Bus, low_byte: Option<u8>, value: u8) {
    let address = low_byte.unwrap().wrapping_add(cpu.y) as u16;
    cpu.write(bus, address, value);
}

fn absolute_read(cpu: &CPU, bus: &Bus, low_byte: Option<u8>, high_byte: Option<u8>) -> u8 {
    let address = pack_bytes_wrapped(low_byte, high_byte);
    cpu.read(bus, address)
}

fn absolute_write(
    cpu: &mut CPU,
    bus: &Bus,
    low_byte: Option<u8>,
    high_byte: Option<u8>,
    value: u8,
) {
    let address = pack_bytes_wrapped(low_byte, high_byte);
    cpu.write(bus, address, value);
}

/// Returns the value and whether a page boundary was crossed.
fn absolute_x_read(
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

fn absolute_x_write(
    cpu: &mut CPU,
    bus: &Bus,
    low_byte: Option<u8>,
    high_byte: Option<u8>,
    value: u8,
) {
    let pre_add_address = pack_bytes_wrapped(low_byte, high_byte);
    let address = pre_add_address.wrapping_add(cpu.x as u16);
    cpu.write(bus, address, value);
}

/// Returns the value and whether a page boundary was crossed.
fn absolute_y_read(
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

fn absolute_y_write(
    cpu: &mut CPU,
    bus: &Bus,
    low_byte: Option<u8>,
    high_byte: Option<u8>,
    value: u8,
) {
    let pre_add_address = pack_bytes_wrapped(low_byte, high_byte);
    let address = pre_add_address.wrapping_add(cpu.y as u16);
    cpu.write(bus, address, value);
}

fn indirect_x_read(cpu: &CPU, bus: &Bus, low_byte: Option<u8>) -> u8 {
    let base_address = low_byte.unwrap().wrapping_add(cpu.x) as u16;

    let resolved_address = pack_bytes(cpu.read(bus, base_address), cpu.read(bus, base_address + 1));

    cpu.read(bus, resolved_address)
}

fn indirect_x_write(cpu: &mut CPU, bus: &Bus, low_byte: Option<u8>, value: u8) {
    let base_address = low_byte.unwrap().wrapping_add(cpu.x) as u16;

    let resolved_address = pack_bytes(cpu.read(bus, base_address), cpu.read(bus, base_address + 1));

    cpu.write(bus, resolved_address, value);
}

fn indirect_y_read(cpu: &CPU, bus: &Bus, low_byte: Option<u8>) -> (u8, bool) {
    let low_base_address = low_byte.unwrap() as u16;
    let high_base_address = low_byte.unwrap().wrapping_add(1) as u16;

    let page_changed = low_base_address > high_base_address;

    let resolved_address = pack_bytes(
        cpu.read(bus, low_base_address),
        cpu.read(bus, high_base_address),
    ) + cpu.y as u16;

    (cpu.read(bus, resolved_address), page_changed)
}

fn indirect_y_write(cpu: &mut CPU, bus: &Bus, low_byte: Option<u8>, value: u8) {
    let low_base_address = low_byte.unwrap() as u16;
    let high_base_address = low_byte.unwrap().wrapping_add(1) as u16;

    let resolved_address = pack_bytes(
        cpu.read(bus, low_base_address),
        cpu.read(bus, high_base_address),
    ) + cpu.y as u16;

    cpu.write(bus, resolved_address, value);
}

#[cfg(test)]
mod test {
    use std::borrow::Borrow;

    use cpu::instruction::execution::twos_compliment_to_signed;

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
    fn test_twos_compliment_to_signed() {
        let neg_52 = 0b11001100;
        assert_eq!(-52, twos_compliment_to_signed(neg_52));

        let pos_52 = 0b00110100;
        assert_eq!(52, twos_compliment_to_signed(pos_52));

        let zero = 0;
        assert_eq!(0, twos_compliment_to_signed(zero));

        let neg_128 = 0b1000_0000;
        assert_eq!(-128, twos_compliment_to_signed(neg_128));
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
