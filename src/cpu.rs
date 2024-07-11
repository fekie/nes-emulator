use crate::bus::Bus;
use nes6502::{Cpu, Interrupts, Mapper};
use std::{cell::RefCell, rc::Rc};

/// We use a container that holds both interrupt states. Each interrupt state is stored in an
/// `Rc<Refcell<bool>>` internally so that we can use [`InterruptsContainer::share()`] to create a new
/// container with the same references so that other components can modify the interrupt states.
/// As [`InterruptsContainer`] still implements [`Interrupts`], it still meets the generic requirements of [`Cpu`].
///
/// One thing to note of this structure is that the program will panic if more than a single mutable borrow occurs,
/// or if a mutable borrow while immutable borrows exist occurs.
pub struct InterruptsContainer {
    interrupt_state: Rc<RefCell<bool>>,
    non_maskable_interrupt_state: Rc<RefCell<bool>>,
}

impl Interrupts for InterruptsContainer {
    fn interrupt_state(&self) -> bool {
        *self.interrupt_state.borrow()
    }

    fn set_interrupt_state(&mut self, new_state: bool) {
        *self.interrupt_state.borrow_mut() = new_state;
    }

    fn non_maskable_interrupt_state(&self) -> bool {
        *self.non_maskable_interrupt_state.borrow()
    }

    fn set_non_maskable_interrupt_state(&mut self, new_state: bool) {
        *self.non_maskable_interrupt_state.borrow_mut() = new_state;
    }
}

pub struct CpuContainer {
    pub cpu: Cpu<CpuMemoryMapper, InterruptsContainer>,
    pub interrupts: InterruptsContainer,
}

pub struct CpuMemoryMapper {
    ram: [u8; 0x2000],
    bus: Rc<RefCell<Bus>>,
}

impl CpuMemoryMapper {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Self {
            ram: [0; 0x2000],
            bus,
        }
    }
}

impl Mapper for CpuMemoryMapper {
    fn read(&self, address: u16) -> u8 {
        match address {
            // Handle the work RAM and the mirrors.
            0x0000..=0x1FFF => self.ram[address as usize % 0x0800],
            // Handle PPU registers and the mirrors.
            0x2000..=0x3FFF => {
                self.bus.borrow().ppu.0.as_ref().unwrap().borrow().registers
                    [((address - 0x2000) % 8) as usize]
            }
            // Saved for APU
            0x4000..=0x4017 => unimplemented!(),
            // Disabled
            0x4018..=0x401F => unimplemented!(),
            // Route to cartridge mapper
            0x4020..=0xFFFF => self
                .bus
                .borrow()
                .cartridge
                .0
                .as_ref()
                .unwrap()
                .borrow()
                .read(address),
        }
    }

    fn write(&mut self, address: u16, byte: u8) {
        match address {
            // Handle the work RAM and the mirrors.
            0x0000..=0x1FFF => self.ram[address as usize % 0x0800] = byte,
            // Handle PPU registers and the mirrors.
            0x2000..=0x3FFF => {
                self.bus
                    .borrow()
                    .ppu
                    .0
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .registers[((address - 0x2000) % 8) as usize] = byte
            }
            // Saved for APU
            0x4000..=0x4017 => {
                // do nothing for now
            }
            // Disabled
            0x4018..=0x401F => unimplemented!(),
            // Route to cartridge mapper
            0x4020..=0xFFFF => self
                .bus
                .borrow()
                .cartridge
                .0
                .as_ref()
                .unwrap()
                .borrow_mut()
                .write(address, byte),
        }
    }
}
