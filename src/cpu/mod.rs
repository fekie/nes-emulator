use crate::ppu::PPU;
use std::cell::RefCell;
use std::rc::Rc;
mod instructions;




#[derive(Debug, Clone, Copy)]
pub struct ProcessorStatus(u8);

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
    ppu: Rc<RefCell<PPU>>,
}

impl CpuMemoryMapper {
    /// Reads the value turned from the address.
    fn read(&self, address: u16) -> u8 {
        match address {
            // Handle the work RAM and the mirrors.
            0x0000..=0x1FFF => self.work_ram.0[address as usize % 0x0800],
            // Handle PPU registers and the mirrors.
            0x2000..=0x3FFF => self.ppu.borrow().registers[((address - 0x2000) % 8) as usize],
            0x4000..=0x4017 => unimplemented!(),

            _ => unimplemented!(),
        }
    }
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