use crate::instruction::{Instruction, AddressingMode};
use crate::registers::Registers;

macro_rules! instruction_table {
    ($($opcode: expr, $mnemonic: expr, $mode: expr, $cycles: expr, $extra_cycle: expr);+) => {
        &[
            $(Instruction {opcode: $opcode, mnemonic: $mnemonic, mode: $mode, cycles: $cycles, extra_cycle: $extra_cycle},)*
        ]
    }
}

macro_rules! invalid_opcode {
    ($opcode: expr) => {
        $opcode, "Invalid", AddressingMode::Null, 0
    }
}

const INSTRUCTIONS: &'static [Instruction] = instruction_table! {
    0x00, "BRK", AddressingMode::Implied, 7, false
};

pub struct Cpu {

}
