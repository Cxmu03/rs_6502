/*#[allow(non_snake_case)]
#[derive(Default)]
pub struct Flags {
    pub C: bool, // Carry Flag
    pub Z: bool, // Zero Flag
    pub I: bool, // Interrupt Disable
    pub D: bool, // Decimal Mode
    pub B: bool, // Break Command
    pub V: bool, // Overflow Flag
    pub N: bool  // Negative Flag
}*/

pub enum Flag {
    Negative = 7,
    Overflow = 6,
    B = 4,
    Decimal = 3,
    InterruptDisable = 2,
    Zero = 1,
    Carry = 0
}

pub struct Flags(u8);

impl Default for Flags {
    fn default() -> Flags {
        Flags(0b00100100) 
    }
}

impl Flags {
    fn set_bit(&mut self, index: u8, value: bool) {
        self.0 |= (value as u8) << index; 
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
    pub Sp: u16,    // Stack Pointer
    pub Acc: u8,    // Accumulator
    pub flags: Flags
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
