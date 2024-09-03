use image::{DynamicImage, GrayImage, Rgb, RgbImage, RgbaImage};
use std::fmt::Write;

pub struct Tile(pub [[Rgb<u8>; 8]; 8]);

#[allow(dead_code)]
pub fn hex_print_word(word: u16) {
    println!("0x{:04X}", word);
}

#[allow(dead_code)]
pub fn hex_print_byte(byte: u8) {
    println!("0x{:02X}", byte);
}

const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
const DARK_GRAY: Rgb<u8> = Rgb([70, 70, 70]);
const GRAY: Rgb<u8> = Rgb([140, 140, 140]);
const LIGHT_GRAY: Rgb<u8> = Rgb([210, 210, 210]);

/// Takes 16 bytes and decodes them into an 8x8 rgb tile
pub fn deinterlace_tile_bytes(tile_bytes: &[u8]) -> Tile {
    assert_eq!(tile_bytes.len(), 16);

    let plane = tile_bytes.split_at(8);
    let mut tile_raw = [[Rgb([0, 0, 0]); 8]; 8];
    //let mut rgb: RgbImage = RgbImage::new(8, 8);

    #[allow(clippy::needless_range_loop)]
    for y in 0..8 {
        for x in 0..8 {
            let bitmask = 0b1000_0000 >> x;
            let bits = (plane.0[y] & bitmask != 0, plane.1[y] & bitmask != 0);

            let color_index = u8::from(bits.0) + (u8::from(bits.1) << 1);

            tile_raw[y][x] = match color_index {
                0 => BLACK,
                1 => DARK_GRAY,
                2 => GRAY,
                3 => LIGHT_GRAY,
                _ => panic!("Invalid color index."),
            }
        }
    }

    Tile(tile_raw)
}

/// Stitches together 512 tiles into two 16x16 tile grids, which then attach
/// to each other side by side, where each sequential 16 tiles is a row.
/// Turns the stitched result into an [`RgbImage`]
pub fn stitch_tiles(tiles: &[Tile]) -> RgbImage {
    let mut img = RgbImage::new(256, 128);

    assert_eq!(tiles.len(), 512);

    let left_tiles = &tiles[0..256];

    for (logical_row, tiles_on_row) in left_tiles.chunks(16).enumerate() {
        for subrow in 0..8 {
            let pixels = tiles_on_row
                .iter()
                .flat_map(|tile| tile.0[subrow])
                .collect::<Vec<Rgb<u8>>>();

            let y = (logical_row * 8) + subrow;
            for x in 0..(8 * 16) {
                *img.get_pixel_mut(x, y as u32) = pixels[x as usize];
            }
        }
    }

    let right_tiles = &tiles[256..512];

    for (logical_row, tiles_on_row) in right_tiles.chunks(16).enumerate() {
        for subrow in 0..8 {
            let pixels = tiles_on_row
                .iter()
                .flat_map(|tile| tile.0[subrow])
                .collect::<Vec<Rgb<u8>>>();

            let y = (logical_row * 8) + subrow;
            for x in 0..(8 * 16) {
                *img.get_pixel_mut(x + 128, y as u32) = pixels[x as usize];
            }
        }
    }

    img
}
