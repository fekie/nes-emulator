use crate::{apu::Apu, cartridge::Cartridge, ppu::Ppu};
use nes6502::{Cpu, Interrupts, Mapper};
use std::{cell::RefCell, rc::Rc};

const PPUCTRL: u16 = 0x2000;
const PPUMASK: u16 = 0x2001;
const PPUSTATUS: u16 = 0x2002;
const OAMADDR: u16 = 0x2003;
const OAMDATA: u16 = 0x2004;
const PPUSCROLL: u16 = 0x2005;
const PPUADDR: u16 = 0x2006;
const PPUDATA: u16 = 0x2007;
const OAMDMA: u16 = 0x4014;

/// We use a container that holds both interrupt states. Each interrupt state is stored in an
/// `Rc<Refcell<bool>>` internally so that we can use [`InterruptsContainer::share()`] to create a new
/// container with the same references so that other components can modify the interrupt states.
/// As [`InterruptsContainer`] still implements [`Interrupts`], it still meets the generic requirements of [`Cpu`].
///
/// One thing to note of this structure is that the program will panic if more than a single mutable borrow occurs,
/// or if a mutable borrow while immutable borrows exist occurs.
#[derive(Default, Debug)]
pub struct InterruptsContainer {
    interrupt_state: bool,
    non_maskable_interrupt_state: bool,
}

impl InterruptsContainer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Interrupts for InterruptsContainer {
    fn interrupt_state(&self) -> bool {
        self.interrupt_state
    }

    fn set_interrupt_state(&mut self, new_state: bool) {
        self.interrupt_state = new_state;
    }

    fn non_maskable_interrupt_state(&self) -> bool {
        self.non_maskable_interrupt_state
    }

    fn set_non_maskable_interrupt_state(&mut self, new_state: bool) {
        self.non_maskable_interrupt_state = new_state;
    }
}

/// A container that holds the CPU + Interrupts. Interrupts can be accessed by using `Cpu.interrupts`.
pub struct CpuContainer(pub Cpu<CpuMemoryMapper, InterruptsContainer>);

#[derive(Clone, Debug)]
pub struct CpuDebugSnapshot {
    pub instruction_address: u16,
    pub program_counter: u16,
    pub accumulator: u8,
    pub x: u8,
    pub y: u8,
    pub stack_pointer: u8,
    pub processor_status: u8,
    pub current_instruction: String,
    pub last_instruction_success: bool,
    pub total_cpu_cycles: u64,
    pub instruction_count: u64,
}

impl Default for CpuDebugSnapshot {
    fn default() -> Self {
        Self {
            instruction_address: 0,
            program_counter: 0,
            accumulator: 0,
            x: 0,
            y: 0,
            stack_pointer: 0,
            processor_status: 0,
            current_instruction: "Not started".to_string(),
            last_instruction_success: false,
            total_cpu_cycles: 0,
            instruction_count: 0,
        }
    }
}

impl CpuContainer {
    pub fn new() -> Self {
        let memory_mapper = CpuMemoryMapper::new();
        let interrupts_container = InterruptsContainer::new();
        CpuContainer(Cpu::new(memory_mapper, interrupts_container))
    }

    /// Initializes the registers, memory, and interrupts. It also pairs other components to
    /// the CPU (such as PPU and Cartridge).
    pub fn initialize(
        &mut self,
        ppu: Rc<RefCell<Ppu>>,
        apu: Rc<RefCell<Apu>>,
        cartridge: Rc<RefCell<Cartridge>>,
    ) {
        self.0.memory_mapper.initialize(ppu, apu, cartridge);
        self.0.initialize();
    }

    pub fn initialized(&self) -> bool {
        self.0.initialized() && self.0.memory_mapper.initialized()
    }

    /// Runs a full instruction cycle. Returns the amount of
    /// cpu cycles taken.
    pub fn cycle(&mut self) -> u8 {
        self.0.cycle()
    }

