use crate::bus::BusPointer;

#[allow(clippy::upper_case_acronyms)]
pub struct Apu {
    bus: BusPointer,
    initialized: bool,
}

impl Apu {
    /// Creates the APU but does not initialize it. Please run [`Initialize`] to
    /// initialize the APU.
    #[allow(clippy::new_without_default)]
    pub fn new(bus: BusPointer) -> Self {
        Self {
            bus,
            initialized: false,
        }
    }

    /// Initialize the APU.
    pub fn initialize(&mut self) {
        self.initialized = true;
    }

    /// Returns the state of initialization.
    pub fn initialized(&self) -> bool {
        self.initialized
    }
}
