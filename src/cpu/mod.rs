use crate::cartridge::Cartridge;
use crate::ppu::PPU;
use crate::Mapper;
use std::cell::RefCell;
use std::rc::Rc;

mod instructions;

#[derive(Debug, Clone, Copy, Default)]
pub struct ProcessorStatus(u8);

impl ProcessorStatus {
    pub fn carry_flag(&self) -> bool {
        self.0 & 0b0000_0001 != 0
    }

    pub fn set_carry_flag(&mut self) {
        let mask = 0b0000_0001;
        self.0 |= mask;
    }

    pub fn clear_carry_flag(&mut self) {
        let mask = 0b1111_1110;
        self.0 &= mask;
    }

    //

    pub fn zero_flag(&self) -> bool {
        self.0 & 0b0000_0010 != 0
    }

    pub fn set_zero_flag(&mut self) {
        let mask = 0b0000_0010;
        self.0 |= mask;
    }

    pub fn clear_zero_flag(&mut self) {
        let mask = 0b1111_1101;
        self.0 &= mask;
        self.0 |= mask;
    }

    //

    pub fn interrupt_disable_flag(&self) -> bool {
        self.0 & 0b0000_0100 != 0
    }

    pub fn set_interrupt_disable_flag(&mut self) {
        let mask = 0b0000_0100;
        self.0 |= mask;
    }

    pub fn clear_interrupt_disable_flag(&mut self) {
        let mask = 0b1111_1011;
        self.0 &= mask;
        self.0 |= mask;
    }

    //

    pub fn decimal_flag(&self) -> bool {
        self.0 & 0b0000_1000 != 0
    }

    pub fn set_decimal_flag(&mut self) {
        let mask = 0b0000_1000;
        self.0 |= mask;
    }

    pub fn clear_decimal_flag(&mut self) {
        let mask = 0b1111_0111;
        self.0 &= mask;
    }

    //

    pub fn break_flag(&self) -> bool {
        self.0 & 0b0001_0000 != 0
    }

    pub fn set_break_flag(&mut self) {
        let mask = 0b0001_0000;
        self.0 |= mask;
    }

    pub fn clear_break_flag(&mut self) {
        let mask = 0b1110_1111;
        self.0 &= mask;
    }

    //

    pub fn overflow_flag(&self) -> bool {
        self.0 & 0b0100_0000 != 0
    }

    pub fn set_overflow_flag(&mut self) {
        let mask = 0b0100_0000;
        self.0 |= mask;
    }

    pub fn clear_overflow_flag(&mut self) {
        let mask = 0b1011_1111;
        self.0 &= mask;
    }

    //

    pub fn negative_flag(&self) -> bool {
        self.0 & 0b1000_0000 != 0
    }

    pub fn set_negative_flag(&mut self) {
        let mask = 0b1000_0000;
        self.0 |= mask;
    }

    pub fn clear_negative_flag(&mut self) {
        let mask = 0b0111_1111;
        self.0 &= mask;
    }
}

pub struct CPU {
    accumulator_register: u8,
    x_register: u8,
    y_register: u8,
    stack_pointer: u8,
    program_counter: u16,
    registers: [u8; 6],
    processor_status: ProcessorStatus,
    memory_mapper: CpuMemoryMapper,
}

impl CPU {
    pub fn new() -> Self {
        todo!()
    }

    /// Runs a full instruction cycle. Returns the amount of
    /// machine cycles taken.
    pub fn cycle(&mut self, ppu: &mut PPU) -> u8 {
        todo!()
    }
}

// We use 2KB of work ram.
pub struct WorkRAM([u8; 0x800]);

