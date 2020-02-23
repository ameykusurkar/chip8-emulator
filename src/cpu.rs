use crate::display::Display;
use rand::Rng;

pub struct Cpu {
    memory: [u8; 4096],
    pc: u16,
    i: u16,
    regs: [u8; 16],
    display: Display,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: [0; 4096],
            pc: 0x200,
            i: 0,
            regs: [0; 16],
            display: Display::new(),
        }
    }

    pub fn load_binary(&mut self, binary: &Vec<u8>) {
        let start = 0x200;
        let binary_area = &mut self.memory[start..start+binary.len()];
        binary_area.copy_from_slice(binary);
    }

    pub fn display_buffer(&self) -> &[bool] {
        self.display.buffer()
    }

    pub fn cycle(&mut self) {
        let opcode = self.fetch_opcode();
        self.execute(opcode);
    }

    fn fetch_opcode(&self) -> u16 {
        let pc_idx = self.pc as usize;
        let opcode_bytes = &self.memory[pc_idx..pc_idx+2];
        (opcode_bytes[0] as u16) << 8 | opcode_bytes[1] as u16
    }

    fn execute(&mut self, opcode: u16) {
        // TODO: Only for debugging
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
                0x00EE => panic!("{:04x} not implemented!", opcode),
                _ => panic!("Unknown opcode {:04x}", opcode),
            },
            0x1000 => {
                // 1nnn - JP addr
                // Jump to location nnn.
                let addr = opcode & 0x0FFF;
                self.pc = addr;

                self.print_i(old, opcode, &format!("JMP {:04x}", addr));
            },
            0x2000 => panic!("{:04x} not implemented!", opcode),
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
            0x5000 => {
                // 5xy0 - SE Vx, Vy
                // Skip next instruction if Vx == Vy.
                let x_idx = ((opcode & 0x0F00) >> 8) as usize;
                let y_idx = ((opcode & 0x00F0) >> 4) as usize;

                let x = self.regs[x_idx] as u32;
                let y = self.regs[y_idx] as u32;

                if x == y {
                    self.pc += 2;
                }

                self.print_i(old, opcode, &format!("SE V{}, V{}", x_idx, y_idx));
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
            0x8000 => match opcode & 0xF00F {
                0x8000 => {
                    // 8xy0 - LD Vx, Vy
                    // Set Vx = Vy.
                    let x_idx = ((opcode & 0x0F00) >> 8) as usize;
                    let y_idx = ((opcode & 0x00F0) >> 4) as usize;

                    self.regs[x_idx] = self.regs[y_idx];

                    self.print_i(old, opcode, &format!("LD V{}, V{}", x_idx, y_idx));
                },
                0x8001 => panic!("{:04x} not implemented!", opcode),
                0x8002 => panic!("{:04x} not implemented!", opcode),
                0x8003 => panic!("{:04x} not implemented!", opcode),
                0x8004 => {
                    // 8xy4 - ADD Vx, Vy
                    // Set Vx = Vx + Vy, set VF = carry.
                    let x_idx = ((opcode & 0x0F00) >> 8) as usize;
                    let y_idx = ((opcode & 0x00F0) >> 4) as usize;

                    let x = self.regs[x_idx] as u32;
                    let y = self.regs[y_idx] as u32;
                    let result = x + y;

                    self.regs[0xF] = (result > 0xFF) as u8;
                    self.regs[x_idx] = result as u8;

                    self.print_i(old, opcode, &format!("ADD V{}, V{}", x_idx, y_idx));
                },
                0x8005 => {
                    // 8xy5 - SUB Vx, Vy
                    // Set Vx = Vx - Vy, set VF = NOT borrow.
                    let x_idx = ((opcode & 0x0F00) >> 8) as usize;
                    let y_idx = ((opcode & 0x00F0) >> 4) as usize;

                    let x = self.regs[x_idx] as u32;
                    let y = self.regs[y_idx] as u32;

                    self.regs[0xF] = (x > y) as u8;
                    self.regs[x_idx] = (x - y) as u8;

                    self.print_i(old, opcode, &format!("SUB V{}, V{}", x_idx, y_idx));
                },
                0x8006 => panic!("{:04x} not implemented!", opcode),
                0x8007 => panic!("{:04x} not implemented!", opcode),
                0x800E => panic!("{:04x} not implemented!", opcode),
                _ => panic!("Unknown opcode {:04x}", opcode),
            },
            0x9000 => {
                // 9xy0 - SNE Vx, Vy
                // Skip next instruction if Vx != Vy.
                let x_idx = ((opcode & 0x0F00) >> 8) as usize;
                let y_idx = ((opcode & 0x00F0) >> 4) as usize;

                let x = self.regs[x_idx] as u32;
                let y = self.regs[y_idx] as u32;

                if x != y {
                    self.pc += 2;
                }

                self.print_i(old, opcode, &format!("SNE V{}, V{}", x_idx, y_idx));
            },
            0xA000 => {
                // Annn - LD I, addr
                // Set I = nnn.
                let addr = opcode & 0x0FFF;
                self.i = addr;

                self.print_i(old, opcode, &format!("LD I, {:04x}", addr));
            },
            0xB000 => panic!("{:04x} not implemented!", opcode),
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
                0xE09E => panic!("{:04x} not implemented!", opcode),
                0xE0A1 => panic!("{:04x} not implemented!", opcode),
                _ => panic!("Unknown opcode {:04x}", opcode),
            },
            0xF000 => match opcode & 0xF0FF {
                0xF007 => panic!("{:04x} not implemented!", opcode),
                0xF00A => panic!("{:04x} not implemented!", opcode),
                0xF015 => panic!("{:04x} not implemented!", opcode),
                0xF018 => panic!("{:04x} not implemented!", opcode),
                0xF01E => {
                    // Fx1E - ADD I, Vx
                    // Set I = I + Vx.
                    let idx = (opcode & 0x0F00) >> 8;
                    self.i += self.regs[idx as usize] as u16;

                    self.print_i(old, opcode, &format!("ADD I, V{}", idx));
                },
                0xF029 => panic!("{:04x} not implemented!", opcode),
                0xF033 => panic!("{:04x} not implemented!", opcode),
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

    fn print_i(&self, pc: u16, opcode: u16, rep: &str) {
        println!("{:#03x}: ({:04x}) {}", pc, opcode, rep);
    }
}

