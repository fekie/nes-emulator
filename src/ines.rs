const HEADER_BYTES: usize = 16;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Ines {
    pub header: Header,
    pub program_rom: Vec<u8>,
    pub character_rom: Vec<u8>,
}

impl Ines {
    pub fn parse(bytes: &[u8]) -> Self {
        todo!()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]

pub struct Header {
    // Size of PRG ROM in 16 KB units
    program_rom_size_multiplier: u8,
    // Size of CHR ROM in 8 KB units (value 0 means the board uses CHR RAM)
    character_rom_size_multiplier: u8,
    nametable_arrangement: NametableArrangement,
    // bits flags_7[8:=5] set as the lowest nibble
    mapper_number: u8,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum NametableArrangement {
    // "horizontally mirrored"
    #[default]
    VerticalArrangement = 0,
    // "vertically mirrored"
    HorizontalArrangement = 1,
}
