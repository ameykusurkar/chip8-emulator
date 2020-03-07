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

    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
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

    pub fn get_keys_pressed(&mut self) -> Option<Vec<u8>> {
        self.window.get_keys()
            .map(|keys| keys.iter().filter_map(|k| Self::decode_key(&k)).collect())
            .filter(|keys: &Vec<u8>| !keys.is_empty())
    }

    fn decode_key(key: &minifb::Key) -> Option<u8> {
        match key {
            minifb::Key::Key1 => Some(0x1),
            minifb::Key::Key2 => Some(0x2),
            minifb::Key::Key3 => Some(0x3),
            minifb::Key::Key4 => Some(0xC),

            minifb::Key::Q => Some(0x4),
            minifb::Key::W => Some(0x5),
            minifb::Key::E => Some(0x6),
            minifb::Key::R => Some(0xD),

            minifb::Key::A => Some(0x7),
            minifb::Key::S => Some(0x8),
            minifb::Key::D => Some(0x9),
            minifb::Key::F => Some(0xE),

            minifb::Key::Z => Some(0xA),
            minifb::Key::X => Some(0x0),
            minifb::Key::C => Some(0xB),
            minifb::Key::V => Some(0xF),
            _ => None,
        }
    }
}