// fn disassemble(opcode: u16) -> String {
//     match opcode & 0xF000 {
//         0x0000 => match opcode {
//             0x00E0 => "CLS",
//             0x00EE => "RET",
//             _ => "Unknown opcode",
//         },
//         0x1000 => "JP addr",
//         0x2000 => "CALL addr",
//         0x3000 => "SE Vx, byte",
//         0x4000 => "SNE Vx, byte",
//         0x5000 => "SE Vx, Vy",
//         0x6000 => "LD Vx, byte",
//         0x7000 => "ADD Vx, byte",
//         0x8000 => match opcode & 0xF00F {
//             0x8000 => "LD Vx, Vy",
//             0x8001 => "OR Vx, Vy",
//             0x8002 => "AND Vx, Vy",
//             0x8003 => "XOR Vx, Vy",
//             0x8004 => "ADD Vx, Vy",
//             0x8005 => "SUB Vx, Vy",
//             0x8006 => "SHR Vx {, Vy}",
//             0x8007 => "SUBN Vx, Vy",
//             0x800E => "SHL Vx {, Vy}",
//             _ => "Unknown opcode",
//         },
//         0x9000 => "SNE Vx, Vy",
//         0xA000 => "LD I, addr",
//         0xB000 => "JP V0, addr",
//         0xC000 => "RND Vx, byte",
//         0xD000 => "DRW Vx, Vy, nibble",
//         0xE000 => match opcode & 0xF0FF {
//             0xE09E => "SKP Vx",
//             0xE0A1 => "SKNP Vx",
//             _ => "Unknown opcode",
//         },
//         0xF000 => match opcode & 0xF0FF {
//             0xF007 => "LD Vx, DT",
//             0xF00A => "LD Vx, K",
//             0xF015 => "LD DT, Vx",
//             0xF018 => "LD ST, Vx",
//             0xF01E => "ADD I, Vx",
//             0xF029 => "LD F, Vx",
//             0xF033 => "LD B, Vx",
//             0xF055 => "LD [I], Vx",
//             0xF065 => "LD Vx, [I]",
//             _ => "Unknown opcode",
//         },
//         _ => "Not implemented!",
//     }.to_string()
// }
