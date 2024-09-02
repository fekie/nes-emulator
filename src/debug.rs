use std::fmt::Write;

#[allow(dead_code)]
pub fn hex_print_word(word: u16) {
    println!("0x{:04X}", word);
}

#[allow(dead_code)]
pub fn hex_print_byte(byte: u8) {
    println!("0x{:02X}", byte);
}

const BLACK: (u8, u8, u8) = (0, 0, 0);
const DARK_GRAY: (u8, u8, u8) = (70, 70, 70);
const GRAY: (u8, u8, u8) = (140, 140, 140);
const LIGHT_GRAY: (u8, u8, u8) = (210, 210, 210);

/// Takes a raw 8 byte 8x8 pixel tile, and converts to an 8x8 rgb grid. Code
/// contributed by Audri.  
pub fn deinterlace_tile(tile: &[u8]) -> [[(u8, u8, u8); 8]; 8] {
    debug_assert_eq!(tile.len(), 16);

    let plane = tile.split_at(8);
    let mut pixels = [[(0, 0, 0); 8]; 8];

    for y in 0..8 {
        for x in 0..8 {
            let bitmask = 0b1000_0000 >> x;
            let bits = (plane.0[y] & bitmask != 0, plane.1[y] & bitmask != 0);

            let color_index = u8::from(bits.0) + (u8::from(bits.1) << 1);
            pixels[y][x] = match color_index {
                0 => BLACK,
                1 => DARK_GRAY,
                2 => GRAY,
                3 => LIGHT_GRAY,
                _ => panic!("Invalid color index."),
            }
        }
    }

    pixels
}

/// Prints a tile to the terminal with funky ascii character stuff. Contributed
/// by my friend Audri.
pub fn print_tile(tile: [[(u8, u8, u8); 8]; 8]) {
    use std::fmt::Write;
    let mut string = String::new();

    for row in 0..4 {
        for x in 0..8 {
            let top_pixel = tile[row * 2][x];
            let (r, g, b) = top_pixel;

            write!(&mut string, "\x1b[38;2;{r};{g};{b}m");

            let bottom_pixel = tile[row * 2 + 1][x];
            let (r, g, b) = bottom_pixel;

            write!(&mut string, "\x1b[48;2;{r};{g};{b}m▀");
        }

        if row < 3 {
            write!(&mut string, "\x1b[49m\n");
        }
    }

    write!(&mut string, "\x1b[39m\x1b[49m");

    print!("{}", string);
}
