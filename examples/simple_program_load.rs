use rs_6502::cpu::Cpu;

use env_logger;

fn main() {
    env_logger::init();

    let program : &[u8]= &[0x02, 0x3C, 0x82, 0xEF, 0xF2, 0xFF];

    let mut cpu = Cpu::new();

    cpu.load_executable(program).expect("Could not load executable");
    cpu.init_registers();

    for _ in 0..program.len() {
        cpu.step();
    }
}