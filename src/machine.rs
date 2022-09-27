use crate::instructions::{Instruction};
use crate::keyboard::Keyboard;
use crate::screen::Screen;
use rand::random;

const PROGRAM_START_AT: usize = 0x200;
const TIMER_RATE: u64 = 16666; // 60 Hz

pub struct Machine {
    ram: [u8; 4098],
    registers: [u8; 16],
    register_i: u16,
    register_delay: u8,
    register_sound: u8,
    last_tick: std::time::Instant,
    pc: usize,
    sp: usize,
    stack: [u16; 16],
}

impl Machine {
    pub fn new() -> Self {
        let mut m = Machine {
            ram: [0; 4098],
            registers: [0; 16],
            register_i: 0,
            register_delay: 0,
            register_sound: 0,
            last_tick: std::time::Instant::now(),
            pc: PROGRAM_START_AT,
            sp: 0,
            stack: [0; 16],
        };

        m.ram[..(5 * 16)].copy_from_slice(&NUMBERS);

        m
    }

    pub fn load(&mut self, rom: &[u8]) {
        let start = self.pc;
        let end = start + rom.len();
        self.ram[start..end].copy_from_slice(rom);
    }

    pub fn step(&mut self, keyboard: &Keyboard, screen: &mut Screen) {
        let ins: u16 = ((self.ram[self.pc] as usize) << 8 | self.ram[self.pc + 1] as usize) as u16;

        self.pc += 2;

        let ins = Instruction::from(ins);

        match ins {
            Instruction::Sys(nnn) => {
                self.pc = nnn as usize;
            }
            Instruction::Cls => {
                screen.clear();
            }
            Instruction::Ret => {
                self.pc = self.stack[self.sp] as usize;
                self.sp -= 1;
            }
            Instruction::Jmp(nnn) => {
                self.pc = nnn as usize;
            }
            Instruction::Call(nnn) => {
                self.sp += 1;
                self.stack[self.sp] = self.pc as u16;
                self.pc = nnn as usize;
            }
            Instruction::SkipEq(x, kk) => {
                if self.registers[x as usize] == kk {
                    self.pc += 2;
                }
            }
            Instruction::SkipNEq(x, kk) => {
                if self.registers[x as usize] != kk {
                    self.pc += 2;
                }
            }
            Instruction::SkipEqV(x, y) => {
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.pc += 2;
                }
            }
            Instruction::Set(x, kk) => {
                self.registers[x as usize] = kk;
            }
            Instruction::Add(x, kk) => {
                // TODO: we will make it doesn't overflow just to pass the panic but this should work like this.
                let r = self.registers[x as usize];

                let (res, _overflowed) = r.overflowing_add(kk);
                // if overflowed {
                //     println!("overflowed");
                // }

                self.registers[x as usize] = res;
            }
            Instruction::Load(x, y) => {
                self.registers[x as usize] = self.registers[y as usize];
            }
            Instruction::Or(x, y) => self.registers[x as usize] |= self.registers[y as usize],
            Instruction::And(x, y) => self.registers[x as usize] &= self.registers[y as usize],
            Instruction::Xor(x, y) => self.registers[x as usize] ^= self.registers[y as usize],
            Instruction::AddCarry(x, y) => {
                let mut extended_x = self.registers[x as usize] as usize;

                extended_x += self.registers[y as usize] as usize;

                self.registers[0xf] = if extended_x > 0xff { 1 } else { 0 };

                self.registers[x as usize] = extended_x as u8;
            }
            Instruction::SubCarry(x, y) => {
                if self.registers[x as usize] > self.registers[y as usize] {
                    self.registers[0xf] = 1;
                } else {
                    self.registers[0xf] = 0;
                }

                // TODO: we will make it doesn't overflow just to pass the panic but this should work like this.
                (self.registers[x as usize], _) =
                    self.registers[x as usize].overflowing_sub(self.registers[y as usize]);
            }
            Instruction::Shr(x, _y) => {
                // TODO: what to do with Y??
                self.registers[0xf] = self.registers[x as usize] & 0x1;
                self.registers[x as usize] /= 2;
            }
            Instruction::SubN(x, y) => {
                if self.registers[y as usize] > self.registers[x as usize] {
                    self.registers[0xf] = 1;
                } else {
                    self.registers[0xf] = 0;
                }

                (self.registers[x as usize], _) = self.registers[y as usize].overflowing_sub(self.registers[x as usize]);
            }
            Instruction::Shl(x, _y) => {
                // TODO: what to do with Y??
                if (self.registers[x as usize] as usize) >> 0xf & 0x1 == 1 {
                    self.registers[0xf] = 1;
                } else {
                    self.registers[0xf] = 0;
                }
                (self.registers[x as usize], _) = self.registers[x as usize].overflowing_mul(2);
            }
            Instruction::Sne(x, y) => {
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.pc += 2;
                }
            }
            Instruction::LoadI(nnn) => {
                self.register_i = nnn;
            }
            Instruction::JmpV0(nnn) => {
                self.pc = self.registers[0] as usize + nnn as usize;
            }
            Instruction::Rnd(x, kk) => {
                self.registers[x as usize] = random_byte() & kk;
            }
            Instruction::Drw(x, y, n) => {
                let x = x as usize;
                let y = y as usize;
                let n = n as usize;

                self.registers[0xF] = 0;

                let sprite =
                    &self.ram[self.register_i as usize..(self.register_i as usize + n as usize)];
                for (i, byte) in sprite.iter().enumerate() {
                    let y = (self.registers[y] as usize + i) % 32;
                    for bit in 0..8 {
                        let x = (self.registers[x] as usize + bit) % 64;

                        let pixel = (byte >> (7 - bit)) & 1;

                        let old_pixel = screen.get(x, y);
                        self.registers[0x0F] |= pixel & old_pixel;
                        screen.set(x, y, old_pixel ^ pixel);
                    }
                }
            }
            Instruction::SkipPressed(x) => {
                if keyboard.is_pressed(self.registers[x as usize] as usize) {
                    self.pc += 2;
                }
            }
            Instruction::SkipNPressed(x) => {
                if !keyboard.is_pressed(self.registers[x as usize] as usize) {
                    self.pc += 2;
                }
            }
            Instruction::LoadDT(x) => self.registers[x as usize] = self.register_delay,
            Instruction::LoadKeyPress(x) => {
                if let Some(i) = keyboard.get_pressed() {
                    self.registers[x as usize] = i;
                } else {
                    // We will assume this call never happened, we will rollback
                    // the PC then return.
                    self.pc -= 2;
                }
            }
            Instruction::SetDT(x) => self.register_delay = self.registers[x as usize],
            Instruction::SetST(x) => self.register_sound = self.registers[x as usize],
            Instruction::AddI(x) => self.register_i += self.registers[x as usize] as u16,
            Instruction::LoadSprite(x) => {
                if self.registers[x as usize] > 15 {
                    panic!("Ooh!")
                }
                self.register_i = (self.registers[x as usize] * 5) as u16
            }
            Instruction::LoadBCD(x) => {
                let mut x = self.registers[x as usize];

                self.ram[self.register_i as usize] = x / 100;
                x %= 100;
                self.ram[self.register_i as usize + 1] = x / 10;
                x %= 10;
                self.ram[self.register_i as usize + 2] = x;
            }
            Instruction::LoadAllI(x) => {
                for i in 0..=(x as usize) {
                    self.ram[self.register_i as usize + i] = self.registers[i]
                }
            }
            Instruction::SetAllI(x) => {
                for i in 0..=(x as usize) {
                    self.registers[i] = self.ram[self.register_i as usize + i]
                }
            }
        };

        if self.last_tick.elapsed() >= std::time::Duration::from_micros(TIMER_RATE) {
            if self.register_delay > 0 {
                self.register_delay -= 1
            };
            if self.register_sound > 0 {
                self.register_sound -= 1
            };

            self.last_tick = std::time::Instant::now();
        }
    }
}

