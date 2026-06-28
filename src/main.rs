struct Chip8 {
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    delay_timer: u8,
    sound_timer: u8,
    stack: Vec<u16>,
    keypad: [bool; 16],
    display: [bool; 64 * 32],
}

const FONTSET: [u8; 80] = [
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

impl Chip8 {
    fn new() -> Self {
        let mut chip8 = Chip8 {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 512,
            delay_timer: 0,
            sound_timer: 0,
            stack: Vec::new(),
            keypad: [false; 16],
            display: [false; 64 * 32],
        };

        chip8.memory[0x050..0x0A0].copy_from_slice(&FONTSET);

        chip8
    }

    fn load_rom(&mut self, pfad: &str) -> std::io::Result<()> {
        let rom = std::fs::read(pfad)?;

        let start = 0x200;
        self.memory[start..start + rom.len()].copy_from_slice(&rom);

        Ok(())
    }
}

fn main() {
    let mut chip8 = Chip8::new();
    println!("PC-Startwert: {}", chip8.pc);

    chip8.load_rom("2-ibm-logo.ch8").expect("ROM konnte nicht geladen werden");

    println!("Erstes ROM-Byte an 0x200: {:02X?}", &chip8.memory[0x200..0x202]);

}