/// Memory Map:
///
/// | Address range |  Size  |                                  Device                                  |   |   |
/// |:-------------:|:------:|:------------------------------------------------------------------------:|---|---|
/// | $0000–$07FF   | $0800  | 2 KB internal RAM                                                        |   |   |
/// | $0800–$0FFF   | $0800  | Mirror  of $0000–$07FF                                                   |   |   |
/// | $1000–$17FF   | $0800  | Mirror  of $0000–$07FF                                                   |   |   |
/// | $1800–$1FFF   | $0800  | Mirror  of $0000–$07FF                                                   |   |   |
/// | $2000–$2007   | $0008  | NES PPU registers                                                        |   |   |
/// | $2008–$3FFF   | $1FF8  | Mirrors of $2000–$2007 (repeats every 8 bytes)                           |   |   |
/// | $4000–$4017   | $0018  | NES APU and I/O registers                                                |   |   |
/// | $4018–$401F   | $0008  | APU and I/O functionality that is normally disabled. See CPU Test Mode.  |   |   |
/// | $4020–$FFFF   | $BFE0  | Unmapped. Available for cartridge use.                                   |   |   |
/// | *$6000-$7FFF  | $2000  | Usually cartridge RAM, when present.                                     |   |   |
/// | *$8000-$FFFF  | $8000  | Usually cartridge ROM and mapper registers.                              |   |   |
pub struct CpuMemoryMapper {
    work_ram: WorkRAM,
    cartridge: Rc<RefCell<Cartridge>>,
    ppu: Rc<RefCell<PPU>>,
}

impl Mapper for CpuMemoryMapper {
    fn read(&self, address: u16) -> u8 {
        match address {
            // Handle the work RAM and the mirrors.
            0x0000..=0x1FFF => self.work_ram.0[address as usize % 0x0800],
            // Handle PPU registers and the mirrors.
            0x2000..=0x3FFF => self.ppu.borrow().registers[((address - 0x2000) % 8) as usize],
            // Saved for APU
            0x4000..=0x4017 => unimplemented!(),
            // Disabled
            0x4018..=0x401F => unimplemented!(),
            // Route to cartridge mapper
            0x4020..=0xFFFF => self.cartridge.borrow().read(address),
        }
    }

    fn write(&mut self, address: u16, byte: u8) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_carry() {
        let mut flag_reg = ProcessorStatus::default();
        assert!(!flag_reg.carry_flag());

        flag_reg.set_carry_flag();
        assert!(flag_reg.carry_flag());

        flag_reg.clear_carry_flag();
        assert!(!flag_reg.carry_flag());
    }

    #[test]
    fn test_zero() {
        let mut flag_reg = ProcessorStatus::default();
        assert!(!flag_reg.zero_flag());

        flag_reg.set_zero_flag();
        assert!(flag_reg.zero_flag());

        flag_reg.clear_zero_flag();
        assert!(!flag_reg.zero_flag());
    }

    #[test]
    fn test_interrupt_disable() {
        let mut flag_reg = ProcessorStatus::default();
        assert!(!flag_reg.interrupt_disable_flag());

        flag_reg.set_interrupt_disable_flag();
        assert!(flag_reg.interrupt_disable_flag());

        flag_reg.clear_interrupt_disable_flag();
        assert!(!flag_reg.interrupt_disable_flag());
    }

    #[test]
    fn test_decimal() {
        let mut flag_reg = ProcessorStatus::default();
        assert!(!flag_reg.decimal_flag());

        flag_reg.set_decimal_flag();
        assert!(flag_reg.decimal_flag());

        flag_reg.clear_decimal_flag();
        assert!(!flag_reg.decimal_flag());
    }

    #[test]
    fn test_overflow() {
        let mut flag_reg = ProcessorStatus::default();
        assert!(!flag_reg.overflow_flag());

        flag_reg.set_overflow_flag();
        assert!(flag_reg.overflow_flag());

        flag_reg.clear_overflow_flag();
        assert!(!flag_reg.overflow_flag());
    }

    #[test]
    fn test_neg() {
        let mut flag_reg = ProcessorStatus::default();
        assert!(!flag_reg.negative_flag());

        flag_reg.set_negative_flag();
        assert!(flag_reg.negative_flag());

        flag_reg.clear_negative_flag();
        assert!(!flag_reg.negative_flag());
    }
}
