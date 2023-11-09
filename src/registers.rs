#[allow(non_snake_case)]
#[derive(Default)]
pub struct Flags {
    pub C: bool,
    pub Z: bool,
    pub I: bool,
    pub D: bool,
    pub B: bool,
    pub V: bool,
    pub N: bool
}

#[allow(non_snake_case)]
#[derive(Default)]
pub struct Registers {
    pub X: u8,
    pub Y: u8,
    pub Pc: u8,
    pub Acc: u16,
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
