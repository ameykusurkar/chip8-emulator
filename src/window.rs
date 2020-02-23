use minifb;

pub struct Window {
    window: minifb::Window,
    width: usize,
    height: usize,
}

impl Window {
    pub fn new(width: usize, height: usize) -> Window {
        let mut window = minifb::Window::new(
            "", width, height, minifb::WindowOptions::default()
        ).unwrap();
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Window { window, width, height }
    }

    pub fn update(&mut self, buffer: &[bool]) {
        let buffer: Vec<u32> = buffer.iter().
            map(|pixel| if *pixel { 0x00FFFFFF } else { 0 }).
            collect();

        self.window.update_with_buffer(&buffer, self.width, self.height).unwrap();
    }
}