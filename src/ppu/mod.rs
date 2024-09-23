use crate::{cpu::CpuContainer, Pixels};
use std::{cell::RefCell, rc::Rc};

/// Uses the [RP2C04-0004](https://www.nesdev.org/wiki/PPU_palettes#RP2C04-0004) palette.
#[allow(clippy::zero_prefixed_literal)]
const PALETTE: [u16; 64] = [
    430, 326, 044, 660, 000, 755, 014, 630, 555, 310, 070, 003, 764, 770, 040, 572, 737, 200, 027,
    747, 000, 222, 510, 740, 653, 053, 447, 140, 403, 000, 473, 357, 503, 031, 420, 006, 407, 507,
    333, 704, 022, 666, 036, 020, 111, 773, 444, 707, 757, 777, 320, 700, 760, 276, 777, 467, 000,
    750, 637, 567, 360, 657, 077, 120,
];

#[allow(clippy::upper_case_acronyms)]
pub struct Ppu {
    pub registers: [u8; 8],
    pub cpu: Option<Rc<RefCell<CpuContainer>>>,
    pub initialized: bool,
}

impl Ppu {
    /// Creates the PPU but does not initialize it. Please run [`Initialize`] to
    /// initialize the PPU.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            registers: [0; 8],
            cpu: None,
            initialized: false,
        }
    }

    /// Initialize the PPU.
    pub fn initialize(&mut self, cpu: Rc<RefCell<CpuContainer>>) {
        self.cpu = Some(cpu);
        self.initialized = true;
    }

    /// Returns the state of initialization.
    pub fn initialized(&self) -> bool {
        self.initialized
    }

    pub fn clock(&mut self, pixels: &Pixels) {
        // do nothing right now
    }
}
