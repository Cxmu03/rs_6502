#[allow(non_snake_case)]
#[derive(Default)]
pub struct Flags {
    pub C: bool, // Carry Flag
    pub Z: bool, // Zero Flag
    pub I: bool, // Interrupt Disable
    pub D: bool, // Decimal Mode
    pub B: bool, // Break Command
    pub V: bool, // Overflow Flag
    pub N: bool  // Negative Flag
}

#[allow(non_snake_case)]
#[derive(Default)]
pub struct Registers {
    pub X: u8,      // X Index Register
    pub Y: u8,      // Y Index Register
    pub Pc: u16,     // Program Counter
    pub Acc: u8,   // Accumulator
    pub flags: Flags
}

impl Flags {
    fn new() -> Flags {
        Flags::default()
    }
}

impl Registers {
    fn new() -> Registers {
        Registers::default()
    }
}
