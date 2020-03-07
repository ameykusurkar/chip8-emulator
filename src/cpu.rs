use crate::display::Display;
use rand::Rng;

pub struct Cpu {
    memory: [u8; 4096],
    pc: u16,
    i: u16,
    regs: [u8; 16],
    display: Display,
    delay_timer: u8,
    stack: [u16; 16],
    sp: u8,
    keyboard: [bool; 16],
    awaiting_key_press: bool,
    current_key_pressed: Option<u8>,
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu {
            memory: [0; 4096],
            pc: 0x200,
            i: 0,
            regs: [0; 16],
            display: Display::new(),
            delay_timer: 0,
            stack: [0; 16],
            sp: 0,
            keyboard: [false; 16],
            awaiting_key_press: false,
            current_key_pressed: None,
        };

        cpu.load_fontset();

        cpu
    }

    pub fn load_binary(&mut self, binary: &Vec<u8>) {
        let start = 0x200;
        let binary_area = &mut self.memory[start..start+binary.len()];
        binary_area.copy_from_slice(binary);
    }

    pub fn timer_interrupt(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }

    pub fn key_press_interrupt(&mut self, key: u8) {
        if self.awaiting_key_press {
            self.current_key_pressed = Some(key);
            self.awaiting_key_press = false;
        }
    }

    pub fn update_keyboard(&mut self, keys: &[u8]) {
        for k in &mut self.keyboard {
            *k = false;
        }

        for key in keys {
            self.keyboard[*key as usize] = true;
        }
    }

    pub fn display_buffer(&self) -> &[bool] {
        self.display.buffer()
    }

    // Returns true if display needs redrawing
    pub fn cycle(&mut self) -> bool {
        let opcode = self.fetch_opcode();
        self.execute(opcode);

        opcode & 0xF000 == 0xD000
    }

    fn load_fontset(&mut self) {
        let fontset = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        let fontset_area = &mut self.memory[0..fontset.len()];
        fontset_area.copy_from_slice(&fontset);
    }

    fn fetch_opcode(&self) -> u16 {
        let pc_idx = self.pc as usize;
        let opcode_bytes = &self.memory[pc_idx..pc_idx+2];
        (opcode_bytes[0] as u16) << 8 | opcode_bytes[1] as u16
    }

    fn execute(&mut self, opcode: u16) {
        if self.awaiting_key_press {
            return;
        }

        // Only used for debugging
        let old = self.pc;

        self.pc += 2;

        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => {
                    // 00E0 - CLS
                    // Clear the display.
                    self.display.clear();
                    self.print_i(old, opcode, "CLS");
                },
                0x00EE => {
                    // 00EE - RET
                    // Return from a subroutine.
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];

                    self.print_i(old, opcode, "RET");
                }
                _ => {
                    // This instruction is SYS nnn, which calls a subroutine
                    // only needed by older computers. Can be ignored.
                    let addr = opcode & 0x0FFF;
                    self.print_i(old, opcode, &format!("SYS {:04x}", addr));
                },
            },
            0x1000 => {
                // 1nnn - JP addr
                // Jump to location nnn.
                let addr = opcode & 0x0FFF;
                self.pc = addr;

                self.print_i(old, opcode, &format!("JMP {:04x}", addr));
            },
            0x2000 => {
                // 2nnn - CALL addr
                // Call subroutine at nnn.
                let addr = opcode & 0x0FFF;

                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = addr;

                self.print_i(old, opcode, &format!("CALL {:04x}", addr));
            },
            0x3000 => {
                // 3xkk - SE Vx, byte
                // Skip next instruction if Vx == kk.
                let idx = (opcode & 0x0F00) >> 8;
                let byte = opcode & 0x00FF;
                if self.regs[idx as usize] == byte as u8 {
                    self.pc += 2;
                }

                self.print_i(old, opcode, &format!("SE V{}, {:02x}", idx, byte));
            },
            0x4000 => {
                // 4xkk - SNE Vx, byte
                // Skip next instruction if Vx != kk.
                let idx = (opcode & 0x0F00) >> 8;
                let byte = opcode & 0x00FF;
                if self.regs[idx as usize] != byte as u8 {
                    self.pc += 2;
                }

                self.print_i(old, opcode, &format!("SNE V{}, {:02x}", idx, byte));
            },
            0x6000 => {
                // 6xkk - LD Vx, byte
                // Set Vx = kk.
                let idx = (opcode & 0x0F00) >> 8;
                let byte = opcode & 0x00FF;
                self.regs[idx as usize] = byte as u8;

                self.print_i(old, opcode, &format!("LD V{}, {:02x}", idx, byte));
            },
            0x7000 => {
                // 7xkk - ADD Vx, byte
                // Set Vx = Vx + kk.
                let idx = (opcode & 0x0F00) >> 8;
                let byte = opcode & 0x00FF;
                self.regs[idx as usize] += byte as u8;

                self.print_i(old, opcode, &format!("ADD V{}, {:02x}", idx, byte));
            }
            0x5000 | 0x8000 | 0x9000  => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;

                self.execute_two_reg_opcode(x, y, opcode, old);
            },
            0xA000 => {
                // Annn - LD I, addr
                // Set I = nnn.
                let addr = opcode & 0x0FFF;
                self.i = addr;

                self.print_i(old, opcode, &format!("LD I, {:04x}", addr));
            },
            0xB000 => {
                // Bnnn - JP V0, addr
                // Jump to location nnn + V0.
                let addr = opcode & 0x0FFF;
                self.pc = addr + self.regs[0] as u16;

                self.print_i(old, opcode, &format!("JP V0, {:04x}", addr));
            },
            0xC000 => {
                // Cxkk - RND Vx, byte
                // Set Vx = random byte AND kk.
                let idx = (opcode & 0x0F00) >> 8;
                let byte = (opcode & 0x00FF) as u8;
                let rand_byte: u8 = rand::thread_rng().gen();
                self.regs[idx as usize] = rand_byte & byte;

                self.print_i(old, opcode, &format!("RND V{}, {:02x}", idx, byte));
            },
            0xD000 => {
                // Dxyn - DRW Vx, Vy, nibble
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                let x_idx = ((opcode & 0x0F00) >> 8) as usize;
                let y_idx = ((opcode & 0x00F0) >> 4) as usize;
                let n = (opcode & 0x000F) as usize;

                let start = self.i as usize;
                let pixel_erased = self.display.draw(
                    self.regs[x_idx] as u32,
                    self.regs[y_idx] as u32,
                    &self.memory[start..start + n],
                );
                self.regs[0xF] = pixel_erased as u8;

                self.print_i(old, opcode, &format!("DRW V{}, V{}, {:x}", x_idx, y_idx, n));
            },
            0xE000 => match opcode & 0xF0FF {
                0xE09E => {
                    // Ex9E - SKP Vx
                    // Skip next instruction if key with the value of Vx is pressed.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let vx = self.regs[x] as usize;

                    if self.keyboard[vx] {
                        self.pc += 2;
                    }

                    self.print_i(old, opcode, &format!("SKP V{}", x));
                }
                0xE0A1 => {
                    // ExA1 - SKNP Vx
                    // Skip next instruction if key with the value of Vx is not pressed.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let vx = self.regs[x] as usize;

                    if !self.keyboard[vx] {
                        self.pc += 2;
                    }

                    self.print_i(old, opcode, &format!("SKNP V{}", x));
                }
                _ => panic!("Unknown opcode {:04x}", opcode),
            },
            0xF000 => match opcode & 0xF0FF {
                0xF007 => {
                    // Fx07 - LD Vx, DT
                    // Set Vx = delay timer value.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.regs[x] = self.delay_timer;

                    self.print_i(old, opcode, &format!("LD V{}, DT", x));
                },
                0xF00A => {
                    // Fx0A - LD Vx, K
                    // Wait for a key press, store the value of the key in Vx.

                    // Since this is a blocking instruction, we will loop on this
                    // instruction until a key is pressed. Meanwhile, the calling code
                    // is waiting looking for key presses, communicated via
                    // `key_press_interrupt()`. Hence when we hit this instruction,
                    // there are two possible scenarios:
                    //  1. This isn't the first iteration of the loop, and a key has
                    //     has already been pressed (`execute()` returns early if
                    //     there hasn't been a key press yet). In that case we can
                    //     reset the flag and progress.
                    //  2. This is the first iteration of the loop. In that case we need
                    //     to indicate that we need to loop, and rollback the program
                    //     counter to the current instruction so that we don't progress.
                    if let Some(key) = self.current_key_pressed {
                        let x = ((opcode & 0x0F00) >> 8) as usize;
                        self.regs[x] = key;
                        self.current_key_pressed = None;
                    } else {
                        self.awaiting_key_press = true;
                        self.pc -= 2;
                    }
                },
                0xF015 => {
                    // Fx15 - LD DT, Vx
                    // Set delay timer = Vx.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.delay_timer = self.regs[x];

                    self.print_i(old, opcode, &format!("LD DT V{}", x));
                },
                0xF018 => {
                    // Fx18 - LD ST, Vx
                    // Set sound timer = Vx.

                    // TODO: No-op for now. The current framebuffer library being
                    // used does not support audio output.
                },
                0xF01E => {
                    // Fx1E - ADD I, Vx
                    // Set I = I + Vx.
                    let idx = (opcode & 0x0F00) >> 8;
                    self.i += self.regs[idx as usize] as u16;

                    self.print_i(old, opcode, &format!("ADD I, V{}", idx));
                },
                0xF029 => {
                    // Fx29 - LD F, Vx
                    // Set I = location of sprite for digit Vx.
                    let x = ((opcode & 0x0F00) >> 8) as usize;

                    // Each digit's sprite is 5 bytes long
                    self.i = self.regs[x] as u16 * 5;

                    self.print_i(old, opcode, &format!("LD F, V{}", x));
                }
                0xF033 => {
                    // Fx33 - LD B, Vx
                    // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let vx = self.regs[x];
                    let i = self.i as usize;

                    self.memory[i] = vx / 100; // hundreds digit
                    self.memory[i + 1] = (vx / 10) % 10; // tens digit
                    self.memory[i + 2] = (vx % 100) % 10; // ones digit

                    self.print_i(old, opcode, &format!("LD B, V{}", x));
                },
                0xF055 => {
                    // Fx55 - LD [I], Vx
                    // Store registers V0 through Vx in memory starting at location I.
                    let idx = ((opcode & 0x0F00) >> 8) as usize;

                    for (offset, val) in self.regs[0..idx+1].iter().enumerate() {
                        self.memory[self.i as usize + offset] = *val;
                    }

                    self.print_i(old, opcode, &format!("LD [I], V{}", idx));
                },
                0xF065 => {
                    // Fx65 - LD Vx, [I]
                    // Read registers V0 through Vx from memory starting at location I.
                    let idx = ((opcode & 0x0F00) >> 8) as usize;

                    let start = self.i as usize;
                    for (i, val) in self.memory[start..start+idx+1].iter().enumerate() {
                        self.regs[i] = *val;
                    }

                    self.print_i(old, opcode, &format!("LD V{}, [I]", idx));
                },
                _ => panic!("Unknown opcode {:04x}", opcode),
            },
            _ => panic!("Unknown opcode {:04x}", opcode),
        };
    }

    fn execute_two_reg_opcode(&mut self, x: usize, y: usize, opcode: u16, old: u16) {
        match opcode & 0xF00F {
            0x5000 => {
                // 5xy0 - SE Vx, Vy
                // Skip next instruction if Vx == Vy.
                if self.regs[x] == self.regs[y] {
                    self.pc += 2;
                }

                self.print_i(old, opcode, &format!("SE V{}, V{}", x, y));
            },
            0x8000 => {
                // 8xy0 - LD Vx, Vy
                // Set Vx = Vy.
                self.regs[x] = self.regs[y];

                self.print_i(old, opcode, &format!("LD V{}, V{}", x, y));
            },
            0x8001 => {
                // 8xy1 - OR Vx, Vy
                // Set Vx = Vx OR Vy.
                self.regs[x] |= self.regs[y];

                self.print_i(old, opcode, &format!("OR V{}, V{}", x, y));
            },
            0x8002 => {
                // 8xy2 - AND Vx, Vy
                // Set Vx = Vx AND Vy.
                self.regs[x] &= self.regs[y];

                self.print_i(old, opcode, &format!("AND V{}, V{}", x, y));
            },
            0x8003 => {
                // 8xy3 - XOR Vx, Vy
                // Set Vx = Vx XOR Vy.
                self.regs[x] ^= self.regs[y];

                self.print_i(old, opcode, &format!("XOR V{}, V{}", x, y));
            }
            0x8004 => {
                // 8xy4 - ADD Vx, Vy
                // Set Vx = Vx + Vy, set VF = carry.
                let vx = self.regs[x] as u32;
                let vy = self.regs[y] as u32;
                let result = vx + vy;

                self.regs[0xF] = (result > 0xFF) as u8;
                self.regs[x] = result as u8;

                self.print_i(old, opcode, &format!("ADD V{}, V{}", x, y));
            },
            0x8005 => {
                // 8xy5 - SUB Vx, Vy
                // Set Vx = Vx - Vy, set VF = NOT borrow.
                let vx = self.regs[x];
                let vy = self.regs[y];

                // Set if NO borrow
                self.regs[0xF] = (vx >= vy) as u8;
                self.regs[x] -= vy;

                self.print_i(old, opcode, &format!("SUB V{}, V{}", x, y));
            },
            0x8006 => {
                // 8xy6 - SHR Vx {, Vy}
                // Set Vx = Vx SHR 1.
                self.regs[0xF] = (self.regs[x] & 0x0001) as u8;
                self.regs[x] >>= 1;

                self.print_i(old, opcode, &format!("SHR V{}", x));
            },
            0x8007 => {
                // 8xy7 - SUBN Vx, Vy
                // Set Vx = Vy - Vx, set VF = NOT borrow.
                let vx = self.regs[x];
                let vy = self.regs[y];

                // Set if NO borrow
                self.regs[0xF] = (vy >= vx) as u8;
                self.regs[x] = vy - vx;

                self.print_i(old, opcode, &format!("SUBN V{}, V{}", x, y));
            },
            0x800E => {
                // 8xyE - SHL Vx {, Vy}
                // Set Vx = Vx SHL 1.
                self.regs[0xF] = ((self.regs[x] & 0x80) == 0x80) as u8;
                self.regs[x] <<= 1;

                self.print_i(old, opcode, &format!("SHL V{}", x));
            },
            0x9000 => {
                // 9xy0 - SNE Vx, Vy
                // Skip next instruction if Vx != Vy.
                if self.regs[x] != self.regs[y] {
                    self.pc += 2;
                }

                self.print_i(old, opcode, &format!("SNE V{}, V{}", x, y));
            },
            _ => panic!("Unknown opcode {:04x}", opcode),
        }
    }

    fn print_i(&self, pc: u16, opcode: u16, rep: &str) {
        println!("{:#03x}: ({:04x}) {}", pc, opcode, rep);
    }
}
