pub struct Display {
    buffer: [bool; 64 * 32],
}

impl Display {
    pub fn new() -> Display {
        Display {
            buffer: [false; 64 * 32],
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
        for i in 0..8 {
            let bit = ((1 << (8-i-1)) & byte) > 0;
            self.buffer[(y * 64 + x + i) as usize] ^= bit;
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = false;
        }
    }
}
