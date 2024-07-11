#[allow(clippy::upper_case_acronyms)]
pub struct Apu {
    initialized: bool,
}

impl Apu {
    /// Creates the APU but does not initialize it. Please run [`Initialize`] to
    /// initialize the APU.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { initialized: false }
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
