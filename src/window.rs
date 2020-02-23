use minifb;

pub struct Window {
    window: minifb::Window,
    width: usize,
    height: usize,
    pixel_width: usize,
}

impl Window {
    pub fn new(width: usize, height: usize, pixel_width: usize) -> Window {
        let window = minifb::Window::new(
            "",
            width * pixel_width,
            height * pixel_width,
            minifb::WindowOptions::default()
        ).unwrap();

        Window { window, width, height, pixel_width }
    }

    pub fn update(&mut self, buffer: &[bool]) {
        let window_width = self.width * self.pixel_width;
        let window_height = self.height * self.pixel_width;

        let mut window_buffer = vec![0; window_width * window_height];

        for (idx, pixel) in buffer.iter().enumerate() {
            for i in 0..self.pixel_width {
                for j in 0..self.pixel_width {
                    let x = (idx % self.width) * self.pixel_width + i;
                    let y = idx / self.width * self.pixel_width + j;
                    if *pixel {
                        window_buffer[y * window_width + x] = 0x00ECF0F1;
                    }
                }
            }
        }

        self.window
            .update_with_buffer(&window_buffer, window_width, window_height)
            .unwrap();
    }
}
