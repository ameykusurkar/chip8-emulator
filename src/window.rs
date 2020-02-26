use minifb;

pub struct Window {
    window: minifb::Window,
    width: usize,
    height: usize,
}

impl Window {
    pub fn new(width: usize, height: usize) -> Window {
        let mut options = minifb::WindowOptions::default();
        options.scale = minifb::Scale::X8;

        let window = minifb::Window::new(
            "",
            width,
            height,
            options,
        ).unwrap();

        Window { window, width, height }
    }

    pub fn update(&mut self, buffer: &[bool]) {
        let buffer: Vec<u32> = buffer.iter()
            .map(|x| if *x { 0x00ECF0F1 } else { 0 })
            .collect();

        self.window
            .update_with_buffer(&buffer, self.width, self.height)
            .unwrap();
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
}
