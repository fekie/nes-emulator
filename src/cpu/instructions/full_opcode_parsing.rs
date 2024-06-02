use super::{AddressingMode, FullOpcode, Opcode};

impl FullOpcode {
    pub fn new(byte: u8) -> FullOpcode {
        let low_nibble = byte & 0b0000_1111;
        let high_nibble = byte >> 4;

        match low_nibble {
            0x0 => low_nibble_0(high_nibble),
            0x1 => low_nibble_1(high_nibble),
            0x2 => low_nibble_2(high_nibble),
            0x3 => low_nibble_3(high_nibble),
            0x4 => low_nibble_4(high_nibble),
            0x5 => low_nibble_5(high_nibble),
            0x6 => low_nibble_6(high_nibble),
            0x7 => low_nibble_7(high_nibble),
            0x8 => low_nibble_8(high_nibble),
            0x9 => low_nibble_9(high_nibble),
            0xA => low_nibble_a(high_nibble),
            0xB => low_nibble_b(high_nibble),
            0xC => low_nibble_c(high_nibble),
            0xD => low_nibble_d(high_nibble),
            0xE => low_nibble_e(high_nibble),
            0xF => low_nibble_f(high_nibble),
            _ => unreachable!(),
        }
    }
}

fn low_nibble_0(high_nibble: u8) -> FullOpcode {
    match high_nibble {
        0x0 => FullOpcode {
            opcode: Opcode::BRK,
            addressing_mode: AddressingMode::Implied,
        },
        0x1 => FullOpcode {
            opcode: Opcode::BPL,
            addressing_mode: AddressingMode::Relative,
        },
        0x2 => FullOpcode {
            opcode: Opcode::JSR,
            addressing_mode: AddressingMode::Absolute,
        },
        0x3 => FullOpcode {
            opcode: Opcode::BMI,
            addressing_mode: AddressingMode::Relative,
        },
        0x4 => FullOpcode {
            opcode: Opcode::RTI,
            addressing_mode: AddressingMode::Implied,
        },
        0x5 => FullOpcode {
            opcode: Opcode::BVC,
            addressing_mode: AddressingMode::Relative,
        },
        0x6 => FullOpcode {
            opcode: Opcode::RTS,
            addressing_mode: AddressingMode::Implied,
        },
        0x7 => FullOpcode {
            opcode: Opcode::BVS,
            addressing_mode: AddressingMode::Relative,
        },
        0x8 => panic!(),
        0x9 => FullOpcode {
            opcode: Opcode::BCC,
            addressing_mode: AddressingMode::Relative,
        },
        0xA => FullOpcode {
            opcode: Opcode::LDY,
            addressing_mode: AddressingMode::Immediate,
        },
        0xB => FullOpcode {
            opcode: Opcode::BCS,
            addressing_mode: AddressingMode::Relative,
        },
        0xC => FullOpcode {
            opcode: Opcode::CPY,
            addressing_mode: AddressingMode::Immediate,
        },
        0xD => FullOpcode {
            opcode: Opcode::BNE,
            addressing_mode: AddressingMode::Relative,
        },
        0xE => FullOpcode {
            opcode: Opcode::CPX,
            addressing_mode: AddressingMode::Immediate,
        },
        0xF => FullOpcode {
            opcode: Opcode::BEQ,
            addressing_mode: AddressingMode::Relative,
        },
        _ => unreachable!(),
    }
}

