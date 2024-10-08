mod instructions;

use std::fmt::{self, Debug, Display, Formatter};
use std::thread::current;

use anyhow::Result;
use indent::indent_all_by;
use log;

use crate::default_memory::DefaultMemory;
use crate::instruction::{AddressingMode, Instruction, InstructionType};
use crate::instruction_table::INSTRUCTIONS;
use crate::memory::Memory;
use crate::registers::{Flag, Flags, Registers};
use crate::util::{get_bit, FromTwosComplementBits};

#[derive(PartialEq, Debug)]
pub enum Voltage {
    Low,
    High,
}

#[derive(Debug)]
pub(crate) enum Operand {
    Byte(u8),
    Short(u16),
}

enum Interrupt {
    Maskable,
    NonMaskable,
}

pub struct Cpu<Memory = DefaultMemory> {
    pub registers: Registers,
    pub memory: Memory,
    pub cycles: u32,
    pub current_instruction: Option<&'static Instruction>,
    irq_line: Voltage,
    nmi_edge: bool,
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
        f.debug_struct("Cpu")
            .field("registers", &self.registers)
            .field("cycles", &self.cycles)
            .field("current_instruction", &self.current_instruction)
            .finish()
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu {
            registers: Registers::new(),
            memory: Memory::new(),
            cycles: 0,
            current_instruction: None,
            irq_line: Voltage::High,
            nmi_edge: false,
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
        self.registers.pc = self.memory.read_short(0xFFFC);
        self.registers.sp = 0xFF;
    }

    pub fn load_executable(&mut self, bytes: &[u8], address: u16) -> Result<()> {
        let start = self.memory.load(bytes, address)?;

        self.set_reset_vector(address);

        Ok(())
    }

    pub fn load_executable_from_file(&mut self, file: &str, address: u16) -> Result<()> {
        let start = self.memory.load_from_file(file, address)?;

        self.set_reset_vector(address);

        Ok(())
    }

    fn set_reset_vector(&mut self, address: u16) {
        self.memory.write_short(0xFFFC, address);
    }

    fn read_current_byte(&self) -> u8 {
        self.memory.read_byte(self.registers.pc)
    }

    fn read_current_short(&self) -> u16 {
        self.memory.read_short(self.registers.pc)
    }

    fn indexed_zero_page(&self, register: u8) -> u16 {
        let zero_page_address = self.memory.read_byte(self.registers.pc);

        zero_page_address.wrapping_add(register) as u16
    }

    fn get_operand_address(&self) -> Option<u16> {
        match self.current_instruction?.mode {
            AddressingMode::Absolute => Some(self.read_current_short()),
            AddressingMode::AbsoluteX => {
                Some(self.read_current_short() + (self.registers.x as u16))
            }
            AddressingMode::AbsoluteY => {
                Some(self.read_current_short() + (self.registers.y as u16))
            }
            AddressingMode::ZeroPage => Some(self.read_current_byte() as u16),
            AddressingMode::ZeroPageX => Some(self.indexed_zero_page(self.registers.x)),
            AddressingMode::ZeroPageY => Some(self.indexed_zero_page(self.registers.y)),
            AddressingMode::Indirect => {
                let direct_address = self.memory.read_short(self.registers.pc);
                let indirect_address = self.memory.read_short(direct_address);
                Some(indirect_address)
            }
            AddressingMode::IndirectX => {
                let direct_address: u16 = self.read_current_short();
                let indirect_address = direct_address + self.registers.x as u16;
                Some(indirect_address)
            }
            AddressingMode::IndirectY => {
                let direct_address: u16 = self.read_current_short();
                let indirect_address = direct_address + self.registers.y as u16;
                Some(indirect_address)
            }
            AddressingMode::Relative => {
                let offset: i8 = i8::from_twos_complement_bits(self.read_current_byte());

                Some(self.registers.pc.wrapping_add_signed(offset.into()))
            }
            AddressingMode::Immediate | AddressingMode::Accumulator | AddressingMode::Implied => {
                None
            }
        }
    }

