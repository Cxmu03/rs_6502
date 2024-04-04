use std::convert::TryFrom;
use std::fmt::{self, Debug, Display, Formatter};

use indent::indent_all_by;

use crate::util::{get_bit, set_bit, toggle_bit};

#[derive(Debug, Copy, Clone)]
pub enum Flag {
    Negative = 7,
    Overflow = 6,
    Break = 4,
    Decimal = 3,
    InterruptDisable = 2,
    Zero = 1,
    Carry = 0,
}

impl TryFrom<u8> for Flag {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Flag::Carry),
            1 => Ok(Flag::Zero),
            2 => Ok(Flag::InterruptDisable),
            3 => Ok(Flag::Decimal),
            4 => Ok(Flag::Break),
            5 => Err(String::from("Flagbit 6 is always unused")),
            6 => Ok(Flag::Overflow),
            7 => Ok(Flag::Negative),
            _ => Err(format!("{value} is not a valid flagbit index")),
        }
    }
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone)]
pub struct Flags(pub u8);

impl Default for Flags {
    fn default() -> Flags {
        Flags(0b00100100)
    }
}

impl Display for Flags {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in (0_u8..=7).rev() {
            if let Ok(flag) = Flag::try_from(i) {
                write!(f, "{:<16} = {}\n", flag.to_string(), self.get(flag))?;
            }
        }
        Ok(())
    }
}

impl Debug for Flags {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Flags {
    pub fn get(&self, flag: Flag) -> bool {
        get_bit(self.0, flag as u8)
    }

    pub fn set(&mut self, flag: Flag, value: bool) {
        set_bit(self.0, flag as u8, value);
    }

    pub fn toggle(&mut self, flag: Flag) {
        toggle_bit(self.0, flag as u8);
    }
}

#[derive(Default)]
pub struct Registers {
    pub x: u8,   // X Index Register
    pub y: u8,   // Y Index Register
    pub pc: u16, // Program Counter
    pub sp: u8,  // Stack Pointer
    pub a: u8,   // Accumulator
    pub flags: Flags,
}

impl Display for Registers {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Registers:\n")?;
        write!(f, "    X   = 0x{0:02X}   = {0} \n", self.x)?;
        write!(f, "    Y   = 0x{0:02X}   = {0}\n", self.y)?;
        write!(f, "    PC  = 0x{0:04X} = {0}\n", self.pc)?;
        write!(f, "    SP  = 0x{0:02X} = {0}\n", self.sp)?;
        write!(f, "    ACC = 0x{0:02X}   = {0}\n\n", self.a)?;

        write!(f, "Flags:\n")?;
        write!(f, "{}", indent_all_by(4, self.flags.to_string()))?;

        Ok(())
    }
}

impl Debug for Registers {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Flags {
    pub fn new() -> Flags {
        Flags::default()
    }
}

impl Registers {
    pub fn new() -> Registers {
        Registers::default()
    }
}
