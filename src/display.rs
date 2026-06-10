use crate::ppu;
use rgb::Rgb;
use std::sync::Mutex;

pub const WIDTH: usize = ppu::VISIBLE_DOTS;
pub const HEIGHT: usize = ppu::VISIBLE_SCANLINES;

// Using an array instead of a vector will lead to a stackoverflow, even when Box'ing.
pub struct Pixels(Mutex<Vec<u32>>);

impl Pixels {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(Mutex::new(vec![0; WIDTH * HEIGHT]))
    }

    pub fn read(&self, x: usize, y: usize) -> Rgb<u8> {
        let i = (y * WIDTH) + x;
        let raw: u32 = self.0.lock().unwrap()[i];

        let r = ((raw >> 16) & 0xF) as u8;
        let g = ((raw >> 8) & 0xF) as u8;
        let b = (raw & 0xF) as u8;

        Rgb { r, g, b }
    }

    pub fn write(&self, x: usize, y: usize, rgb: Rgb<u8>) {
        let i = (y * WIDTH) + x;
        self.0.lock().unwrap()[i] = ((rgb.r as u32) << 16) | ((rgb.g as u32) << 8) | (rgb.b as u32);
    }

    pub fn copy_to_buffer(&self, buffer: &mut [u32]) {
        buffer.copy_from_slice(self.0.lock().unwrap().as_slice())
    }

    pub fn copy_to_app_buffer(&self, buffer: &mut [u32], app_width: usize) {
        let pixels = self.0.lock().unwrap();
        for y in 0..HEIGHT {
            let source_row = y * WIDTH;
            let target_row = y * app_width;
            buffer[target_row..target_row + WIDTH]
                .copy_from_slice(&pixels[source_row..source_row + WIDTH]);
        }
    }
}
