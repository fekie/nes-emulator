#[allow(dead_code)]
pub fn hex_print_word(word: u16) {
    println!("0x{:04X}", word);
}

#[allow(dead_code)]
pub fn hex_print_byte(byte: u8) {
    println!("0x{:02X}", byte);
}
