use crate::cpu::{Cpu, Interrupt};
use crate::memory::Memory;
use crate::registers::{Flag, Flags};
use crate::util::get_bit;

impl Cpu {
    pub fn brk(&mut self) {
        self.registers.flags.set(Flag::Break, true);
        self.registers.pc += 1;
        self.handle_interrupt(Interrupt::Maskable);
    }

    pub fn ora(&mut self) {
        let operand = self.get_operand_value().expect("Could not get operand");

        self.registers.a = self.registers.a | operand;

        self.update_zero_flag(self.registers.a);
        self.update_negative_flag(self.registers.a);
    }

    pub fn kil(&mut self) {
        todo!()
    }

    pub fn asl(&mut self) {
        self.replace_accumulator_or_memory_with_carry(|value, _| (value << 1, value & 0x80 != 0))
    }

    pub fn php(&mut self) {
        let mut status = self.registers.flags;

        status.set(Flag::Break, true);

        self.push_byte(status.0);
    }

    pub fn bpl(&mut self) {
        self.branch_if(self.registers.flags.get(Flag::Negative) == false);
    }

    pub fn clc(&mut self) {
        self.registers.flags.set(Flag::Carry, false);
    }

    pub fn jsr(&mut self) {
        self.push_short(self.registers.pc + 1);

        self.branch_if(true);
    }

    pub fn and(&mut self) {
        let value = self.get_operand_value().expect("Could not get operand value");

        self.registers.a = self.registers.a & value;

        self.update_zero_flag(self.registers.a);
        self.update_negative_flag(self.registers.a);
    }

    pub fn bit(&mut self) {
        let value = self.get_operand_value().expect("Could not get operand value");

        self.registers.flags.set(Flag::Negative, get_bit(value, Flag::Negative as u8));
        self.registers.flags.set(Flag::Overflow, get_bit(value, Flag::Overflow as u8));

        self.update_zero_flag(value & self.registers.a);
    }

    pub fn rol(&mut self) {
        self.replace_accumulator_or_memory_with_carry(|value, carry_in| {
            ((value << 1) | carry_in, value & 0x80 != 0)
        })
    }

    pub fn plp(&mut self) {
        let current_break_status = self.registers.flags.get(Flag::Break);

        self.registers.flags = Flags(self.pop_byte());

        self.registers.flags.set(Flag::Break, current_break_status);
    }

    pub fn bmi(&mut self) {
        self.branch_if(self.registers.flags.get(Flag::Negative) == true);
    }

    pub fn sec(&mut self) {
        self.registers.flags.set(Flag::Carry, true);
    }

    pub fn rti(&mut self) {
        self.registers.flags = Flags(self.pop_byte());
        self.registers.flags.set(Flag::Break, false);
        self.registers.pc = self.pop_short();
    }

    pub fn eor(&mut self) {
        let value = self.get_operand_value().expect("Could not get operand value");

        self.registers.a = self.registers.a ^ value;

        self.update_zero_flag(self.registers.a);
        self.update_negative_flag(self.registers.a);
    }

    pub fn lsr(&mut self) {
        self.replace_accumulator_or_memory_with_carry(|value, _| (value >> 1, value & 1 == 1))
    }

    pub fn pha(&mut self) {
        self.push_byte(self.registers.a);
    }

    pub fn jmp(&mut self) {
        let new_pc = self.get_operand_address().expect("New address should be valid");

        self.registers.pc = new_pc;
    }

    pub fn bvc(&mut self) {
        self.branch_if(self.registers.flags.get(Flag::Overflow) == false);
    }

    pub fn cli(&mut self) {
        self.registers.flags.set(Flag::InterruptDisable, false);
    }

    pub fn rts(&mut self) {
        let return_address = self.pop_short() + 1;

        self.registers.pc = return_address;
    }

    pub fn adc(&mut self) {
        let value = self.get_operand_value().expect("Should get a valid operand");

        if self.registers.flags.get(Flag::Decimal) {
            self.add_bcd(value);
        } else {
            self.add_binary(value);
        }

        self.update_negative_flag(self.registers.a);
        self.update_zero_flag(self.registers.a);
    }

    pub fn ror(&mut self) {
        self.replace_accumulator_or_memory_with_carry(|value, carry_in| {
            ((value >> 1) | (carry_in << 7), value & 1 == 1)
        })
    }

    pub fn pla(&mut self) {
        self.registers.a = self.pop_byte();

        self.update_zero_flag(self.registers.a);
        self.update_negative_flag(self.registers.a);
    }

    pub fn bvs(&mut self) {
        self.branch_if(self.registers.flags.get(Flag::Overflow) == true);
    }

    pub fn sei(&mut self) {
        self.registers.flags.set(Flag::InterruptDisable, true);
    }

    pub fn sta(&mut self) {
        let address = self.get_operand_address().expect("Could not get operand");

        self.memory.write_byte(address, self.registers.a);
    }

