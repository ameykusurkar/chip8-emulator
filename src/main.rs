use std::fs::File;
use std::io::Read;

mod cpu;

use cpu::Cpu;

fn main() -> std::io::Result<()> {
    let path = std::env::args().nth(1);
    let mut f = File::open(path.unwrap())?;

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let mut cpu = Cpu::new();
    cpu.load_binary(&buffer);

    for _ in 0..buffer.len() / 2 {
        cpu.cycle();
    }

    Ok(())
}