fn low_nibble_1(high_nibble: u8) -> FullOpcode {
    match high_nibble {
        0x0 => FullOpcode {
            opcode: Opcode::ORA,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0x1 => FullOpcode {
            opcode: Opcode::ORA,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0x2 => FullOpcode {
            opcode: Opcode::AND,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0x3 => FullOpcode {
            opcode: Opcode::AND,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0x4 => FullOpcode {
            opcode: Opcode::EOR,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0x5 => FullOpcode {
            opcode: Opcode::EOR,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0x6 => FullOpcode {
            opcode: Opcode::ADC,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0x7 => FullOpcode {
            opcode: Opcode::ADC,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0x8 => FullOpcode {
            opcode: Opcode::STA,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0x9 => FullOpcode {
            opcode: Opcode::STA,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0xA => FullOpcode {
            opcode: Opcode::LDA,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0xB => FullOpcode {
            opcode: Opcode::LDA,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0xC => FullOpcode {
            opcode: Opcode::CMP,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0xD => FullOpcode {
            opcode: Opcode::CMP,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0xE => FullOpcode {
            opcode: Opcode::SBC,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0xF => FullOpcode {
            opcode: Opcode::SBC,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        _ => unreachable!(),
    }
}

fn low_nibble_2(high_nibble: u8) -> FullOpcode {
    match high_nibble {
        0x0 => panic!("Illegal instruction"),
        0x1 => panic!("Illegal instruction"),
        0x2 => panic!("Illegal instruction"),
        0x3 => panic!("Illegal instruction"),
        0x4 => panic!("Illegal instruction"),
        0x5 => panic!("Illegal instruction"),
        0x6 => panic!("Illegal instruction"),
        0x7 => panic!("Illegal instruction"),
        0x8 => panic!("Illegal instruction"),
        0x9 => panic!("Illegal instruction"),
        0xA => FullOpcode {
            opcode: Opcode::LDX,
            addressing_mode: AddressingMode::Immediate,
        },
        0xB => panic!("Illegal instruction"),
        0xC => panic!("Illegal instruction"),
        0xD => panic!("Illegal instruction"),
        0xE => panic!("Illegal instruction"),
        0xF => panic!("Illegal instruction"),
        _ => unreachable!(),
    }
}

fn low_nibble_3(high_nibble: u8) -> FullOpcode {
    match high_nibble {
        0x0 => panic!("Illegal instruction"),
        0x1 => panic!("Illegal instruction"),
        0x2 => panic!("Illegal instruction"),
        0x3 => panic!("Illegal instruction"),
        0x4 => panic!("Illegal instruction"),
        0x5 => panic!("Illegal instruction"),
        0x6 => panic!("Illegal instruction"),
        0x7 => panic!("Illegal instruction"),
        0x8 => panic!("Illegal instruction"),
        0x9 => panic!("Illegal instruction"),
        0xA => panic!("Illegal instruction"),
        0xB => panic!("Illegal instruction"),
        0xC => panic!("Illegal instruction"),
        0xD => panic!("Illegal instruction"),
        0xE => panic!("Illegal instruction"),
        0xF => panic!("Illegal instruction"),
        _ => unreachable!(),
    }
}

fn low_nibble_4(high_nibble: u8) -> FullOpcode {
    match high_nibble {
        0x0 => panic!("Illegal instruction"),
        0x1 => panic!("Illegal instruction"),
        0x2 => FullOpcode {
            opcode: Opcode::BIT,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x3 => panic!("Illegal instruction"),
        0x4 => panic!("Illegal instruction"),
        0x5 => panic!("Illegal instruction"),
        0x6 => panic!("Illegal instruction"),
        0x7 => panic!("Illegal instruction"),
        0x8 => FullOpcode {
            opcode: Opcode::STY,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x9 => FullOpcode {
            opcode: Opcode::STY,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xA => FullOpcode {
            opcode: Opcode::LDY,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xB => FullOpcode {
            opcode: Opcode::LDY,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xC => FullOpcode {
            opcode: Opcode::CPY,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xD => panic!("Illegal instruction"),
        0xE => FullOpcode {
            opcode: Opcode::CPX,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xF => panic!("Illegal instruction"),
        _ => unreachable!(),
    }
}

fn low_nibble_5(high_nibble: u8) -> FullOpcode {
    match high_nibble {
        0x0 => FullOpcode {
            opcode: Opcode::ORA,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x1 => FullOpcode {
            opcode: Opcode::ORA,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x2 => FullOpcode {
            opcode: Opcode::AND,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x3 => FullOpcode {
            opcode: Opcode::AND,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x4 => FullOpcode {
            opcode: Opcode::EOR,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x5 => FullOpcode {
            opcode: Opcode::EOR,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x6 => FullOpcode {
            opcode: Opcode::ADC,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x7 => FullOpcode {
            opcode: Opcode::ADC,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x8 => FullOpcode {
            opcode: Opcode::STA,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x9 => FullOpcode {
            opcode: Opcode::STA,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xA => FullOpcode {
            opcode: Opcode::LDA,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xB => FullOpcode {
            opcode: Opcode::LDA,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xC => FullOpcode {
            opcode: Opcode::CMP,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xD => FullOpcode {
            opcode: Opcode::CMP,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xE => FullOpcode {
            opcode: Opcode::SBC,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xF => FullOpcode {
            opcode: Opcode::SBC,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        _ => unreachable!(),
    }
}

fn low_nibble_6(high_nibble: u8) -> FullOpcode {
    match high_nibble {
        0x0 => FullOpcode {
            opcode: Opcode::ASL,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x1 => FullOpcode {
            opcode: Opcode::ASL,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x2 => FullOpcode {
            opcode: Opcode::ROL,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x3 => FullOpcode {
            opcode: Opcode::ROL,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x4 => FullOpcode {
            opcode: Opcode::LSR,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x5 => FullOpcode {
            opcode: Opcode::LSR,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x6 => FullOpcode {
            opcode: Opcode::ROR,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x7 => FullOpcode {
            opcode: Opcode::ROR,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x8 => FullOpcode {
            opcode: Opcode::STX,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x9 => FullOpcode {
            opcode: Opcode::STX,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xA => FullOpcode {
            opcode: Opcode::LDX,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xB => FullOpcode {
            opcode: Opcode::LDX,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xC => FullOpcode {
            opcode: Opcode::DEC,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xD => FullOpcode {
            opcode: Opcode::DEC,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xE => FullOpcode {
            opcode: Opcode::INC,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xF => FullOpcode {
            opcode: Opcode::INC,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        _ => unreachable!(),
    }
}

fn low_nibble_7(high_nibble: u8) -> FullOpcode {
    match high_nibble {
        0x0 => panic!("Illegal instruction"),
        0x1 => panic!("Illegal instruction"),
        0x2 => panic!("Illegal instruction"),
        0x3 => panic!("Illegal instruction"),
        0x4 => panic!("Illegal instruction"),
        0x5 => panic!("Illegal instruction"),
        0x6 => panic!("Illegal instruction"),
        0x7 => panic!("Illegal instruction"),
        0x8 => panic!("Illegal instruction"),
        0x9 => panic!("Illegal instruction"),
        0xA => panic!("Illegal instruction"),
        0xB => panic!("Illegal instruction"),
        0xC => panic!("Illegal instruction"),
        0xD => panic!("Illegal instruction"),
        0xE => panic!("Illegal instruction"),
        0xF => panic!("Illegal instruction"),
        _ => unreachable!(),
    }
}

fn low_nibble_8(high_nibble: u8) -> FullOpcode {
    match high_nibble {
        0x0 => FullOpcode {
            opcode: Opcode::PHP,
            addressing_mode: AddressingMode::Implied,
        },
        0x1 => FullOpcode {
            opcode: Opcode::CLC,
            addressing_mode: AddressingMode::Implied,
        },
        0x2 => FullOpcode {
            opcode: Opcode::PLP,
            addressing_mode: AddressingMode::Implied,
        },
        0x3 => FullOpcode {
            opcode: Opcode::SEC,
            addressing_mode: AddressingMode::Implied,
        },
        0x4 => FullOpcode {
            opcode: Opcode::PHA,
            addressing_mode: AddressingMode::Implied,
        },
        0x5 => FullOpcode {
            opcode: Opcode::CLI,
            addressing_mode: AddressingMode::Implied,
        },
        0x6 => FullOpcode {
            opcode: Opcode::PLA,
            addressing_mode: AddressingMode::Implied,
        },
        0x7 => FullOpcode {
            opcode: Opcode::SEI,
            addressing_mode: AddressingMode::Implied,
        },
        0x8 => FullOpcode {
            opcode: Opcode::DEY,
            addressing_mode: AddressingMode::Implied,
        },
        0x9 => FullOpcode {
            opcode: Opcode::TYA,
            addressing_mode: AddressingMode::Implied,
        },
        0xA => FullOpcode {
            opcode: Opcode::TAY,
            addressing_mode: AddressingMode::Implied,
        },
        0xB => FullOpcode {
            opcode: Opcode::CLV,
            addressing_mode: AddressingMode::Implied,
        },
        0xC => FullOpcode {
            opcode: Opcode::INY,
            addressing_mode: AddressingMode::Implied,
        },
        0xD => FullOpcode {
            opcode: Opcode::CLD,
            addressing_mode: AddressingMode::Implied,
        },
        0xE => FullOpcode {
            opcode: Opcode::INX,
            addressing_mode: AddressingMode::Implied,
        },
        0xF => FullOpcode {
            opcode: Opcode::SED,
            addressing_mode: AddressingMode::Implied,
        },
        _ => unreachable!(),
    }
}

fn low_nibble_9(high_nibble: u8) -> FullOpcode {
    match high_nibble {
        0x0 => FullOpcode {
            opcode: Opcode::ORA,
            addressing_mode: AddressingMode::Immediate,
        },
        0x1 => FullOpcode {
            opcode: Opcode::ORA,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0x2 => FullOpcode {
            opcode: Opcode::AND,
            addressing_mode: AddressingMode::Immediate,
        },
        0x3 => FullOpcode {
            opcode: Opcode::AND,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0x4 => FullOpcode {
            opcode: Opcode::EOR,
            addressing_mode: AddressingMode::Immediate,
        },
        0x5 => FullOpcode {
            opcode: Opcode::EOR,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0x6 => FullOpcode {
            opcode: Opcode::ADC,
            addressing_mode: AddressingMode::Immediate,
        },
        0x7 => FullOpcode {
            opcode: Opcode::ADC,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0x8 => panic!("Illegal instruction"),
        0x9 => FullOpcode {
            opcode: Opcode::STA,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0xA => FullOpcode {
            opcode: Opcode::LDA,
            addressing_mode: AddressingMode::Immediate,
        },
        0xB => FullOpcode {
            opcode: Opcode::LDA,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0xC => FullOpcode {
            opcode: Opcode::CMP,
            addressing_mode: AddressingMode::Immediate,
        },
        0xD => FullOpcode {
            opcode: Opcode::CMP,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0xE => FullOpcode {
            opcode: Opcode::SBC,
            addressing_mode: AddressingMode::Immediate,
        },
        0xF => FullOpcode {
            opcode: Opcode::SBC,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        _ => unreachable!(),
    }
}

fn low_nibble_a(high_nibble: u8) -> FullOpcode {
    match high_nibble {
        0x0 => FullOpcode {
            opcode: Opcode::ASL,
            addressing_mode: AddressingMode::Implied,
        },
        0x1 => panic!("Illegal instruction"),
        0x2 => FullOpcode {
            opcode: Opcode::ROL,
            addressing_mode: AddressingMode::Implied,
        },
        0x3 => panic!("Illegal instruction"),
        0x4 => FullOpcode {
            opcode: Opcode::LSR,
            addressing_mode: AddressingMode::Implied,
        },
        0x5 => panic!("Illegal instruction"),
        0x6 => FullOpcode {
            opcode: Opcode::ROR,
            addressing_mode: AddressingMode::Implied,
        },
        0x7 => panic!("Illegal instruction"),
        0x8 => FullOpcode {
            opcode: Opcode::TXA,
            addressing_mode: AddressingMode::Implied,
        },
        0x9 => FullOpcode {
            opcode: Opcode::TXS,
            addressing_mode: AddressingMode::Implied,
        },
        0xA => FullOpcode {
            opcode: Opcode::TAX,
            addressing_mode: AddressingMode::Implied,
        },
        0xB => FullOpcode {
            opcode: Opcode::TSX,
            addressing_mode: AddressingMode::Implied,
        },
        0xC => FullOpcode {
            opcode: Opcode::DEX,
            addressing_mode: AddressingMode::Implied,
        },
        0xD => panic!("Illegal instruction"),
        0xE => FullOpcode {
            opcode: Opcode::NOP,
            addressing_mode: AddressingMode::Implied,
        },
        0xF => panic!("Illegal instruction"),
        _ => unreachable!(),
    }
}

fn low_nibble_b(high_nibble: u8) -> FullOpcode {
    match high_nibble {
        0x0 => panic!("Illegal instruction"),
        0x1 => panic!("Illegal instruction"),
        0x2 => panic!("Illegal instruction"),
        0x3 => panic!("Illegal instruction"),
        0x4 => panic!("Illegal instruction"),
        0x5 => panic!("Illegal instruction"),
        0x6 => panic!("Illegal instruction"),
        0x7 => panic!("Illegal instruction"),
        0x8 => panic!("Illegal instruction"),
        0x9 => panic!("Illegal instruction"),
        0xA => panic!("Illegal instruction"),
        0xB => panic!("Illegal instruction"),
        0xC => panic!("Illegal instruction"),
        0xD => panic!("Illegal instruction"),
        0xE => panic!("Illegal instruction"),
        0xF => panic!("Illegal instruction"),
        _ => unreachable!(),
    }
}

fn low_nibble_c(high_nibble: u8) -> FullOpcode {
    match high_nibble {
        0x0 => panic!("Illegal instruction"),
        0x1 => panic!("Illegal instruction"),
        0x2 => FullOpcode {
            opcode: Opcode::BIT,
            addressing_mode: AddressingMode::Absolute,
        },
        0x3 => panic!("Illegal instruction"),
        0x4 => FullOpcode {
            opcode: Opcode::JMP,
            addressing_mode: AddressingMode::Absolute,
        },
        0x5 => panic!("Illegal instruction"),
        0x6 => FullOpcode {
            opcode: Opcode::JMP,
            addressing_mode: AddressingMode::Indirect,
        },
        0x7 => panic!("Illegal instruction"),
        0x8 => FullOpcode {
            opcode: Opcode::STY,
            addressing_mode: AddressingMode::Absolute,
        },
        0x9 => panic!("Illegal instruction"),
        0xA => FullOpcode {
            opcode: Opcode::LDY,
            addressing_mode: AddressingMode::Absolute,
        },
        0xB => FullOpcode {
            opcode: Opcode::LDY,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0xC => FullOpcode {
            opcode: Opcode::CPY,
            addressing_mode: AddressingMode::Absolute,
        },
        0xD => panic!("Illegal instruction"),
        0xE => FullOpcode {
            opcode: Opcode::CPX,
            addressing_mode: AddressingMode::Absolute,
        },
        0xF => panic!("Illegal instruction"),
        _ => unreachable!(),
    }
}

fn low_nibble_d(high_nibble: u8) -> FullOpcode {
    match high_nibble {
        0x0 => FullOpcode {
            opcode: Opcode::ORA,
            addressing_mode: AddressingMode::Absolute,
        },
        0x1 => FullOpcode {
            opcode: Opcode::ORA,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x2 => FullOpcode {
            opcode: Opcode::AND,
            addressing_mode: AddressingMode::Absolute,
        },
        0x3 => FullOpcode {
            opcode: Opcode::AND,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x4 => FullOpcode {
            opcode: Opcode::EOR,
            addressing_mode: AddressingMode::Absolute,
        },
        0x5 => FullOpcode {
            opcode: Opcode::EOR,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x6 => FullOpcode {
            opcode: Opcode::ADC,
            addressing_mode: AddressingMode::Absolute,
        },
        0x7 => FullOpcode {
            opcode: Opcode::ADC,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x8 => FullOpcode {
            opcode: Opcode::STA,
            addressing_mode: AddressingMode::Absolute,
        },
        0x9 => FullOpcode {
            opcode: Opcode::STA,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0xA => FullOpcode {
            opcode: Opcode::LDA,
            addressing_mode: AddressingMode::Absolute,
        },
        0xB => FullOpcode {
            opcode: Opcode::LDA,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0xC => FullOpcode {
            opcode: Opcode::CMP,
            addressing_mode: AddressingMode::Absolute,
        },
        0xD => FullOpcode {
            opcode: Opcode::CMP,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0xE => FullOpcode {
            opcode: Opcode::SBC,
            addressing_mode: AddressingMode::Absolute,
        },
        0xF => FullOpcode {
            opcode: Opcode::SBC,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        _ => unreachable!(),
    }
}
fn low_nibble_e(high_nibble: u8) -> FullOpcode {
    match high_nibble {
        0x0 => FullOpcode {
            opcode: Opcode::ASL,
            addressing_mode: AddressingMode::Absolute,
        },
        0x1 => FullOpcode {
            opcode: Opcode::ASL,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x2 => FullOpcode {
            opcode: Opcode::ROL,
            addressing_mode: AddressingMode::Absolute,
        },
        0x3 => FullOpcode {
            opcode: Opcode::ROL,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x4 => FullOpcode {
            opcode: Opcode::LSR,
            addressing_mode: AddressingMode::Absolute,
        },
        0x5 => FullOpcode {
            opcode: Opcode::LSR,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x6 => FullOpcode {
            opcode: Opcode::ROR,
            addressing_mode: AddressingMode::Absolute,
        },
        0x7 => FullOpcode {
            opcode: Opcode::ROR,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x8 => FullOpcode {
            opcode: Opcode::STX,
            addressing_mode: AddressingMode::Absolute,
        },
        0x9 => panic!("Illegal instruction"),
        0xA => FullOpcode {
            opcode: Opcode::LDX,
            addressing_mode: AddressingMode::Absolute,
        },
        0xB => FullOpcode {
            opcode: Opcode::LDX,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0xC => FullOpcode {
            opcode: Opcode::DEC,
            addressing_mode: AddressingMode::Absolute,
        },
        0xD => FullOpcode {
            opcode: Opcode::DEC,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0xE => FullOpcode {
            opcode: Opcode::INC,
            addressing_mode: AddressingMode::Absolute,
        },
        0xF => FullOpcode {
            opcode: Opcode::INC,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        _ => unreachable!(),
    }
}
fn low_nibble_f(high_nibble: u8) -> FullOpcode {
    match high_nibble {
        0x0 => panic!("Illegal instruction"),
        0x1 => panic!("Illegal instruction"),
        0x2 => panic!("Illegal instruction"),
        0x3 => panic!("Illegal instruction"),
        0x4 => panic!("Illegal instruction"),
        0x5 => panic!("Illegal instruction"),
        0x6 => panic!("Illegal instruction"),
        0x7 => panic!("Illegal instruction"),
        0x8 => panic!("Illegal instruction"),
        0x9 => panic!("Illegal instruction"),
        0xA => panic!("Illegal instruction"),
        0xB => panic!("Illegal instruction"),
        0xC => panic!("Illegal instruction"),
        0xD => panic!("Illegal instruction"),
        0xE => panic!("Illegal instruction"),
        0xF => panic!("Illegal instruction"),
        _ => unreachable!(),
    }
}
