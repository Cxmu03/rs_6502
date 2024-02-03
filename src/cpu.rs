use std::fmt::{Debug, Display, self, Formatter};

use indent::indent_all_by;
use anyhow::{Result};
use log;

use crate::registers::Registers;
use crate::instruction_table::INSTRUCTIONS;
use crate::memory::Memory;
use crate::instruction::{Instruction, AddressingMode};
use crate::util::FromTwosComplementBits;
use crate::default_memory::DefaultMemory;

#[derive(Debug)]
pub(crate) enum Operand {
    Byte(u8),
    Short(u16)
}

pub struct Cpu<Memory=DefaultMemory> {
    pub registers: Registers,
    pub memory: Memory,
    pub cycles: u32
}

impl Display for Cpu {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Cpu:\n")?;
        write!(f, "    cycles = {}\n\n", self.cycles);
        
        write!(f, "{}", indent_all_by(4, self.registers.to_string()));
        Ok(())
    }
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu {
            registers: Registers::new(),
            memory: Memory::new(),
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

    pub fn init_registers(&mut self) {
        self.registers.Pc = self.memory.read_short(0xFFFC);
        self.registers.Sp = 0x1FF;
    }

    pub fn load_executable(&mut self, bytes: &[u8]) -> Result<()> {
        let start = self.memory.load(bytes)?;

        self.set_reset_vector(start);

        Ok(())
    }

    pub fn load_executable_from_file(&mut self, file: &str) -> Result<()> {
        let start = self.memory.load_from_file(file)?;

        self.set_reset_vector(start);

        Ok(())
    }

    fn set_reset_vector(&mut self, address: u16) {
        self.memory.write_short(0xFFFC, address);
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

    fn get_operand_address(&self, instruction: &Instruction) -> Option<u16> {
        match instruction.mode {
            AddressingMode::Absolute => {
                Some(self.read_current_short())
            } 
            AddressingMode::AbsoluteX => {
                Some(self.read_current_short() + (self.registers.X as u16))
            }
            AddressingMode::AbsoluteY => {
                Some(self.read_current_short() + (self.registers.Y as u16))
            }
            AddressingMode::ZeroPage => {
                Some(self.read_current_byte() as u16)
            }
            AddressingMode::ZeroPageX => {
                Some(self.indexed_zero_page(self.registers.X))
            }
            AddressingMode::ZeroPageY => {
                Some(self.indexed_zero_page(self.registers.Y))
            }
            AddressingMode::Indirect => {
                let direct_address = self.memory.read_short(self.registers.Pc);
                let indirect_address = self.memory.read_short(direct_address);
                Some(indirect_address)
            }
            AddressingMode::IndirectX => {
                let direct_address: u16 = self.read_current_short();
                let indirect_address = direct_address + self.registers.X as u16;
                Some(indirect_address)
            }
            AddressingMode::IndirectY => {
                let direct_address: u16 = self.read_current_short();
                let indirect_address = direct_address + self.registers.Y as u16;
                Some(indirect_address)
            }
            AddressingMode::Relative => {
                let offset: i8 = i8::from_twos_complement_bits(self.read_current_byte());

                Some(self.registers.Pc.wrapping_add_signed(offset.into()))
            }
            AddressingMode::Immediate | AddressingMode::Accumulator | AddressingMode::Implied => {
                None
            }
        }
    }

    fn get_operand_value(&mut self, instruction: &Instruction) -> Option<u8> {
        match instruction.mode {
            AddressingMode::Implied => {
                None
            }
            AddressingMode::Immediate => {
                Some(self.read_current_byte())
            }
            AddressingMode::Accumulator => {
                Some(self.registers.Acc)
            }
            _ => {
                // Previous arms prevent get_operand_address from returning None
                let address = self.get_operand_address(instruction).unwrap();

                Some(self.memory.read_byte(address))
            }
        }
    }


    pub fn step(&mut self) {
        let opcode: u8 = self.read_current_byte();
        let current_instruction = &INSTRUCTIONS[opcode as usize];

        let operand = self.get_operand_value(current_instruction);

        self.registers.Pc += 1;

        log::debug!("Read instruction {:?} with opcode {:02X} and operand {:?}", current_instruction.instruction_type, current_instruction.opcode, operand);

        self.registers.Pc += current_instruction.mode.operand_size();

        self.execute_instruction(&current_instruction);

        self.cycles += current_instruction.cycles as u32;
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        (instruction.fun)(self);
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

    pub fn brk(&mut self) {
        todo!()
    }

    pub fn ora(&mut self) {
        todo!()
    }

    pub fn kil(&mut self) {
        todo!()
    }

    pub fn asl(&mut self) {
        todo!()
    }

    pub fn php(&mut self) {
        todo!()
    }

    pub fn bpl(&mut self) {
        todo!()
    }

    pub fn clc(&mut self) {
        todo!()
    }

    pub fn jsr(&mut self) {
        todo!()
    }

    pub fn and(&mut self) {
        todo!()
    }

    pub fn bit(&mut self) {
        todo!()
    }

    pub fn rol(&mut self) {
        todo!()
    }

    pub fn plp(&mut self) {
        todo!()
    }

    pub fn bmi(&mut self) {
        todo!()
    }

    pub fn sec(&mut self) {
        todo!()
    }

    pub fn rti(&mut self) {
        todo!()
    }

    pub fn eor(&mut self) {
        todo!()
    }

    pub fn lsr(&mut self) {
        todo!()
    }

    pub fn pha(&mut self) {
        todo!()
    }

    pub fn jmp(&mut self) {
        todo!()
    }

    pub fn bvc(&mut self) {
        todo!()
    }

    pub fn cli(&mut self) {
        todo!()
    }

    pub fn rts(&mut self) {
        todo!()
    }

    pub fn adc(&mut self) {
        todo!()
    }

    pub fn ror(&mut self) {
        todo!()
    }

    pub fn pla(&mut self) {
        todo!()
    }

    pub fn bvs(&mut self) {
        todo!()
    }

    pub fn sei(&mut self) {
        todo!()
    }

    pub fn sta(&mut self) {
        todo!()
    }

    pub fn sty(&mut self) {
        todo!()
    }

    pub fn stx(&mut self) {
        todo!()
    }

    pub fn dey(&mut self) {
        todo!()
    }

    pub fn txa(&mut self) {
        todo!()
    }

    pub fn bcc(&mut self) {
        todo!()
    }

    pub fn tya(&mut self) {
        todo!()
    }

    pub fn txs(&mut self) {
        todo!()
    }

    pub fn ldy(&mut self) {
        todo!()
    }

    pub fn lda(&mut self) {
        todo!()
    }

    pub fn ldx(&mut self) {
        todo!()
    }

    pub fn tay(&mut self) {
        todo!()
    }

    pub fn tax(&mut self) {
        todo!()
    }

    pub fn bcs(&mut self) {
        todo!()
    }

    pub fn clv(&mut self) {
        todo!()
    }

    pub fn tsx(&mut self) {
        todo!()
    }

    pub fn cpy(&mut self) {
        todo!()
    }

    pub fn cmp(&mut self) {
        todo!()
    }

    pub fn dec(&mut self) {
        todo!()
    }

    pub fn iny(&mut self) {
        todo!()
    }

    pub fn dex(&mut self) {
        todo!()
    }

    pub fn bne(&mut self) {
        todo!()
    }

    pub fn cld(&mut self) {
        todo!()
    }

    pub fn cpx(&mut self) {
        todo!()
    }

    pub fn sbc(&mut self) {
        todo!()
    }

    pub fn inc(&mut self) {
        todo!()
    }

    pub fn inx(&mut self) {
        todo!()
    }

    pub fn nop(&mut self) { }

    pub fn beq(&mut self) {
        todo!()
    }

    pub fn sed(&mut self) {
        todo!()
    }
}
