// 1	2	3	C
// 4	5	6	D
// 7	8	9	E
// A	0	B	F

use rand::{thread_rng, Rng};

pub struct Keyboard {
    keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard { keys: [false; 16] }
    }

    pub fn is_pressed(&self, n: usize) -> bool {
        println!("checking for key {n:}");
        self.keys[n]
        // thread_rng().gen_bool(0.5)
        // false
    }

    pub fn get_pressed(&self) -> Option<u8> {
        for i in 0..16 {
            if self.keys[i] {
                return Some(i as u8);
            }
        }

        None
    }

    pub fn press(&mut self, n: usize) {
        self.keys[n] = true
    }

    pub fn release(&mut self, n: usize) {
        self.keys[n] = false
    }
}
