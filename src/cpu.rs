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
        self.registers = Registers::new(); 
        self.registers.Pc = self.memory.read::<u16, 2>(0xFFFD);
        self.registers.Sp = 0xFF;

        self.cycles = 8;
    }

    fn push_byte(&mut self, value: u8) {
        self.registers.Sp -= 1;
        self.memory.write::<u8, 1>(self.registers.Sp, value);
    }

    fn push_short(&mut self, value: u16) {
        self.registers.Sp -= 2;
        self.memory.write::<u16, 2>(self.registers.Sp, value);
    }

    fn pop_byte(&mut self, address: u16) -> u8 {
        let value = self.memory.read::<u8, 1>(address);
        self.registers.Sp += 1;
        
        value
    }

    fn pop_short(&mut self, address: u16) -> u16 {
        let value = self.memory.read(address);
        self.registers.Sp += 2;

        value

    }
}
