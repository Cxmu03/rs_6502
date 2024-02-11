use rs_6502::cpu::Cpu;
use rs_6502::memory::Memory;

fn main() {
    let mut cpu = Cpu::new();

    cpu.memory.write_byte(0x600, 0xCD);
    cpu.memory.write_byte(0x601, 0xAB);

    // Should print out 0xABCD
    println!("{:04X}", cpu.memory.read_short(0x600));
}
