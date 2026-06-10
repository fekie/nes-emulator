use crate::cpu::CpuContainer;
use crate::display::Pixels;
use nes6502::Interrupts;
use rgb::Rgb;
use std::{cell::RefCell, rc::Rc};

pub const VISIBLE_DOTS: usize = 256;
pub const VISIBLE_SCANLINES: usize = 240;
pub const DOTS_PER_SCANLINE: usize = 341;
pub const POST_RENDER_SCANLINES: usize = 1;
pub const VBLANK_SCANLINES: usize = 20;
pub const PRE_RENDER_SCANLINES: usize = 1;
pub const TOTAL_SCANLINES: usize =
    VISIBLE_SCANLINES + POST_RENDER_SCANLINES + VBLANK_SCANLINES + PRE_RENDER_SCANLINES;
pub const PPU_DOTS_PER_CPU_CYCLE: usize = 3;
pub const PPU_DOTS_PER_FRAME: usize = DOTS_PER_SCANLINE * TOTAL_SCANLINES;
pub const CPU_CYCLES_PER_FRAME: f64 = PPU_DOTS_PER_FRAME as f64 / PPU_DOTS_PER_CPU_CYCLE as f64;

const VBLANK_START_SCANLINE: usize = VISIBLE_SCANLINES + POST_RENDER_SCANLINES;
const PRE_RENDER_SCANLINE: usize = TOTAL_SCANLINES - 1;
const PPUSTATUS_VBLANK: u8 = 0b1000_0000;

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

    pub fn set(&mut self, byte: u8) {
        self.0 = byte;
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PpuDebugSnapshot {
    pub scanline: usize,
    pub dot: usize,
    pub frame: u64,
    pub in_vblank: bool,
}

#[allow(clippy::upper_case_acronyms)]
pub struct Ppu {
    pub registers: [u8; 8],
    pub ppu_status: PpuStatus,
    pub cpu: Option<Rc<RefCell<CpuContainer>>>,
    pub initialized: bool,
    scanline: usize,
    dot: usize,
    frame: u64,
    in_vblank: bool,
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
            scanline: 0,
            dot: 0,
            frame: 0,
            in_vblank: false,
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

    pub fn debug_snapshot(&self) -> PpuDebugSnapshot {
        PpuDebugSnapshot {
            scanline: self.scanline,
            dot: self.dot,
            frame: self.frame,
            in_vblank: self.in_vblank,
        }
    }

    pub fn clock(&mut self, pixels: &Pixels) {
        if self.scanline < VISIBLE_SCANLINES && self.dot < VISIBLE_DOTS {
            pixels.write(self.dot, self.scanline, self.pixel_color());
        }

        self.advance_dot();
    }

    fn advance_dot(&mut self) {
        self.dot += 1;

        if self.dot < DOTS_PER_SCANLINE {
            return;
        }

        self.dot = 0;
        self.scanline += 1;

        if self.scanline == VBLANK_START_SCANLINE {
            self.start_vblank();
        }

        if self.scanline == PRE_RENDER_SCANLINE {
            self.end_vblank();
        }

        if self.scanline == TOTAL_SCANLINES {
            self.scanline = 0;
            self.frame = self.frame.wrapping_add(1);
        }
    }

    fn pixel_color(&self) -> Rgb<u8> {
        let stripe = ((self.scanline / 8) + (self.frame as usize % 4)) % 4;

        match stripe {
            0 => Rgb {
                r: 0x24,
                g: 0x5C,
                b: 0xA6,
            },
            1 => Rgb {
                r: 0x1F,
                g: 0x8F,
                b: 0x4C,
            },
            2 => Rgb {
                r: 0xB7,
                g: 0x87,
                b: 0x2C,
            },
            _ => Rgb {
                r: 0x8A,
                g: 0x44,
                b: 0xAD,
            },
        }
    }

    fn start_vblank(&mut self) {
        self.in_vblank = true;
        self.registers[2] |= PPUSTATUS_VBLANK;

        if self.ppu_status.generate_nmi_on_blanking() == 1 {
            if let Some(cpu) = &self.cpu {
                cpu.borrow_mut()
                    .0
                    .interrupts
                    .set_non_maskable_interrupt_state(true);
            }
        }
    }

    fn end_vblank(&mut self) {
        self.in_vblank = false;
        self.registers[2] &= !PPUSTATUS_VBLANK;
    }
}

// "Read" Ppu controls
impl Ppu {
    pub fn read_ppu_status(&self) -> u8 {
        self.registers[2]
    }

    pub fn read_oam_data(&self) -> u8 {
        dbg!("read oam data");
        // Unimplemented
        0
    }
}

// "Write" Ppu controls
impl Ppu {
    pub fn write_ppu_ctrl(&mut self, byte: u8) {
        self.registers[0] = byte;
        self.ppu_status.set(byte);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display::Pixels;

    #[test]
    fn clock_writes_visible_pixels_by_dot_and_scanline() {
        let pixels = Pixels::new();
        let mut ppu = Ppu::new();

        ppu.clock(&pixels);

        assert_ne!(pixels.read(0, 0), Rgb { r: 0, g: 0, b: 0 });
        assert_eq!(ppu.debug_snapshot().dot, 1);
        assert_eq!(ppu.debug_snapshot().scanline, 0);
    }

    #[test]
    fn vblank_starts_after_visible_and_post_render_scanlines() {
        let pixels = Pixels::new();
        let mut ppu = Ppu::new();

        for _ in 0..DOTS_PER_SCANLINE * VBLANK_START_SCANLINE {
            ppu.clock(&pixels);
        }

        let snapshot = ppu.debug_snapshot();
        assert_eq!(snapshot.scanline, VBLANK_START_SCANLINE);
        assert_eq!(snapshot.dot, 0);
        assert!(snapshot.in_vblank);
        assert_ne!(ppu.read_ppu_status() & PPUSTATUS_VBLANK, 0);
    }

    #[test]
    fn frame_timing_matches_ntsc_cycle_chart_shape() {
        assert_eq!(DOTS_PER_SCANLINE, 341);
        assert_eq!(VISIBLE_SCANLINES, 240);
        assert_eq!(POST_RENDER_SCANLINES, 1);
        assert_eq!(VBLANK_SCANLINES, 20);
        assert_eq!(PRE_RENDER_SCANLINES, 1);
        assert_eq!(TOTAL_SCANLINES, 262);
        assert!((CPU_CYCLES_PER_FRAME - 29780.666666666668).abs() < f64::EPSILON);
    }
}
