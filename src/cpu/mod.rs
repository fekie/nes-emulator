use instruction::{FullOpcode, Instruction, Opcode};

use crate::bus::Bus;
use crate::Mapper;
use processor_status::ProcessorStatus;

const STACK_POINTER_STARTING_VALUE: u8 = 0xFF;
pub const NMI_VECTOR_ADDRESS: u16 = 0xFFFA;
pub const RESET_VECTOR_ADDRESS: u16 = 0xFFFC;
pub const IRQ_BRK_VECTOR_ADDRESS: u16 = 0xFFFE;

mod instruction;
mod processor_status;
mod helper;

#[allow(clippy::upper_case_acronyms)]
pub struct CPU {
    pub accumulator: u8,
    pub x: u8,
    pub y: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
    pub registers: [u8; 6],
    pub processor_status: ProcessorStatus,
    pub memory_mapper: CpuMemoryMapper,
    pub initialized: bool,
}

impl CPU {
    /// Creates a new CPU but does not initialize it as it needs to be connected
    /// to the bus to initialize. You can initialize it with [`Self::initialize`].
    pub fn new() -> Self {
        Self {
            accumulator: 0,
            x: 0,
            y: 0,
            stack_pointer: STACK_POINTER_STARTING_VALUE,
            program_counter: 0,
            registers: [0; 6],
            processor_status: ProcessorStatus::new(),
            memory_mapper: CpuMemoryMapper::new(),
            initialized: false,
        }
    }

    /// Initializes the CPU to a state ready to run instructions.
    pub fn initialize(&mut self, bus: &Bus) {
        self.processor_status.clear_carry_flag();
        self.processor_status.clear_zero_flag();
        self.processor_status.set_interrupt_disable_flag();
        self.processor_status.clear_decimal_flag();
        self.processor_status.clear_overflow_flag();
        self.processor_status.clear_negative_flag();
        self.processor_status.clear_break_flag();

        self.stack_pointer = STACK_POINTER_STARTING_VALUE;

        self.program_counter = {
            let low_byte = self.memory_mapper.read(bus, RESET_VECTOR_ADDRESS) as u16;
            let high_byte = self.memory_mapper.read(bus, RESET_VECTOR_ADDRESS + 1) as u16;
            (high_byte << 8) + low_byte
        };

        self.initialized = true;
    }

    pub fn initialized(&self) -> bool {
        self.initialized
    }

    /// Runs a full instruction cycle. Returns the amount of
    /// machine cycles taken.
    pub fn cycle(&mut self, bus: &Bus) -> u8 {
        // check for interrupts
        /* if *bus.interrupts.borrow().interrupt.borrow() == Request::Active || *bus.interrupts.borrow().non_maskable_interrupt.borrow() == Request::Active {
            // if we get an interrupt, then set the previous pc back

        } */
        // fetch
        let instruction = self.fetch(bus);
        self.pretty_print_cpu_state(instruction);

        // execute
        self.execute(instruction, bus)
    }

    /// Fetches the next instruction and updates the program counter.
    pub fn fetch(&mut self, bus: &Bus) -> Instruction {
        let full_opcode = FullOpcode::new(self.memory_mapper.read(bus, self.program_counter));

        let bytes_required = full_opcode.addressing_mode.bytes_required();

        // Low byte comes first as words are in little-endian
        let (low_byte, high_byte) = match bytes_required {
            1 => (None, None),
            2 => (
                Some(self.memory_mapper.read(bus, self.program_counter + 1)),
                None,
            ),
            3 => (
                Some(self.memory_mapper.read(bus, self.program_counter + 1)),
                Some(self.memory_mapper.read(bus, self.program_counter + 2)),
            ),
            _ => unreachable!(),
        };

        // Decide how much we need to increment the PC
        self.program_counter += bytes_required;

        Instruction {
            opcode: full_opcode.opcode,
            addressing_mode: full_opcode.addressing_mode,
            low_byte,
            high_byte,
        }
    }

