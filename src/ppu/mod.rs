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

/// Holds the status of the ppu for PPUCTRL
pub struct PpuStatus(u8);

impl PpuStatus {
    pub fn new() -> Self {
        Self(0)
    }

    /// Returns a code from 0-3
    /// The codes corresponding to base addresses are as follows:
    /// 0 = $2000
    /// 1 = $2400
    /// 2 = $2800
    /// 3 = $2C00
    pub fn base_nametable_address_code(&self) -> u8 {
        self.0 & 0b0000_0011
    }

    /// VRAM address increment per CPU read/write of PPUDATA
    /// (0: add 1, going across; 1: add 32, going down)
    pub fn vram_address_increment(&self) -> u8 {
        (self.0 & 0b0000_0100) >> 2
    }

    /// Sprite pattern table address for 8x8 sprites
    /// (0: $0000; 1: $1000; ignored in 8x16 mode)
    pub fn sprite_pattern_table_address(&self) -> u8 {
        (self.0 & 0b0000_1000) >> 3
    }

    /// Background pattern table address (0: $0000; 1: $1000)
    pub fn background_pattern_table_address(&self) -> u8 {
        (self.0 & 0b0001_0000) >> 4
    }

    /// Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
    pub fn sprite_size(&self) -> u8 {
        (self.0 & 0b0010_0000) >> 5
    }

    /// PPU master/slave select
    /// (0: read backdrop from EXT pins; 1: output color on EXT pins)
    pub fn master_slave_select(&self) -> u8 {
        (self.0 & 0b0100_0000) >> 6
    }

    /// Generate an NMI at the start of the            
    /// vertical blanking interval (0: off; 1: on)
    pub fn generate_nmi_on_blanking(&self) -> u8 {
        (self.0 & 0b1000_0000) >> 7
    }
}

#[allow(clippy::upper_case_acronyms)]
pub struct Ppu {
    pub registers: [u8; 8],
    pub ppu_status: PpuStatus,
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
            ppu_status: PpuStatus::new(),
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

// "Read" Ppu controls
impl Ppu {
    pub fn read_ppu_status(&self) -> u8 {
        dbg!("read ppu status");
        // Unimplemented
        0
    }

    pub fn read_oam_data(&self) -> u8 {
        dbg!("read oam data");
        // Unimplemented
        0
    }
}

// "Write" Ppu controls
impl Ppu {
    pub fn write_ppu_ctrl(&mut self) {
        // Unimplemented
    }

    pub fn write_ppu_mask(&mut self) {
        // Unimplemented
    }

    pub fn write_oam_addr(&mut self) {
        // Unimplemented
    }

    pub fn write_oam_data(&mut self) {
        // Unimplemented
    }

    pub fn write_ppu_scroll(&mut self) {
        // Unimplemented
    }

    pub fn write_ppu_addr(&mut self) {
        // Unimplemented
    }

    pub fn write_ppu_data(&mut self) {
        // Unimplemented
    }
}