    pub fn sty(&mut self) {
        let address = self.get_operand_address().expect("Could not get operand");

        self.memory.write_byte(address, self.registers.y);
    }

    pub fn stx(&mut self) {
        let address = self.get_operand_address().expect("Could not get operand");

        self.memory.write_byte(address, self.registers.y);
    }

    pub fn dey(&mut self) {
        self.registers.y = self.registers.y - 1;

        self.update_zero_flag(self.registers.y);
        self.update_negative_flag(self.registers.y);
    }

    pub fn txa(&mut self) {
        self.registers.a = self.registers.x;

        self.update_zero_flag(self.registers.a);
        self.update_negative_flag(self.registers.a);
    }

    pub fn bcc(&mut self) {
        self.branch_if(self.registers.flags.get(Flag::Carry) == false)
    }

    pub fn tya(&mut self) {
        self.registers.a = self.registers.y;

        self.update_zero_flag(self.registers.a);
        self.update_negative_flag(self.registers.a);
    }

    pub fn txs(&mut self) {
        self.registers.sp = self.registers.x;
    }

    pub fn ldy(&mut self) {
        let operand = self.get_operand_value().expect("Could not get operand");

        self.registers.y = operand;

        self.update_zero_flag(operand);
        self.update_negative_flag(operand);
    }

    pub fn lda(&mut self) {
        let operand = self.get_operand_value().expect("Could not get operand");

        self.registers.a = operand;

        self.update_zero_flag(operand);
        self.update_negative_flag(operand);
    }

    pub fn ldx(&mut self) {
        let operand = self.get_operand_value().expect("Could not get operand");

        self.registers.x = operand;

        self.update_zero_flag(operand);
        self.update_negative_flag(operand);
    }

    pub fn tay(&mut self) {
        self.registers.y = self.registers.a;

        self.update_zero_flag(self.registers.x);
        self.update_negative_flag(self.registers.x);
    }

    pub fn tax(&mut self) {
        self.registers.x = self.registers.a;

        self.update_zero_flag(self.registers.x);
        self.update_negative_flag(self.registers.x);
    }

    pub fn bcs(&mut self) {
        self.branch_if(self.registers.flags.get(Flag::Carry));
    }

    pub fn clv(&mut self) {
        self.registers.flags.set(Flag::Overflow, false);
    }

    pub fn tsx(&mut self) {
        self.registers.x = self.registers.sp;

        self.update_zero_flag(self.registers.x);
        self.update_negative_flag(self.registers.x);
    }

    pub fn cpy(&mut self) {
        self.compare_register_with_memory(self.registers.y);
    }

    pub fn cmp(&mut self) {
        self.compare_register_with_memory(self.registers.a);
    }

    pub fn dec(&mut self) {
        let address = self.get_operand_address().expect("Could not get operand address");

        let new_value = self.memory.read_byte(address) - 1;

        self.memory.write_byte(address, new_value);
        self.update_zero_flag(new_value);
        self.update_negative_flag(new_value);
    }

    pub fn iny(&mut self) {
        self.registers.y = self.registers.y + 1;

        self.update_zero_flag(self.registers.y);
        self.update_negative_flag(self.registers.y);
    }

    pub fn dex(&mut self) {
        self.registers.x = self.registers.x - 1;

        self.update_zero_flag(self.registers.x);
        self.update_negative_flag(self.registers.x);
    }

    pub fn bne(&mut self) {
        self.branch_if(self.registers.flags.get(Flag::Zero) == false);
    }

    pub fn cld(&mut self) {
        self.registers.flags.set(Flag::Decimal, false);
    }

    pub fn cpx(&mut self) {
        self.compare_register_with_memory(self.registers.x);
    }

    pub fn sbc(&mut self) {
        let value = self.get_operand_value().expect("Should get a valid operand");

        if self.registers.flags.get(Flag::Decimal) {
            self.sbc_bcd(value);
        } else {
            self.sbc_binary(value);
        }

        self.update_negative_flag(self.registers.a);
        self.update_zero_flag(self.registers.a);
    }

    pub fn inc(&mut self) {
        let address = self.get_operand_address().expect("Could not get operand address");

        let new_value = self.memory.read_byte(address) + 1;

        self.memory.write_byte(address, new_value);
        self.update_zero_flag(new_value);
        self.update_negative_flag(new_value);
    }

    pub fn inx(&mut self) {
        self.registers.x = self.registers.x + 1;

        self.update_zero_flag(self.registers.x);
        self.update_negative_flag(self.registers.x);
    }

    pub fn nop(&mut self) {}

    pub fn beq(&mut self) {
        self.branch_if(self.registers.flags.get(Flag::Zero) == true);
    }

    pub fn sed(&mut self) {
        self.registers.flags.set(Flag::Decimal, true);
    }
}