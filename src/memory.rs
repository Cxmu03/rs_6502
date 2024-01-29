use anyhow::Result;

pub trait Memory {
    fn new() -> Self;
    fn read_byte(&self, address: u16) -> u8;
    fn read_short(&self, address: u16) -> u16;

    fn write_byte(&mut self, address: u16, value: u8);
    fn write_short(&mut self, address: u16, value: u16);

    fn load_executable(&mut self, name: &str) -> Result<usize>;
}