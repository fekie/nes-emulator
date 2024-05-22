use crate::ppu::PPU;
use std::cell::RefCell;
use std::rc::Rc;

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

pub struct CpuMemoryMapper {
    foo: Rc<RefCell<PPU>>,
}
