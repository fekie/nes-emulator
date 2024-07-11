use crate::apu::Apu;
use crate::cartridge::{self, Cartridge};
//use crate::cpu::{CpuMemoryMapper, InterruptsContainer};
use crate::ines::Ines;
use crate::ppu::Ppu;
use nes6502::Cpu;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

const PPU_CLOCK_DIVISOR: u8 = 4;

/* #[derive(Copy, Clone, PartialEq, Debug)]
pub enum Request {
    Active,
    Inactive,
} */

/* /// The wrapper over a component attached to the bus.
///
/// This allows us to create a Bus statically with no components,
/// and then populating the Bus with components so that the components can
/// have smart pointers to the Bus.
///
/// Putting it in a struct also allows us to make a cleaner interface
/// as we can assume that components will not be able to be modified or ran
/// until after the Bus has been populated. Otherwise we get to deal with a
/// Option<Rc<RefCell<T>> each time.
#[derive(Debug)]
pub struct Component<T>(pub Option<Rc<RefCell<T>>>);

impl<T> Component<T> {
    pub fn empty() -> Self {
        Self(None)
    }

    pub fn populate(&mut self, attachment: T) {
        self.0 = Some(Rc::new(RefCell::new(attachment)))
    }
} */

/* /// Wraps over internally mutatable interrupt states.
#[derive(Debug)]
pub struct Interrupts {
    pub interrupt: Request,
    pub non_maskable_interrupt: Request,
    pub initialized: bool,
}

impl Interrupts {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            interrupt: Request::Inactive,
            non_maskable_interrupt: Request::Inactive,
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
        self.interrupt
    }

    pub fn set_interrupt_state(&mut self, new_state: Request) {
        self.interrupt = new_state;
    }

    pub fn non_maskable_interrupt_state(&self) -> Request {
        *self.non_maskable_interrupt.borrow()
    }

    pub fn set_non_maskable_interrupt_state(&mut self, new_state: Request) {
        self.non_maskable_interrupt = new_state;
    }
} */

/* pub type BusPointer = Rc<RefCell<Bus>>;

pub struct Bus {
    pub cpu: Option<RefCell<Cpu<CpuMemoryMapper, InterruptsContainer>>>,
    pub ppu: Option<RefCell<Ppu>>,
    pub apu: Option<RefCell<Apu>>,
    // todo: add APU later
    // Cartridge can sometimes interact with the cpu and apu
    // and it needs to be clocked just like
    pub cartridge: Option<RefCell<Cartridge>>,
    /// A counter for our current cycles that wraps around. We mostly need it
    /// to be able to keep the PPU in sync as it needs to tick on every
    /// 4th machine cycle.
    pub current_machine_cycles: u8,
    pub initialized: bool,
}

impl Bus {
    /// Creates an empty bus with no components.
    pub fn empty() -> Self {
        Self {
            cpu: None,
            ppu: None,
            apu: None,
            cartridge: None,
            current_machine_cycles: 0,
            initialized: false,
        }
    }

    /// Initialize bus with components, and then attach a reference to the bus in each component.
    pub fn initialize(
        &mut self,
        cpu: RefCell<Cpu<CpuMemoryMapper, InterruptsContainer>>,
        ppu: RefCell<Ppu>,
        apu: RefCell<Apu>,
        cartridge: RefCell<Cartridge>,
    ) {
        self.cpu = Some(cpu);
        self.ppu = Some(ppu);
        self.apu = Some(apu);
        self.cartridge = Some(cartridge);

        self.initialized = true;

        /* let cpu = Cpu::new(CpuMemoryMapper::new(Rc::clone(&bus)));
        let ppu = Ppu::new(Rc::clone(&bus));
        let apu = Apu::new(Rc::clone(&bus));
        let cartridge = Cartridge::new(rom, Rc::clone(&bus));
        let interrupts = Interrupts::new();

        bus.borrow_mut().cpu.populate(cpu);
        bus.borrow_mut().ppu.populate(ppu);
        bus.borrow_mut().apu.populate(apu);
        bus.borrow_mut().cartridge.populate(cartridge);
        bus.borrow_mut().interrupts.populate(interrupts); */

        // We go ahead and explicitly initialize everything,
        // even if under the hood it doesn't change any values.
        // This allows us to be sure that we're initializing anything,
        // and allows for us to be able to add initialization behavior
        // later if needed.
        /* bus.borrow_mut()
            .cpu
            .0
            .as_ref()
            .unwrap()
            .borrow_mut()
            .initialize();

        bus.borrow_mut()
            .ppu
            .0
            .as_ref()
            .unwrap()
            .borrow_mut()
            .initialize();

        bus.borrow_mut()
            .apu
            .0
            .as_ref()
            .unwrap()
            .borrow_mut()
            .initialize();

        bus.borrow_mut()
            .cartridge
            .0
            .as_ref()
            .unwrap()
            .borrow_mut()
            .initialize();

        bus.borrow_mut()
            .interrupts
            .0
            .as_ref()
            .unwrap()
            .borrow_mut()
            .initialize(); */

        /* let foo = self.cpu.0.as_ref().unwrap();
        let bar = foo.borrow().initialized();

        let cpu_initialized = self.cpu.0.as_ref().unwrap().borrow();

        assert!(self.cpu.0.as_ref().unwrap().initialized());

        assert!(self.ppu.borrow().initialized());
        assert!(self.apu.borrow().initialized());
        assert!(self.cartridge.borrow().initialized());
        assert!(self.interrupts.borrow().initialized()); */

        /* bus.borrow_mut().initialized = true;

        bus */
    }
} */

/* impl Bus {
    /// Runs one cycle of the cpu and returns how many cpu cycles it took.
    /// CPU cycles can be converted to clock cycles by multiplying by 12.
    pub fn clock_cpu(&self) -> u8 {
        self.cpu.0.as_ref().unwrap().borrow_mut().cycle()
    }

    /// Clocks all the devices on the bus except for the cpu, such as the PPU, APU, and Cartridge
    pub fn clock_bus(&mut self, additional_machine_cycles: u8) {
        for _ in 0..additional_machine_cycles {
            // clock ppu every 4 cycles
            if self.current_machine_cycles % PPU_CLOCK_DIVISOR == 0 {
                self.ppu.0.as_ref().unwrap().borrow_mut().tick();
            }

            // clock cartridge every tick
            self.cartridge.0.as_ref().unwrap().borrow_mut().clock();

            self.current_machine_cycles = self.current_machine_cycles.wrapping_add(1);
        }
    }
}
 */
