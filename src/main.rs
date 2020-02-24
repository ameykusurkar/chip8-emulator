use std::fs::File;
use std::io::Read;

use std::io::*;

mod cpu;
mod display;
mod window;

use cpu::Cpu;
use window::Window;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() -> std::io::Result<()> {
    let path = std::env::args().nth(1);
    let mut f = File::open(path.unwrap())?;

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let mut cpu = Cpu::new();
    cpu.load_binary(&buffer);

    let mut window = Window::new(WIDTH, HEIGHT);

    let mut last_update = std::time::Instant::now();

    loop {
        let should_redraw = cpu.cycle();

        // Just for now, while we print execution
        std::io::stdout().flush()?;
        std::thread::sleep(std::time::Duration::from_millis(10));

        // For some reason, the window freezes if it has not been updated in
        // a while, so make sure that it is updated at least every 100ms.
        if should_redraw || last_update.elapsed().as_millis() > 100 {
            window.update(cpu.display_buffer());
            last_update = std::time::Instant::now();
        }
    }
}
