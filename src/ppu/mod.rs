pub struct PPU {
    pub registers: [u8; 8],
}

impl PPU {
    pub fn new() -> Self {
        PPU { registers: [0; 8] }
    }
}