    /// Executes the instruction and returns the amount of machine cycles that it took.
    pub fn execute(&mut self, instruction: Instruction, bus: &Bus) -> u8 {
        match instruction.opcode {
            Opcode::ADC => self.instruction_adc(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::AND => self.instruction_and(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::ASL => self.instruction_asl(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::BCC => self.instruction_bcc(instruction.low_byte),
            Opcode::BCS => self.instruction_bcs(instruction.low_byte),
            Opcode::BEQ => self.instruction_beq(instruction.low_byte),
            Opcode::BIT => self.instruction_bit(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::BMI => self.instruction_bmi(instruction.low_byte),
            Opcode::BNE => self.instruction_bne(instruction.low_byte),
            Opcode::BPL => self.instruction_bpl(instruction.low_byte),
            Opcode::BRK => self.instruction_brk(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::BVC => self.instruction_bvc(instruction.low_byte),
            Opcode::BVS => self.instruction_bvs(instruction.low_byte),
            Opcode::CLC => self.instruction_clc(),
            Opcode::CLD => self.instruction_cld(),
            Opcode::CLI => self.instruction_cli(),
            Opcode::CLV => self.instruction_clv(),
            Opcode::CMP => self.instruction_cmp(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::CPX => self.instruction_cpx(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::CPY => self.instruction_cpy(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::DEC => self.instruction_dec(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::DEX => self.instruction_dex(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::DEY => self.instruction_dey(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::EOR => self.instruction_eor(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::INC => self.instruction_inc(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::INX => self.instruction_inx(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::INY => self.instruction_iny(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::JMP => self.instruction_jmp(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::JSR => self.instruction_jsr(bus, instruction.low_byte, instruction.high_byte),
            Opcode::LDA => self.instruction_lda(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::LDX => self.instruction_ldx(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::LDY => self.instruction_ldy(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::LSR => self.instruction_lsr(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::NOP => self.instruction_nop(),
            Opcode::ORA => self.instruction_ora(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::PHA => self.instruction_pha(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::PHP => self.instruction_php(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::PLA => self.instruction_pla(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::PLP => self.instruction_plp(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::ROL => self.instruction_rol(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::ROR => self.instruction_ror(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::RTI => self.instruction_rti(bus),
            Opcode::RTS => self.instruction_rts(bus),
            Opcode::SBC => self.instruction_sbc(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::SEC => self.instruction_sec(),
            Opcode::SED => self.instruction_sed(),
            Opcode::SEI => self.instruction_sei(),
            Opcode::STA => self.instruction_sta(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::STX => self.instruction_stx(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::STY => self.instruction_sty(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::TAX => self.instruction_tax(),
            Opcode::TAY => self.instruction_tay(),
            Opcode::TSX => self.instruction_tsx(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::TXA => self.instruction_txa(),
            Opcode::TXS => self.instruction_txs(bus, instruction.addressing_mode, instruction.low_byte, instruction.high_byte),
            Opcode::TYA => self.instruction_tya(),
        }
    }

    // Shortcuts to read a byte from the memory mapper because
    // we use this a lot.
    pub(in crate::cpu) fn read(&self, bus: &Bus, address: u16) -> u8 {
        self.memory_mapper.read(bus, address)
    }

    pub(in crate::cpu) fn write(&mut self, bus: &Bus, address: u16, value: u8)  {
        self.memory_mapper.write(bus, address, value);
    }

    #[allow(dead_code)]
    /// Pretty prints the full state of the CPU. Meant to be used after fetch but
    /// before execution to work correctly.
    pub fn pretty_print_cpu_state(&self, instruction: Instruction) {
        println!("------------------------------------");
        println!("New PC: ${:02X}", self.program_counter);
        println!("Instruction (not yet executed): {:#?}", instruction);
        println!("Accumulator: {} | X: {} | Y: {}", self.accumulator, self.x, self.y);
        println!("Stack Pointer: ${:02X} -> ${:04X}", self.stack_pointer, self.stack_pointer as u16 + 0x0100);
        println!("Registers: {:?}", self.registers);
        println!(
            "Carry: {} | Zero: {} | Interrupt Disable: {} | Decimal: {} | Break: {} | Overflow: {} | Negative: {}",
            self.processor_status.carry_flag(), self.processor_status.zero_flag(), self.processor_status.interrupt_disable_flag(), 
            self.processor_status.decimal_flag(), self.processor_status.break_flag(), self.processor_status.overflow_flag(), 
            self.processor_status.negative_flag()
        );
        println!("------------------------------------");
    }
}

// We use 2KB of work ram.
#[derive(Debug)]
pub struct WorkRAM([u8; 0x800]);

/// Memory Map:
///
/// | Address range |  Size  |                                  Device                                  |   |   |
/// |:-------------:|:------:|:------------------------------------------------------------------------:|---|---|
/// | $0000–$07FF   | $0800  | 2 KB internal RAM                                                        |   |   |
/// | $0800–$0FFF   | $0800  | Mirror  of $0000–$07FF                                                   |   |   |
/// | $1000–$17FF   | $0800  | Mirror  of $0000–$07FF                                                   |   |   |
/// | $1800–$1FFF   | $0800  | Mirror  of $0000–$07FF                                                   |   |   |
/// | $2000–$2007   | $0008  | NES PPU registers                                                        |   |   |
/// | $2008–$3FFF   | $1FF8  | Mirrors of $2000–$2007 (repeats every 8 bytes)                           |   |   |
/// | $4000–$4017   | $0018  | NES APU and I/O registers                                                |   |   |
/// | $4018–$401F   | $0008  | APU and I/O functionality that is normally disabled. See CPU Test Mode.  |   |   |
/// | $4020–$FFFF   | $BFE0  | Unmapped. Available for cartridge use.                                   |   |   |
/// | *$6000-$7FFF  | $2000  | Usually cartridge RAM, when present.                                     |   |   |
/// | *$8000-$FFFF  | $8000  | Usually cartridge ROM and mapper registers.                              |   |   |
#[derive(Debug)]
pub struct CpuMemoryMapper {
    work_ram: WorkRAM,
}

impl CpuMemoryMapper {
    pub fn new() -> Self {
        Self {
            work_ram: WorkRAM([0; 2048]),
        }
    }
}

impl Mapper for CpuMemoryMapper {
    fn read(&self, bus: &Bus, address: u16) -> u8 {
        match address {
            // Handle the work RAM and the mirrors.
            0x0000..=0x1FFF => self.work_ram.0[address as usize % 0x0800],
            // Handle PPU registers and the mirrors.
            0x2000..=0x3FFF => bus.ppu.borrow().registers[((address - 0x2000) % 8) as usize],
            // Saved for APU
            0x4000..=0x4017 => unimplemented!(),
            // Disabled
            0x4018..=0x401F => unimplemented!(),
            // Route to cartridge mapper
            0x4020..=0xFFFF => bus.cartridge.borrow().read(address),
        }
    }

    fn write(&mut self, bus: &Bus, address: u16, byte: u8) {
        match address {
            // Handle the work RAM and the mirrors.
            0x0000..=0x1FFF => self.work_ram.0[address as usize % 0x0800] = byte,
            // Handle PPU registers and the mirrors.
            0x2000..=0x3FFF => {
                bus.ppu.borrow_mut().registers[((address - 0x2000) % 8) as usize] = byte
            }
            // Saved for APU
            0x4000..=0x4017 => unimplemented!(),
            // Disabled
            0x4018..=0x401F => unimplemented!(),
            // Route to cartridge mapper
            0x4020..=0xFFFF => bus.cartridge.borrow_mut().write(address, byte),
        }
    }
}
