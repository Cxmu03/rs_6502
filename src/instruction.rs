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
}

pub enum InstructionType {
    ADC, AND, ASL, BCC, BCS, BEQ, BIT, 
    BMI, BNE, BPL, BRK, BVC, BVS, CLC, 
    CLD, CLI, CLV, CMP, CPX, CPY, DEC, 
    DEX, DEY, EOR, INC, INX, INY, JMP, 
    JSR, LDA, LDX, LDY, LSR, NOP, ORA, 
    PHA, PHP, PLA, PLP, ROL, ROR, RTI, 
    RTS, SBC, SEC, SED, SEI, STA, STX, 
    STY, TAX, TAY, TSX, TXA, TXS, TYA,
}

pub struct Instruction {
    pub opcode: u8,
    pub instruction_type: InstructionType,
    pub mode: AddressingMode,
    pub cycles: u8,
    pub extra_cycle: bool // Adds extra cycle if page boundary is crossed
}

impl Instruction {
    pub fn invalid(opcode: u8) -> Instruction {
        Instruction {
            opcode,
            instruction_type: InstructionType::NOP,
            mode: AddressingMode::Implied,
            cycles: 2,
            extra_cycle: false
        }
    }
}
