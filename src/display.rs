use crate::{WIDTH, HEIGHT};

pub struct Display {
    buffer: [bool; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            buffer: [false; WIDTH * HEIGHT],
        }
    }

    pub fn buffer(&self) -> &[bool] {
        &self.buffer
    }

    pub fn draw(&mut self, x: u32, y: u32, sprite: &[u8]) {
        for (offset, byte) in sprite.iter().enumerate() {
            self.draw_byte(x, y + (offset as u32), *byte);
        }
    }

    fn draw_byte(&mut self, x: u32, y: u32, byte: u8) {
        let j = y % HEIGHT as u32;
        for n in 0..8 {
            let bit = ((1 << (8-n-1)) & byte) > 0;
            let i = (x + n) % WIDTH as u32;
            self.buffer[(j * (WIDTH as u32) + i) as usize] ^= bit;
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = false;
        }
    }
}
