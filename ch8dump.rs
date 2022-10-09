use std::env;

  struct Nibbles {
      nn: u8,
      n: u8,
      id: u8,
  }

fn main(){
    let args: Vec<String> = env::args().collect();
    let rom_path = &args[1];
    let rom_file = std::fs::read(rom_path.to_string()).unwrap();
    for (i, command) in rom_file.chunks_exact(2).enumerate(){
        let instruction: u16 = ((command[0] as u16) << 8) + command[1] as u16;
        let nb = Nibbles {
              nn: (instruction & 0x00ff) as u8,
              n: (instruction & 0x000f) as u8,
              id: ((instruction & 0xf000) >> 12) as u8,
          };
        let mnemonic = match nb.id {
              0 => match nb.nn {
                  0xE0 => "CLS",
                  0xEE => "RET",
                  _ => "Unknown instruction",
              },
              1 => "JP addr",
              2 => "CALL addr",
              3 => "SE Vx, byte",
              4 => "SNE Vx, byte",
              5 => "SE Vx, Vy",
              6 => "LD Vx, byte",
              7 => "ADD Vx, byte",
              8 => match nb.n {
                  0 => "LD Vx, Vy",
                  1 => "OR Vx, Vy",
                  2 => "AND Vx, Vy",
                  3 => "XOR Vx, Vy",
                  4 => "ADD Vx, Vy",
                  5 => "SUB Vx, Vy",
                  6 => "SHR Vx, {, Vy}",
                  7 => "SUBN Vx, Vy",
                  0xE => "SHL Vx, {, Vy}",
                  _ => "Uknown instruction",
              },
              9 => "SNE Vx, Vy",
              0xA => "LD I, addr",
              0xB => "JP V0, addr",
              0xC => "RND Vx, byte",
              0xD => "DRW Vx, Vy, nibble **IMPORTANT**",
              0xE => match nb.nn {
                  0x9E => "SKP Vx",
                  0xA1 => "SKNP Vx",
                  _ => "Unknown instruction",
              },
              0xF => match nb.nn {
                  7 => "LD Vx, DT",
                  0xA => "LD Vx, K (scan keycode)",
                  0x15 => "LD DT, Vx",
                  0x18 => "LD ST, Vx",
                  0x1E => "ADD I, Vx",
                  0x29 => "LD F, Vx (font chooser)",
                  0x33 => "LD B, Vx (BCD)",
                  0x55 => "LD [I], Vx",
                  0x65 => "LD Vx, [I]",
                  _ => "Uknown instruction",
              },
              _ => "Unknown instruction",
        }; 
        println!("{}:{}  {:X?}", i, mnemonic, instruction); 
    }
}
