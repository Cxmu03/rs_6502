use std::fs::File;
use std::io::Read;

use anyhow::{Result, Error, anyhow};

use crate::memory::Memory;

pub struct DefaultMemory {
    data: [u8; 1 << 16]
}

impl Memory for DefaultMemory {
    fn new() -> Self {
        let mut data: [u8; 1 << 16] = [0u8; 1 << 16];

        Self {
            data: data
        }
    }

    fn read_byte(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn read_short(&self, address: u16) -> u16 {
        u16::from_le_bytes([self.data[address as usize], self.data[address as usize + 1]])
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }

    fn write_short(&mut self, address: u16, value: u16) {
        self.data[address as usize] = (value & 0xFFFF) as u8;
        self.data[address as usize + 1] = (value >> 8) as u8;
    }

    fn load_executable(&mut self, name: &str) -> Result<usize>{
        let mut file = File::open(name)?;

        let len = file.metadata()?.len();
        let start = 0x200 as usize;
        let end = start + len as usize;

        // Vectors start at 0xFFFA and should not be overwritten with program code
        let max_binary_size: u64 = (0xFFFA_usize - start) as u64;

        if len > max_binary_size {
            return Err(anyhow!("Binary size ({len}) exceeds maximum size of {max_binary_size}"));
        }

        let read_size = file.read(&mut self.data[start..end])?;

        Ok(read_size)
    }
}