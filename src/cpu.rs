use crate::registers::Registers;
use crate::memory::Memory;

pub struct Cpu {
    pub registers: Registers,
    pub memory: Memory
}
