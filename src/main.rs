use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let path = std::env::args().nth(1);
    let mut f = File::open(path.unwrap())?;

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let mut address = 0x200;

    for bytes in buffer.chunks(2) {
        let opcode = u8_to_u16(bytes[0], bytes[1]);
        println!("{:#03x}: ({:04x}) {}", address, opcode, disassemble(opcode));
        address += 2;
    }

    Ok(())
}

fn u8_to_u16(x: u8, y: u8) -> u16 {
    (x as u16) << 8 | y as u16
}

fn disassemble(opcode: u16) -> String {
    match opcode & 0xF000 {
        0x0000 => match opcode {
            0x00E0 => "CLS",
            0x00EE => "RET",
            _ => "Unknown opcode",
        },
        0x1000 => "JP addr",
        0x2000 => "CALL addr",
        0x3000 => "SE Vx, byte",
        0x4000 => "SNE Vx, byte",
        0x5000 => "SE Vx, Vy",
        0x6000 => "LD Vx, byte",
        0x7000 => "ADD Vx, byte",
        0x8000 => match opcode & 0xF00F {
            0x8000 => "LD Vx, Vy",
            0x8001 => "OR Vx, Vy",
            0x8002 => "AND Vx, Vy",
            0x8003 => "XOR Vx, Vy",
            0x8004 => "ADD Vx, Vy",
            0x8005 => "SUB Vx, Vy",
            0x8006 => "SHR Vx {, Vy}",
            0x8007 => "SUBN Vx, Vy",
            0x800E => "SHL Vx {, Vy}",
            _ => "Unknown opcode",
        },
       0x9000 => "SNE Vx, Vy",
       0xA000 => "LD I, addr",
       0xB000 => "JP V0, addr",
       0xC000 => "RND Vx, byte",
       0xD000 => "DRW Vx, Vy, nibble",
       0xE000 => match opcode & 0xF0FF {
           0xE09E => "SKP Vx",
           0xE0A1 => "SKNP Vx",
            _ => "Unknown opcode",
       },
       0xF000 => match opcode & 0xF0FF {
           0xF007 => "LD Vx, DT",
           0xF00A => "LD Vx, K",
           0xF015 => "LD DT, Vx",
           0xF018 => "LD ST, Vx",
           0xF01E => "ADD I, Vx",
           0xF029 => "LD F, Vx",
           0xF033 => "LD B, Vx",
           0xF055 => "LD [I], Vx",
           0xF065 => "LD Vx, [I]",
           _ => "Unknown opcode",
       },
       _ => "Not implemented!",
    }.to_string()
}
