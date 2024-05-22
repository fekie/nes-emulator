use crate::ppu::PPU;
use std::cell::RefCell;
use std::rc::Rc;
mod instructions;




pub struct ProcessorStatus(u8);

impl ProcessorStatus {
    pub fn new() -> Self {ProcessorStatus(0)}
    pub fn carry_flag(&self) -> bool {return self.0 & 0b0000_0001 != 0}
    pub fn set_carry_flag(&mut self) {let mask = 0b0000_0001;self.0 |= mask;}
    pub fn clear_carry_flag(&mut self) {let mask = 0b1111_1110;self.0 &= mask;}
    pub fn zero_flag(&self) -> bool {return self.0 & 0b0000_0010 != 0}
    pub fn set_zero_flag(&mut self) {let mask = 0b0000_0010;self.0 |= mask;}
    pub fn clear_zero_flag(&mut self) {let mask = 0b1111_1101;self.0 &= mask;self.0 |= mask;}
    pub fn inter_flag(&self) -> bool {return self.0 & 0b0000_0100 != 0}
    pub fn set_inter_flag(&mut self) {let mask = 0b0000_0100;self.0 |= mask;}
    pub fn clear_inter_flag(&mut self) {let mask = 0b1111_1011;self.0 &= mask;self.0 |= mask;}
    pub fn deci_flag(&self) -> bool {return self.0 & 0b0000_1000 != 0}
    pub fn set_deci_flag(&mut self) {let mask = 0b0000_1000;self.0 |= mask;}
    pub fn clear_deci_flag(&mut self) {let mask = 0b1111_0111;self.0 &= mask;}
    pub fn break_flag(&self) -> bool {return self.0 & 0b0001_0000 != 0}
    pub fn set_break_flag(&mut self) {let mask = 0b0001_0000;self.0 |= mask;}
    pub fn clear_break_flag(&mut self) {let mask = 0b1110_1111;self.0 &= mask;}
    pub fn over_flag(&self) -> bool {return self.0 & 0b0100_0000 != 0}
    pub fn set_over_flag(&mut self) {let mask = 0b0100_0000;self.0 |= mask;}
    pub fn clear_over_flag(&mut self) {let mask = 0b1011_1111;self.0 &= mask;}
    pub fn neg_flag(&self) -> bool {return self.0 & 0b1000_0000 != 0}
    pub fn set_neg_flag(&mut self) {let mask = 0b1000_0000;self.0 |= mask;}
    pub fn clear_neg_flag(&mut self) {let mask = 0b0111_1111;self.0 &= mask;}
}

pub struct Cpu {
    accumulator_register: u8,
    x_register: u8,
    y_register: u8,
    stack_pointer: u8,
    program_counter: u16,
    registers: [u8; 6],
    processor_status: ProcessorStatus,
    memory_mapper: CpuMemoryMapper,
}

pub struct CpuMemoryMapper {
    foo: Rc<RefCell<PPU>>,
}



#[test]
fn test_flags() {
    let mut flag_reg = ProcessorStatus::new();
    assert_eq!(flag_reg.carry_flag(), false);
    assert_eq!({flag_reg.set_carry_flag(); flag_reg.carry_flag()}, true);
    assert_eq!({flag_reg.clear_carry_flag(); flag_reg.carry_flag()}, false);
    assert_eq!(flag_reg.zero_flag(), false);
    assert_eq!({flag_reg.set_zero_flag(); flag_reg.zero_flag()}, true);
    assert_eq!({flag_reg.clear_zero_flag(); flag_reg.zero_flag()}, false);
    assert_eq!(flag_reg.inter_flag(), false);
    assert_eq!({flag_reg.set_inter_flag(); flag_reg.inter_flag()}, true);
    assert_eq!({flag_reg.clear_inter_flag(); flag_reg.inter_flag()}, false);
    assert_eq!(flag_reg.deci_flag(), false);
    assert_eq!({flag_reg.set_deci_flag(); flag_reg.deci_flag()}, true);
    assert_eq!({flag_reg.clear_deci_flag(); flag_reg.deci_flag()}, false);
    assert_eq!(flag_reg.break_flag(), false);
    assert_eq!({flag_reg.set_break_flag(); flag_reg.break_flag()}, true);
    assert_eq!({flag_reg.clear_break_flag(); flag_reg.break_flag()}, false);
    assert_eq!(flag_reg.over_flag(), false);
    assert_eq!({flag_reg.set_over_flag(); flag_reg.over_flag()}, true);
    assert_eq!({flag_reg.clear_over_flag(); flag_reg.over_flag()}, false);
    assert_eq!(flag_reg.neg_flag(), false);
    assert_eq!({flag_reg.set_neg_flag(); flag_reg.neg_flag()}, true);
    assert_eq!({flag_reg.clear_neg_flag(); flag_reg.neg_flag()}, false);
}