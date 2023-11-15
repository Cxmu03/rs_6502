use anyhow::{Result, Error, anyhow};

pub struct Memory {
    pub sp: u16,      // Stack pointer
    data: [u8; 65536] // 64kb of ram
}

impl Memory {
    pub fn address_in_rom(&self, address: u16) -> bool {
        false
    }
        
    pub fn read_byte(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn read_short(&self, address: u16) -> u16 {
        ((self.data[(address + 1) as usize] as u16) << 8) | (self.data[address as usize] as u16)  
    }

    pub fn write_byte(&mut self, address: u16, value: u8) -> Result<(), Error> {
        if self.address_in_rom(address) {
            return Err(anyhow!("Trying to access ROM"));
        }

        self.data[address as usize] = value;
        Ok(())
    }

    pub fn write_short(&mut self, address: u16, value: u16) -> Result<(), Error> {
        if self.address_in_rom(address) {
            return Err(anyhow!("Trying to access ROM"));
        }

        self.data[address as usize] = (value & 0xFF) as u8;
        self.data[(address + 1) as usize] =  (value >> 8) as u8;
        Ok(())
    }

    pub fn push_byte(&mut self, value: u8) {
        self.sp -= 1;
        self.write_byte(self.sp, value).expect("Stack is going into ROM, aborting");
    }

    pub fn push_short(&mut self, value: u16) {
        self.sp -= 2;
        self.write_short(self.sp, value).expect("Stack is going into ROM, aborting");
    }

    pub fn pop_byte(&mut self, address: u16) -> u8 {
        let value = self.read_byte(address);
        self.sp += 1;
        
        value
    }

    pub fn pop_short(&mut self, address: u16) -> u16 {
        let value = self.read_short(address);
        self.sp += 2;

        value

    }
}
