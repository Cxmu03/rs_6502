use crate::instruction::{Instruction, AddressingMode};

macro_rules! instruction_table {
    ($($opcode: expr, $mnemonic: expr, $mode: expr, $cycles: expr, $extra_cycle: expr);+) => {
        &[
            $(Instruction {opcode: $opcode, mnemonic: $mnemonic, mode: $mode, cycles: $cycles, extra_cycle: $extra_cycle},)*
        ]
    }
}

pub const INSTRUCTIONS: &'static [Instruction] = instruction_table! {
    0x00, "BRK", AddressingMode::Implied, 7, false
};

