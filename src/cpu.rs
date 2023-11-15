use crate::registers::Registers;
use crate::memory::Memory;

pub struct Cpu {
    pub registers: Registers,
    pub memory: Memory,
    pub cycles: u32
}

impl Cpu {
    pub fn new(&mut self) {
        self.reset();
        self.cycles = 0;
    }

    pub fn reset(&mut self) {
        self.registers = Registers::default(); 
        self.registers.Pc = self.memory.read_short(0xFFFD);
        self.memory.sp = 0xFF;

        self.cycles = 8;
    }
}
