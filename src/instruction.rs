
pub enum AddressingMode {
    Accumulator, // Acc, 1 byte
    Immediate,   // 8 bit operand, 1 byte
    Absolute,    // 16 bit absolute address, 3 bytes
    ZeroPage,    // Zp Address, 2 bytes
    ZeroPageX,   // X + 8 bit offset, 2 bytes
    ZeroPageY,   // Y + 8 bit offset, 2 bytes
    AbsoluteX,   // X + 16 bit address offset, 3 bytes
    AbsoluteY,   // Y + 16 bit address offset, 3 bytes
    Relative,    // 8 bit offset for jump, 2 bytes
    Indirect,    // 16 bit address, 3 
    IndirectX,   // todo
    IndirectY,   // todo
    Implied,     // No operand
    Null         // Invalid Instruction
}

pub struct Instruction<'a> {
    pub opcode: u8,
    pub mnemonic: &'a str,
    pub mode: AddressingMode,
    pub cycles: u8,
    pub extra_cycle: bool
}
