pub enum AddressMode {}

pub enum Instruction {
    BRK {},                    // doesnt havent address mode
    DEY { mode: AddressMode }, // does have address mode
}
