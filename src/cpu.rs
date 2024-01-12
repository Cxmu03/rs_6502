use crate::registers::Registers;
use crate::instruction_table::INSTRUCTIONS;
use crate::memory::Memory;
use crate::instruction::{Instruction, AddressingMode};
use crate::util::FromTwosComplementBits;

macro_rules! SomeShort {
    ($short: expr) => {
        Some(Operand::Short($short))
    }
}

macro_rules! SomeByte {
    ($byte: expr) => {
        Some(Operand::Byte($byte))
    }
}

enum Operand {
    Byte(u8),
    Short(u16)
}

pub struct Cpu {
    pub registers: Registers,
    pub memory: Box<dyn Memory>,
    pub cycles: u32
}

impl Cpu {
    pub fn new(memory: Box<dyn Memory>) -> Cpu {
        let mut cpu = Cpu {
            registers: Registers::new(),
            memory: memory,
            cycles: 0
        };

        cpu.init_registers();

        return cpu;
    }

    pub fn reset(&mut self) {
        self.registers = Registers::new(); 
        self.init_registers();

        self.cycles = 8;
    }

    fn init_registers(&mut self) {
        self.registers.Pc = self.memory.read_short(0xFFFD);
        self.registers.Sp = 0x1FF;
    }

    fn read_current_byte(&self) -> u8 {
        self.memory.read_byte(self.registers.Pc)
    }

    fn read_current_short(&self) -> u16 {
        self.memory.read_short(self.registers.Pc)
    }

    fn indexed_zero_page(&self, register: u8) -> u16 {
        let zero_page_address = self.memory.read_byte(self.registers.Pc);

        zero_page_address.wrapping_add(register) as u16
    }

    fn get_operand_for_instruction(&self, instruction: &Instruction) -> Option<Operand> {
        match instruction.mode {
            AddressingMode::Absolute => {
                SomeShort!(self.read_current_short())
            } 
            AddressingMode::AbsoluteX => {
                SomeShort!(self.read_current_short() + (self.registers.X as u16))
            }
            AddressingMode::AbsoluteY => {
                SomeShort!(self.read_current_short() + (self.registers.Y as u16))
            }
            AddressingMode::ZeroPage => {
                SomeByte!(self.read_current_byte())
            }
            AddressingMode::ZeroPageX => {
                SomeShort!(self.indexed_zero_page(self.registers.X)) 
            }
            AddressingMode::ZeroPageY => {
                SomeShort!(self.indexed_zero_page(self.registers.Y)) 
            }
            AddressingMode::Indirect => {
                let direct_address = self.memory.read_short(self.registers.Pc);
                let indirect_address = self.memory.read_short(direct_address);
                SomeShort!(indirect_address)
            }
            AddressingMode::IndirectX => {
                let direct_address: u16 = self.read_current_short();
                let indirect_address = direct_address + self.registers.X as u16;
                SomeShort!(indirect_address)
            }
            AddressingMode::IndirectY => {
                let direct_address: u16 = self.read_current_short();
                let indirect_address = direct_address + self.registers.Y as u16;
                SomeShort!(indirect_address)
            }
            AddressingMode::Relative => {
                let offset: i8 = i8::from_twos_complement_bits(self.read_current_byte());

                SomeShort!(self.registers.Pc.wrapping_add_signed(offset.into()))
            }
            AddressingMode::Immediate => {
                SomeByte!(self.memory.read_byte(self.registers.Pc))
            }
            _ => {
                None
            }
        }
    }


    pub fn step(&mut self) {
        let opcode: u8 = self.read_current_byte();
        let current_instruction = &INSTRUCTIONS[opcode as usize];

        self.registers.Pc += 1;

        let operand = self.get_operand_for_instruction(current_instruction);

        self.registers.Pc += current_instruction.mode.operand_size();

        self.execute_instruction(&current_instruction, operand); 

        self.cycles += current_instruction.cycles as u32;
    }

    fn execute_instruction(&mut self, instruction: &Instruction, operand: Option<Operand>) {
        todo!()
    }

    fn push_byte(&mut self, value: u8) {
        self.registers.Sp -= 1;

        if self.registers.Sp <= 0xFF {
            panic!("Stack overflow")
        }

        self.memory.write_byte(self.registers.Sp, value);
    }

    fn push_short(&mut self, value: u16) {
        self.registers.Sp -= 2;

        if self.registers.Sp <= 0xFF {
            panic!("Stack overflow")
        }

        self.memory.write_short(self.registers.Sp, value);
    }

    fn pop_byte(&mut self, address: u16) -> u8 {
        let value = self.memory.read_byte(address);
        self.registers.Sp += 1;
        
        value
    }

    fn pop_short(&mut self, address: u16) -> u16 {
        let value = self.memory.read_short(address);
        self.registers.Sp += 2;

        value

    }
}
