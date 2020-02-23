use std::fs::File;
use std::io::Read;

use std::io::*;

mod cpu;
mod display;
mod window;

use cpu::Cpu;
use window::Window;

fn main() -> std::io::Result<()> {
    let path = std::env::args().nth(1);
    let mut f = File::open(path.unwrap())?;

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let mut cpu = Cpu::new();
    cpu.load_binary(&buffer);

    let mut window = Window::new(64, 32, 10);

    loop {
        cpu.cycle();

        // Just for now, while we print execution
        std::io::stdout().flush()?;
        std::thread::sleep(std::time::Duration::from_millis(10));

        window.update(cpu.display_buffer());
    }
}