fn random_byte() -> u8 {
    random()
}

#[cfg(test)]
mod tests {
    use crate::keyboard::Keyboard;
    use crate::machine::Machine;
    use crate::screen::Screen;

    #[test]
    fn test_load_bcd() {
        let mut screen = Screen::new();
        let keyboard = Keyboard::new();
        let mut machine = Machine::new();

        machine.load(&[0xf4, 0x33]);
        machine.registers[4] = 235;

        machine.step(&keyboard, &mut screen);

        assert_eq!(machine.ram[machine.register_i as usize], 2);
        assert_eq!(machine.ram[machine.register_i as usize + 1], 3);
        assert_eq!(machine.ram[machine.register_i as usize + 2], 5);
    }
}

const NUMBERS: [u8; 5 * 16] = [
    // 0
    0b11110000,
    0b10010000,
    0b10010000,
    0b10010000,
    0b11110000,
    // 1
    0b00100000,
    0b01100000,
    0b00100000,
    0b00100000,
    0b01110000,
    // 2
    0b11110000,
    0b00010000,
    0b11110000,
    0b10000000,
    0b11110000,
    // 3
    0b11110000,
    0b00010000,
    0b11110000,
    0b00010000,
    0b11110000,
    // 4
    0b10010000,
    0b10010000,
    0b11110000,
    0b00010000,
    0b00010000,
    // 5
    0b11110000,
    0b10000000,
    0b11110000,
    0b00010000,
    0b11110000,
    // 6
    0b11110000,
    0b10000000,
    0b11110000,
    0b10010000,
    0b11110000,
    // 7
    0b11110000,
    0b00010000,
    0b00100000,
    0b01000000,
    0b01000000,
    // 8
    0b11110000,
    0b10010000,
    0b11110000,
    0b10010000,
    0b11110000,
    // 9
    0b11110000,
    0b10010000,
    0b11110000,
    0b00010000,
    0b11110000,
    // A
    0b11110000,
    0b10010000,
    0b11110000,
    0b10010000,
    0b10010000,
    // B
    0b11100000,
    0b10010000,
    0b11100000,
    0b10010000,
    0b11100000,
    // C
    0b11110000,
    0b10000000,
    0b10000000,
    0b10000000,
    0b11110000,
    // D
    0b11100000,
    0b10010000,
    0b10010000,
    0b10010000,
    0b11100000,
    // E
    0b11110000,
    0b10000000,
    0b11110000,
    0b10000000,
    0b11110000,
    // F
    0b11110000,
    0b10000000,
    0b11110000,
    0b10000000,
    0b10000000,
];
