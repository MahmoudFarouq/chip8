// (0,0)	(63,0)
// (0,31)	(63,31)

use std::fmt::{Debug, Formatter};

pub struct Screen {
    pixels: [[u8; 64]; 32],
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            pixels: [[0; 64]; 32]
        }
    }

    pub fn clear(&mut self) {
        for i in 0..64 {
            for j in 0..32 {
                self.pixels[j][i] = 0;
            }
        }
    }

    pub fn set(&mut self, x: usize, y: usize, bit: u8) {
        self.pixels[y][x] = bit
    }

    pub fn get(&mut self, x: usize, y: usize) -> u8 {
        if self.pixels[y][x] == 0 {
            0
        } else {
            1
        }
    }
}

impl Debug for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut builder = String::new();

        for j in 0..32 {
            for i in 0..64 {
                builder += if self.pixels[j][i] == 0 { "0" } else { "1" };
            }

            builder += "\n"
        }

        write!(f, "{:}", builder)
    }
}