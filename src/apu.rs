use crate::bus::Bus;

pub struct APU {
    initialized: bool,
}

impl APU {
    /// Creates the APU but does not initialize it. Please run [`Initialize`] to
    /// initialize the APU.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        APU { initialized: false }
    }
}

impl APU {
    /// Initialize the APU.
    pub fn initialize(&mut self, bus: &Bus) {
        self.initialized = true;
    }

    /// Returns the state of initialization.
    pub fn initialized(&self) -> bool {
        self.initialized
    }
}
