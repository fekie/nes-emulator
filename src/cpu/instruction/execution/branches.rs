use super::twos_compliment_to_signed;
use super::CPU;

impl CPU {
    pub(crate) fn instruction_bcc(&mut self, low_byte: Option<u8>) -> u8 {
        let needs_branch = !self.processor_status.carry_flag();
        branch(self, low_byte, needs_branch)
    }

    pub(crate) fn instruction_bcs(&mut self, low_byte: Option<u8>) -> u8 {
        let needs_branch = self.processor_status.carry_flag();
        branch(self, low_byte, needs_branch)
    }

    pub(crate) fn instruction_beq(&mut self, low_byte: Option<u8>) -> u8 {
        let needs_branch = self.processor_status.zero_flag();
        branch(self, low_byte, needs_branch)
    }

    pub(crate) fn instruction_bmi(&mut self, low_byte: Option<u8>) -> u8 {
        let needs_branch = self.processor_status.negative_flag();
        branch(self, low_byte, needs_branch)
    }

    pub(crate) fn instruction_bne(&mut self, low_byte: Option<u8>) -> u8 {
        let needs_branch = !self.processor_status.zero_flag();
        branch(self, low_byte, needs_branch)
    }

    pub(crate) fn instruction_bpl(&mut self, low_byte: Option<u8>) -> u8 {
        let needs_branch = !self.processor_status.negative_flag();
        branch(self, low_byte, needs_branch)
    }

    pub(crate) fn instruction_bvc(&mut self, low_byte: Option<u8>) -> u8 {
        let needs_branch = !self.processor_status.overflow_flag();
        branch(self, low_byte, needs_branch)
    }

    pub(crate) fn instruction_bvs(&mut self, low_byte: Option<u8>) -> u8 {
        let needs_branch = self.processor_status.overflow_flag();
        branch(self, low_byte, needs_branch)
    }
}

/// Executes a branch based on whether it needs a branch.
fn branch(cpu: &mut CPU, low_byte: Option<u8>, needs_branch: bool) -> u8 {
    let value = twos_compliment_to_signed(low_byte.unwrap());
    let original_page = cpu.program_counter >> 8;

    if needs_branch {
        match value.is_positive() {
            true => cpu.program_counter - value as u16,
            false => cpu.program_counter - (-value) as u16,
        };
    }

    let new_page = cpu.program_counter >> 8;
    let page_crossed = original_page != new_page;

    match needs_branch {
        true => match page_crossed {
            true => 4,
            false => 3,
        },
        false => 2,
    }
}
