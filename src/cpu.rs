use crate::bus::Bus;
use nes6502::Mapper;
use std::{cell::RefCell, rc::Rc};

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