    pub fn cycle_debug(&mut self, debug_snapshot: &mut CpuDebugSnapshot) -> u8 {
        let instruction_address = self.0.program_counter;
        let (cycles, success, instruction) = self.0.cycle_debug();

        debug_snapshot.instruction_address = instruction_address;
        debug_snapshot.program_counter = self.0.program_counter;
        debug_snapshot.accumulator = self.0.accumulator;
        debug_snapshot.x = self.0.x;
        debug_snapshot.y = self.0.y;
        debug_snapshot.stack_pointer = self.0.stack_pointer;
        debug_snapshot.processor_status = self.0.processor_status.0;
        debug_snapshot.current_instruction = match instruction {
            Some(instruction) => format!("{instruction:?}"),
            None => "Invalid opcode".to_string(),
        };
        debug_snapshot.last_instruction_success = success;
        debug_snapshot.total_cpu_cycles += cycles as u64;
        debug_snapshot.instruction_count += 1;

        cycles
    }
}

pub struct CpuMemoryMapper {
    ram: [u8; 0x2000],
    ppu: Option<Rc<RefCell<Ppu>>>,
    apu: Option<Rc<RefCell<Apu>>>,
    cartridge: Option<Rc<RefCell<Cartridge>>>,
    initialized: bool,
}

impl CpuMemoryMapper {
    fn new() -> Self {
        Self {
            ram: [0; 0x2000],
            ppu: None,
            apu: None,
            cartridge: None,
            initialized: false,
        }
    }

    fn initialize(
        &mut self,
        ppu: Rc<RefCell<Ppu>>,
        apu: Rc<RefCell<Apu>>,
        cartridge: Rc<RefCell<Cartridge>>,
    ) {
        self.ppu = Some(ppu);
        self.apu = Some(apu);
        self.cartridge = Some(cartridge);

        self.initialized = true;
    }

    fn initialized(&self) -> bool {
        self.initialized
    }
}

impl Mapper for CpuMemoryMapper {
    fn read(&self, address: u16) -> u8 {
        match address {
            // Handle the work RAM and the mirrors.
            0x0000..=0x1FFF => self.ram[address as usize % 0x0800],
            // Handle PPU registers and the mirrors.
            0x2000..=0x3FFF => {
                let adjusted_address = 0x2000 + ((address - 0x2000) % 8);

                match adjusted_address {
                    PPUSTATUS => self.ppu.as_ref().unwrap().borrow().read_ppu_status(),
                    OAMDATA => self.ppu.as_ref().unwrap().borrow().read_oam_data(),
                    _ => panic!("Illegal PPU Operation"),
                }
            }
            // Saved for APU
            0x4000..=0x4017 => unimplemented!(),
            // Disabled
            0x4018..=0x401F => unimplemented!(),
            // Route to cartridge mapper
            0x4020..=0xFFFF => self.cartridge.as_ref().unwrap().borrow().read(address),
        }
    }

    fn write(&mut self, address: u16, byte: u8) {
        match address {
            // Handle the work RAM and the mirrors.
            0x0000..=0x1FFF => self.ram[address as usize % 0x0800] = byte,
            // Handle PPU registers and the mirrors.
            0x2000..=0x3FFF => {
                let adjusted_address = 0x2000 + ((address - 0x2000) % 8);

                match adjusted_address {
                    PPUCTRL => self.ppu.as_ref().unwrap().borrow_mut().write_ppu_ctrl(byte),
                    PPUMASK => self.ppu.as_ref().unwrap().borrow_mut().write_ppu_mask(),
                    OAMADDR => self.ppu.as_ref().unwrap().borrow_mut().write_oam_addr(),
                    OAMDATA => self.ppu.as_ref().unwrap().borrow_mut().write_oam_data(),
                    PPUSCROLL => self.ppu.as_ref().unwrap().borrow_mut().write_ppu_scroll(),
                    PPUADDR => self.ppu.as_ref().unwrap().borrow_mut().write_ppu_addr(),
                    PPUDATA => self.ppu.as_ref().unwrap().borrow_mut().write_ppu_data(),
                    _ => panic!("Illegal PPU Operation"),
                }
            }
            // Saved for APU
            0x4000..=0x4017 => match address {
                OAMDMA => {
                    unimplemented!()
                }
                _ => {
                    // do nothing for now
                }
            },
            // Disabled
            0x4018..=0x401F => unimplemented!(),
            // Route to cartridge mapper
            0x4020..=0xFFFF => self
                .cartridge
                .as_ref()
                .unwrap()
                .borrow_mut()
                .write(address, byte),
        }
    }
}
