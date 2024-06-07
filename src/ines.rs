const HEADER_BYTES: usize = 16;
const KB: usize = 1024;

const DEFAULT_PROGRAM_ROM_SIZE_MULTIPLIER: u8 = 2;
const DEFAULT_CHARACTER_ROM_SIZE_MULTIPLIER: u8 = 1;
const PROGRAM_BLOCK_SIZE: usize = 16;
const CHARACTER_BLOCK_SIZE: usize = 8;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Ines {
    pub header: Header,
    pub program_rom: Vec<u8>,
    pub character_rom: Vec<u8>,
}

impl Ines {
    pub fn parse(bytes: &[u8]) -> Self {
        let header_bytes = &bytes[0..HEADER_BYTES];

        assert_eq!(header_bytes[0], 0x4E, "File is not a valid NES ROM.");
        assert_eq!(header_bytes[1], 0x45, "File is not a valid NES ROM.");
        assert_eq!(header_bytes[2], 0x53, "File is not a valid NES ROM.");
        assert_eq!(header_bytes[3], 0x1A, "File is not a valid NES ROM.");

        let program_rom_size_multiplier = header_bytes[4];
        let character_rom_size_multiplier = header_bytes[5];
        let mapper_number = header_bytes[7] >> 4;
        let nametable_arrangement = match header_bytes[6] & 0x0000_0001 != 0 {
            true => NametableArrangement::HorizontalArrangement,
            false => NametableArrangement::VerticalArrangement,
        };

        let header = Header {
            program_rom_size_multiplier,
            character_rom_size_multiplier,
            nametable_arrangement,
            mapper_number,
        };

        let program_rom_size = program_rom_size_multiplier as usize * KB * PROGRAM_BLOCK_SIZE;
        let character_rom_size = character_rom_size_multiplier as usize * KB * CHARACTER_BLOCK_SIZE;

        let program_rom = bytes[HEADER_BYTES..program_rom_size + HEADER_BYTES].to_vec();
        let character_rom = bytes
            [HEADER_BYTES + program_rom_size..HEADER_BYTES + program_rom_size + character_rom_size]
            .to_vec();

        Self {
            header,
            program_rom,
            character_rom,
        }
    }
}

impl Default for Ines {
    fn default() -> Self {
        let header = Header::default();
        let program_rom =
            vec![0; header.program_rom_size_multiplier as usize * KB * PROGRAM_BLOCK_SIZE];
        let character_rom =
            vec![0; header.character_rom_size_multiplier as usize * KB * CHARACTER_BLOCK_SIZE];

        Self {
            header,
            program_rom,
            character_rom,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]

pub struct Header {
    // Size of PRG ROM in 16 KB units
    pub program_rom_size_multiplier: u8,
    // Size of CHR ROM in 8 KB units (value 0 means the board uses CHR RAM)
    pub character_rom_size_multiplier: u8,
    pub nametable_arrangement: NametableArrangement,
    // bits flags_7[8:=5] set as the lowest nibble
    pub mapper_number: u8,
}

impl Default for Header {
    /// The default header specifies 32KB of program rom, 8KB of character rom,
    /// Horizontal nametable arrangement, and the NROM mapper.
    fn default() -> Self {
        Self {
            program_rom_size_multiplier: DEFAULT_PROGRAM_ROM_SIZE_MULTIPLIER,
            character_rom_size_multiplier: DEFAULT_CHARACTER_ROM_SIZE_MULTIPLIER,
            nametable_arrangement: NametableArrangement::default(),
            mapper_number: 0,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum NametableArrangement {
    // "horizontally mirrored"
    #[default]
    VerticalArrangement = 0,
    // "vertically mirrored"
    HorizontalArrangement = 1,
}
