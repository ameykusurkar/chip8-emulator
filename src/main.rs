use std::fs::File;
use std::io::Read;

use std::io::*;

mod cpu;

use cpu::Cpu;

fn main() -> std::io::Result<()> {
    let path = std::env::args().nth(1);
    let mut f = File::open(path.unwrap())?;

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let mut cpu = Cpu::new();
    cpu.load_binary(&buffer);

    loop {
        cpu.cycle();

        // Just for now, while we print execution
        std::io::stdout().flush()?;
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
