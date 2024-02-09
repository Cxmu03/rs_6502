use std::fmt::{Display, Debug, Formatter, self};
use std::convert::TryFrom;

use indent::indent_all_by;

#[derive(Debug, Copy, Clone)]
pub enum Flag {
    Negative = 7,
    Overflow = 6,
    Break = 4,
    Decimal = 3,
    InterruptDisable = 2,
    Zero = 1,
    Carry = 0
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
            _ => Err(format!("{value} is not a valid flagbit index"))
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
    pub fn set_bit(&mut self, index: u8, value: bool) {
        let mask = (value as u8) << index;

        if (self.0 & 1 << index) != mask {
            self.toggle_bit(index);
        }
    }

    fn toggle_bit(&mut self, index: u8) {
        self.0 ^= 1 << index;
    }

    fn get_bit(&self, index: u8) -> bool {
        self.0 & (1 << index) == (1 << index)
    }

    pub fn get(&self, flag: Flag) -> bool {
        self.get_bit(flag as u8)
    }

    pub fn set(&mut self, flag: Flag, value: bool) {
        self.set_bit(flag as u8, value);
    }

    pub fn toggle(&mut self, flag: Flag) {
        self.toggle_bit(flag as u8);
    }
}

#[allow(non_snake_case)]
#[derive(Default)]
pub struct Registers {
    pub X: u8,      // X Index Register
    pub Y: u8,      // Y Index Register
    pub Pc: u16,    // Program Counter
    pub Sp: u8,    // Stack Pointer
    pub Acc: u8,    // Accumulator
    pub flags: Flags
}

impl Display for Registers {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Registers:\n")?;
        write!(f, "    X   = 0x{0:02X}   = {0} \n", self.X)?;
        write!(f, "    Y   = 0x{0:02X}   = {0}\n", self.Y)?;
        write!(f, "    PC  = 0x{0:04X} = {0}\n", self.Pc)?;
        write!(f, "    SP  = 0x{0:02X} = {0}\n", self.Sp)?;
        write!(f, "    ACC = 0x{0:02X}   = {0}\n\n", self.Acc)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_bit() {
        let value = 0b00010000;
        let mut flags = Flags(value);

        flags.set_bit(4, true);
        assert_eq!(flags.0, value);

        flags.set_bit(4, false);
        assert_eq!(flags.0, 0u8);

        flags.set_bit(4, false);
        assert_eq!(flags.0, 0u8);

        flags.set_bit(4, true);
        assert_eq!(flags.0, value);
    }
}