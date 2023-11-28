use crate::registers::Registers;
use crate::memory::{ByteData, DefaultMemory};
use crate::instruction_table::INSTRUCTIONS;
use crate::instruction::{Instruction, AddressingMode};
use crate::util::FromTwosComplementBits;

pub struct Cpu {
    pub registers: Registers,
    pub memory: DefaultMemory,
    pub cycles: u32
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu {
            registers: Registers::new(),
            memory: DefaultMemory::new(),
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
        self.registers.Pc = self.memory.read::<u16, 2>(0xFFFD);
        self.registers.Sp = 0x1FF;
    }

    fn read_address<T: ByteData<N>, const N: usize>(&self) -> T {
        self.memory.read::<T, N>(self.registers.Pc)
    }

    fn indexed_zero_page(&self, register: u8) -> u16 {
        let zero_page_address = self.memory.read::<u8, 1>(self.registers.Pc);

        zero_page_address.wrapping_add(register) as u16
    }

    fn get_operand_for_instruction(&self, instruction: &Instruction) -> Option<u16> {
        match instruction.mode {
            AddressingMode::Absolute => {
                Some(self.read_address::<u16, 2>())
            } 
            AddressingMode::AbsoluteX => {
                Some(self.read_address::<u16, 2>() + (self.registers.X as u16))
            }
            AddressingMode::AbsoluteY => {
                Some(self.read_address::<u16, 2>() + (self.registers.Y as u16))
            }
            AddressingMode::ZeroPage => {
                Some(self.read_address::<u8, 1>() as u16)
            }
            AddressingMode::ZeroPageX => {
                Some(self.indexed_zero_page(self.registers.X)) 
            }
            AddressingMode::ZeroPageY => {
                Some(self.indexed_zero_page(self.registers.Y)) 
            }
            AddressingMode::Indirect => {
                let direct_address = self.memory.read::<u16, 2>(self.registers.Pc);
                let indirect_address = self.memory.read::<u16, 2>(direct_address);
                Some(indirect_address)
            }
            AddressingMode::IndirectX => {
                let direct_address: u16 = self.read_address();
                let indirect_address = direct_address + self.registers.X as u16;
                Some(indirect_address)
            }
            AddressingMode::IndirectY => {
                let direct_address: u16 = self.read_address();
                let indirect_address = direct_address + self.registers.Y as u16;
                Some(indirect_address)
            }
            AddressingMode::Relative => {
                let offset: i8 = i8::from_twos_complement_bits(self.read_address::<u8, 1>());

                Some(self.registers.Pc.wrapping_add_signed(offset.into()))
            }
            _ => {
                None
            }
        }
    }


    pub fn step(&mut self) {
        let opcode: u8 = self.memory.read(self.registers.Pc);
        let current_instruction = &INSTRUCTIONS[opcode as usize];

        self.registers.Pc += 1;

        let operand = self.get_operand_for_instruction(current_instruction);

        self.execute_instruction(&current_instruction, operand); 
    }

    fn execute_instruction(&mut self, instruction: &Instruction, operand: Option<u16>) {
        todo!()
    }

    fn push_byte(&mut self, value: u8) {
        self.registers.Sp -= 1;

        if self.registers.Sp <= 0xFF {
            panic!("Stack overflow")
        }

        self.memory.write::<u8, 1>(self.registers.Sp, value);
    }

    fn push_short(&mut self, value: u16) {
        self.registers.Sp -= 2;

        if self.registers.Sp <= 0xFF {
            panic!("Stack overflow")
        }

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
