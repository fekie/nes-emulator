use crate::apu::APU;
use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::ppu::PPU;
use std::cell::RefCell;
use std::rc::Rc;

const PPU_CLOCK_DIVISOR: u8 = 4;

#[derive(Copy, Clone, PartialEq)]
pub enum Request {
    Active,
    Inactive,
}

/// Wraps over internally mutatable interrupt states.
pub struct Interrupts {
    pub interrupt: Rc<RefCell<Request>>,
    pub non_maskable_interrupt: Rc<RefCell<Request>>,
    pub initialized: bool,
}

impl Interrupts {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            interrupt: Rc::new(RefCell::new(Request::Inactive)),
            non_maskable_interrupt: Rc::new(RefCell::new(Request::Inactive)),
            initialized: false,
        }
    }

    /// Initialize the APU.
    pub fn initialize(&mut self) {
        // already initialized in new()
        self.initialized = true;
    }

    /// Returns the state of initialization.
    pub fn initialized(&self) -> bool {
        self.initialized
    }

    pub fn interrupt_state(&self) -> Request {
        *self.interrupt.borrow()
    }

    pub fn set_interrupt_state(&self, new_state: Request) {
        *self.interrupt.borrow_mut() = new_state;
    }

    pub fn non_maskable_interrupt_state(&self) -> Request {
        *self.non_maskable_interrupt.borrow()
    }

    pub fn set_non_maskable_interrupt_state(&self, new_state: Request) {
        *self.non_maskable_interrupt.borrow_mut() = new_state;
    }
}

pub struct Bus {
    pub cpu: Rc<RefCell<CPU>>,
    pub ppu: Rc<RefCell<PPU>>,
    pub apu: Rc<RefCell<APU>>,
    // todo: add APU later
    // Cartridge can sometimes interact with the cpu and apu
    // and it needs to be clocked just like
    pub cartridge: Rc<RefCell<Cartridge>>,
    pub interrupts: Rc<RefCell<Interrupts>>,
    /// A counter for our current cycles that wraps around. We mostly need it
    /// to be able to keep the PPU in sync as it needs to tick on every
    /// 4th machine cycle.
    pub current_machine_cycles: u8,
    pub initialized: bool,
}

impl Bus {
    /// Creates the bus and the devices on it but does not initialize them.
    pub fn new(cartridge: Cartridge) -> Self {
        let cpu = Rc::new(RefCell::new(CPU::new()));
        let ppu = Rc::new(RefCell::new(PPU::new()));
        let apu = Rc::new(RefCell::new(APU::new()));
        let cartridge = Rc::new(RefCell::new(cartridge));
        let interrupts = Rc::new(RefCell::new(Interrupts::new()));
        let current_machine_cycles = 0;
        let initialized = false;

        Self {
            cpu,
            ppu,
            apu,
            cartridge,
            interrupts,
            current_machine_cycles,
            initialized,
        }
    }

    /// Initializes the CPU but starts the PC at an arbitrary value.
    /// A common value to use would be 0x8000 as this is where program rom starts.
    #[cfg(test)]
    pub fn initialize_test_mode(&mut self, program_counter: u16) {
        self.initialize();
        self.cpu.borrow_mut().program_counter = program_counter;
    }

    pub fn initialize(&mut self) {
        // We go ahead and explicitly initialize everything,
        // even if under the hood it doesn't change any values.
        // This allows us to be sure that we're initializing anything,
        // and allows for us to be able to add initialization behavior
        // later if needed.
        self.cpu.borrow_mut().initialize(self);
        self.ppu.borrow_mut().initialize();
        self.apu.borrow_mut().initialize();
        self.cartridge.borrow_mut().initialize();
        self.interrupts.borrow_mut().initialize();

        assert!(self.cpu.borrow().initialized());
        assert!(self.ppu.borrow().initialized());
        assert!(self.apu.borrow().initialized());
        assert!(self.cartridge.borrow().initialized());
        assert!(self.interrupts.borrow().initialized());

        self.initialized = true;
    }
}

impl Bus {
    /// Runs one cycle of the cpu and returns how many cpu cycles it took.
    /// CPU cycles can be converted to clock cycles by multiplying by 12.
    pub fn clock_cpu(&self) -> u8 {
        self.cpu.borrow_mut().cycle(self)
    }

    /// Clocks all the devices on the bus except for the cpu, such as the PPU, APU, and Cartridge
    pub fn clock_bus(&mut self, additional_machine_cycles: u8) {
        for _ in 0..additional_machine_cycles {
            // clock ppu every 4 cycles
            if self.current_machine_cycles % PPU_CLOCK_DIVISOR == 0 {
                self.ppu.borrow_mut().tick(&self.cartridge.borrow());
            }

            // clock cartridge every tick
            self.cartridge.borrow_mut().clock(&self.interrupts.borrow());

            self.current_machine_cycles = self.current_machine_cycles.wrapping_add(1);
        }
    }
}
