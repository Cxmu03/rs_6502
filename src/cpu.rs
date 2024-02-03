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

        log::debug!("Read instruction {:?} with opcode {:02X} and operand {:?}", current_instruction.instruction_type, current_instruction.opcode, operand);

        self.registers.Pc += current_instruction.mode.operand_size();

        self.execute_instruction(&current_instruction, operand);

        self.cycles += current_instruction.cycles as u32;
    }

    fn execute_instruction(&mut self, instruction: &Instruction, operand: Option<Operand>) {
        (instruction.fun)(self, operand);
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

    pub fn nop(&mut self, operand: Option<Operand>) {}

    pub fn brk(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn ora(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn kil(&mut self, operand: Option<Operand>) { todo!() }

    pub fn asl(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn php(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn bpl(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn clc(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn jsr(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn and(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn bit(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn rol(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn plp(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn bmi(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn sec(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn rti(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn eor(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn lsr(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn pha(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn jmp(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn bvc(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn cli(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn rts(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn adc(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn ror(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn pla(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn bvs(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn sei(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn sta(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn sty(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn stx(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn dey(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn txa(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn bcc(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn tya(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn txs(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn ldy(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn lda(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn ldx(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn tay(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn tax(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn cxs(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn clv(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn tsx(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn cpy(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn cmp(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn dec(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn iny(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn dex(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn bne(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn cld(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn cpx(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn sbc(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn inc(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn inx(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn beq(&mut self, operand: Option<Operand>) {
        todo!()
    }

    pub fn sed(&mut self, operand: Option<Operand>) {
        todo!()
    }
}
