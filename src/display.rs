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

    // Returns true if any pixel was un-set while drawing
    pub fn draw(&mut self, x: u32, y: u32, sprite: &[u8]) -> bool {
        let erased_pixels: Vec<bool> = sprite.iter()
            .enumerate()
            .map(|(offset, byte)| self.draw_byte(x, y + (offset as u32), *byte))
            .collect();

        erased_pixels.iter().any(|x| *x)
    }

    // Returns true if any pixel was un-set while drawing
    fn draw_byte(&mut self, x: u32, y: u32, byte: u8) -> bool {
        let mut erased = false;

        let j = y % HEIGHT as u32;
        for n in 0..8 {
            let bit = ((1 << (8-n-1)) & byte) > 0;
            let i = (x + n) % WIDTH as u32;
            let idx = (j * (WIDTH as u32) + i) as usize;

            // If pixel was already set, this will cause it to be erased
            erased |= self.buffer[idx] & bit;

            self.buffer[idx] ^= bit;
        }

        erased
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = false;
        }
    }
}
