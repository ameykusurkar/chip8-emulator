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

// in Hz
const CLOCK_SPEED: u32 = 540;
const REFRESH_RATE: u32 = 60;

fn main() -> std::io::Result<()> {
    let path = std::env::args().nth(1).unwrap();
    let mut f = File::open(&path)?;

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let mut cpu = Cpu::new();
    cpu.load_binary(&buffer);

    let mut window = Window::new(WIDTH, HEIGHT);
    window.set_title(&format!("CHIP-8 ({})", path));

    let redraw_interval = (1000.0 / REFRESH_RATE as f64) as u64;
    let cycles_per_refresh = CLOCK_SPEED / REFRESH_RATE;

    while window.is_open() {
        let now = std::time::Instant::now();

        if let Some(keys) = window.get_keys_pressed() {
            cpu.update_keyboard(&keys);
            cpu.key_press_interrupt(keys[0]);
        } else {
            cpu.update_keyboard(&[]);
        }

        for _ in 0..cycles_per_refresh {
            cpu.cycle();
        }

        let cpu_elapsed = now.elapsed().as_millis() as u64;

        // Just for now, while we print execution
        std::io::stdout().flush()?;

        std::thread::sleep(std::time::Duration::from_millis(redraw_interval - cpu_elapsed));
        cpu.timer_interrupt();
        window.update(cpu.display_buffer());
    }

    Ok(())
}
