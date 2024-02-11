use std::fs::File;
use std::io::Read;

use anyhow::{anyhow, Error, Result};

use crate::memory::Memory;

pub struct DefaultMemory {
    data: [u8; 1 << 16],
}

impl DefaultMemory {
    fn verify_executable(content_length: usize) -> Result<(usize, usize)> {
        let start = 0xFFFA as usize - content_length;
        let end = 0xFFFA;

        // 32k binary limit
        let max_binary_size = 1 << 15;

        if content_length > max_binary_size {
            return Err(anyhow!(
                "Binary size ({content_length}) exceeds maximum size of {max_binary_size}"
            ));
        }

        Ok((start, end))
    }
}

impl Memory for DefaultMemory {
    fn new() -> Self {
        let mut data: [u8; 1 << 16] = [0u8; 1 << 16];

        Self { data: data }
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

    fn load(&mut self, executable: &[u8]) -> Result<u16> {
        let (start, end) = Self::verify_executable(executable.len())?;

        self.data[start..end].copy_from_slice(executable);

        Ok(start as u16)
    }

    fn load_from_file(&mut self, name: &str) -> Result<u16> {
        let mut file = File::open(name)?;

        let (start, end) = Self::verify_executable(file.metadata()?.len() as usize)?;

        file.read(&mut self.data[start..end])?;

        Ok(start as u16)
    }
}