    fn get_operand_value(&mut self) -> Option<u8> {
        match self.current_instruction?.mode {
            AddressingMode::Implied => None,
            AddressingMode::Immediate => Some(self.read_current_byte()),
            AddressingMode::Accumulator => Some(self.registers.a),
            _ => {
                // Previous arms prevent get_operand_address from returning None
                let address = self.get_operand_address()?;

                Some(self.memory.read_byte(address))
            }
        }
    }

    pub fn set_irq_line(&mut self, state: Voltage) {
        self.irq_line = state;
    }

    fn handle_interrupt(&mut self, typ: Interrupt) {
        let vector_location: u16 = match typ {
            Interrupt::Maskable => 0xFFFA,
            Interrupt::NonMaskable => 0xFFFE,
        };

        let isr_address = self.memory.read_short(vector_location);

        self.push_short(self.registers.pc);
        self.push_byte(self.registers.flags.0);

        self.cycles += 7;

        self.registers.pc = isr_address;
    }

    pub fn step(&mut self) {
        let opcode: u8 = self.read_current_byte();
        let current_instruction = &INSTRUCTIONS[opcode as usize];

        self.current_instruction = Some(current_instruction);

        let operand = self.get_operand_value();

        self.registers.pc += 1;

        log::trace!(
            "Executing instruction {:?} with opcode {:02X} ({:?}) and operand {:?}",
            current_instruction.instruction_type,
            current_instruction.opcode,
            current_instruction.mode,
            operand
        );

        if !current_instruction.is_jump() {
            self.registers.pc += current_instruction.mode.operand_size();
        }

        self.execute_instruction(&current_instruction);

        self.cycles += current_instruction.cycles as u32;

        if self.nmi_edge == true {
            self.handle_interrupt(Interrupt::NonMaskable);
            return;
        }

        if self.irq_line == Voltage::Low
            && self.registers.flags.get(Flag::InterruptDisable) == false
        {
            self.handle_interrupt(Interrupt::Maskable);
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        (instruction.fun)(self);
    }

    fn push_byte(&mut self, value: u8) {
        if self.registers.sp == 0 {
            panic!("Stack overflow");
        }

        self.registers.sp -= 1;

        self.memory.write_byte(self.registers.sp as u16 + 0x100, value);
    }

    fn push_short(&mut self, value: u16) {
        if self.registers.sp == 0 {
            panic!("Stack overflow");
        }

        self.registers.sp -= 2;

        self.memory.write_short(self.registers.sp as u16 + 0x100, value);
    }

    fn pop_byte(&mut self) -> u8 {
        if self.registers.sp == 0xFF {
            panic!("Stack underflow");
        }

        let value = self.memory.read_byte(self.registers.sp as u16 + 0x100);
        self.registers.sp += 1;

        value
    }

    fn pop_short(&mut self) -> u16 {
        if self.registers.sp >= 0xFE {
            panic!("Stack underflow");
        }

        let value = self.memory.read_short(self.registers.sp as u16 + 0x100);
        self.registers.sp += 2;

        value
    }

    fn branch_if(&mut self, condition: bool) {
        let new_pc = self.get_operand_address().expect("PC offset should be valid");

        if condition {
            self.registers.pc = new_pc;
        }
    }

    fn add_bcd(&mut self, value: u8) {
        let vh = u16::from(value >> 4);
        let vl = u16::from(value & 0xF);

        let al = u16::from(self.registers.a & 0xF);
        let ah = u16::from(self.registers.a >> 4);

        let carry = self.registers.flags.get(Flag::Carry) as u16;

        let mut sum_l: u16 = al + vl + carry;
        let mut sum_h: u16 = ah + vh;

        let mut carry_out = false;

        if sum_l >= 0xA {
            sum_l = (sum_l + 6) & 0xF;
            sum_h += 1;
        }

        if sum_h >= 0xA {
            sum_h = (sum_h + 6) & 0xF;
            carry_out = true;
        }

        let sum: u8 = ((sum_h as u8) << 4) | (sum_l as u8);
        let sum_binary: u8 = (u16::from(self.registers.a) + u16::from(value) + carry) as u8;

        let did_overflow = ((value ^ sum_binary) & (self.registers.a ^ sum_binary) & 0x80) != 0;

        self.registers.a = sum;

        self.registers.flags.set(Flag::Carry, carry_out);
        self.registers.flags.set(Flag::Overflow, did_overflow);
    }

    fn add_binary(&mut self, value: u8) {
        let value = u16::from(value);
        let acc_before = self.registers.a;
        let carry = u16::from(self.registers.flags.get(Flag::Carry));

        let mut carry_out = false;

        let sum = u16::from(self.registers.a) + value + carry;

        if sum > 255 {
            carry_out = true
        }

        self.registers.a = sum as u8;

        let did_overflow =
            (((value as u8) ^ self.registers.a) & (acc_before ^ self.registers.a) & 0x80) != 0;

        self.registers.flags.set(Flag::Carry, carry_out);
        self.registers.flags.set(Flag::Overflow, did_overflow);
    }

    fn sbc_bcd(&mut self, value: u8) {
        let vl = u8::from(value & 0xF);
        let vh = u8::from(value >> 4);

        let al = u8::from(self.registers.a & 0xF);
        let ah = u8::from(self.registers.a >> 4);

        let carry = 1 - u8::from(self.registers.flags.get(Flag::Carry));
        let mut carry_out = true;

        let mut sum_l = al.wrapping_sub(vl).wrapping_sub(carry) & 0xF;
        let mut sum_h = ah.wrapping_sub(vh) & 0xF;

        if sum_l > 0xA {
            sum_l -= 6;
            sum_h = sum_h.wrapping_sub(1) & 0xF;
        }

        if sum_h > 0xA {
            sum_h -= 6;
            carry_out = false;
        }

        let sum: u8 = (sum_h << 4) | sum_l;
        let sum_binary = self.registers.a.wrapping_sub(value).wrapping_sub(carry);

        let did_overflow = ((value ^ sum_binary) & (self.registers.a ^ sum_binary) & 0x80) != 0;

        self.registers.a = sum;

        self.registers.flags.set(Flag::Carry, carry_out);
        self.registers.flags.set(Flag::Overflow, did_overflow);
    }

    fn sbc_binary(&mut self, value: u8) {
        self.add_binary(!value);
    }

    fn replace_accumulator_or_memory_with_carry(&mut self, fun: impl Fn(u8, u8) -> (u8, bool)) {
        let value = self.get_operand_value().expect("Could not get operand value");
        let carry_in = self.registers.flags.get(Flag::Carry) as u8;

        let (value, carry) = fun(value, carry_in);

        if self.current_instruction.unwrap().mode == AddressingMode::Accumulator {
            self.registers.a = value;
        } else {
            let address = self.get_operand_address().unwrap();
            self.memory.write_byte(address, value);
        }

        self.registers.flags.set(Flag::Carry, carry);
        self.update_negative_flag(value);
        self.update_zero_flag(value);
    }

    fn compare_register_with_memory(&mut self, register: u8) {
        let a_before = self.registers.a;

        // Subtraction in cmp should always happen without carry
        self.registers.flags.set(Flag::Carry, true);

        self.registers.a = register;
        self.sbc();

        self.registers.a = a_before;
    }

    fn update_zero_flag(&mut self, value: u8) {
        self.registers.flags.set(Flag::Zero, value == 0);
    }

    fn update_negative_flag(&mut self, value: u8) {
        let flag = 0b10000000;
        let is_negative = (value & flag) == flag;

        self.registers.flags.set(Flag::Negative, is_negative);
    }
}
