type U4 = u8;

type U12 = u16;

pub enum Instruction {
    /// 0nnn - SYS addr
    ///
    /// Jump to a machine code routine at nnn.
    Sys(U12),

    /// 00E0 - CLSP
    /// Clear the display.
    Cls,

    /// 00EE - RET
    /// Return from a subroutine.
    ///
    /// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    Ret,

    /// 1nnn - JP addr
    /// Jump to location nnn.
    ///
    /// The interpreter sets the program counter to nnn.
    Jmp(U12),

    /// 2nnn - CALL addr
    /// Call subroutine at nnn.
    ///
    /// The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    Call(U12),

    /// 3xkk - SE Vx, byte
    /// Skip next instruction if Vx = kk.
    ///
    /// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    SkipEq(U4, u8),

    /// 4xkk - SNE Vx, byte
    /// Skip next instruction if Vx != kk.
    ///
    /// The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    SkipNEq(U4, u8),

    /// 5xy0 - SE Vx, Vy
    /// Skip next instruction if Vx = Vy.
    ///
    /// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    SkipEqV(U4, U4),

    /// 6xkk - LD Vx, byte
    /// Set Vx = kk.
    ///
    /// The interpreter puts the value kk into register Vx.
    Set(U4, u8),

    /// 7xkk - ADD Vx, byte
    /// Set Vx = Vx + kk.
    ///
    /// Adds the value kk to the value of register Vx, then stores the result in Vx.
    Add(U4, u8),

    /// 8xy0 - LD Vx, Vy
    /// Set Vx = Vy.
    ///
    /// Stores the value of register Vy in register Vx.
    Load(U4, U4),

    /// 8xy1 - OR Vx, Vy
    /// Set Vx = Vx OR Vy.
    ///
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
    /// A bitwise OR compares the corresponding bits from two values, and if either bit is 1,
    /// then the same bit in the result is also 1. Otherwise, it is 0.
    Or(U4, U4),

    /// 8xy2 - AND Vx, Vy
    /// Set Vx = Vx AND Vy.
    ///
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
    /// A bitwise AND compares the corresponding bits from two values, and if both bits are 1,
    /// then the same bit in the result is also 1. Otherwise, it is 0.
    And(U4, U4),

    /// 8xy3 - XOR Vx, Vy
    /// Set Vx = Vx XOR Vy.
    ///
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
    /// An exclusive OR compares the corresponding bits from two values, and if the bits are not both the same,
    /// then the corresponding bit in the result is set to 1. Otherwise, it is 0.
    Xor(U4, U4),

    /// 8xy4 - ADD Vx, Vy
    /// Set Vx = Vx + Vy, set VF = carry.
    ///
    /// The values of Vx and Vy are added together.
    /// If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
    /// Only the lowest 8 bits of the result are kept, and stored in Vx.
    AddCarry(U4, U4),

    /// 8xy5 - SUB Vx, Vy
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    ///
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    SubCarry(U4, U4),

    /// 8xy6 - SHR Vx {, Vy}
    /// Set Vx = Vx SHR 1.
    ///
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    Shr(U4, U4),

    /// 8xy7 - SUBN Vx, Vy
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    ///
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    SubN(U4, U4),

    /// 8xyE - SHL Vx {, Vy}
    /// Set Vx = Vx SHL 1.
    ///
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    Shl(U4, U4),

    /// 9xy0 - SNE Vx, Vy
    /// Skip next instruction if Vx != Vy.
    ///
    /// The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    Sne(U4, U4),

    /// Annn - LD I, addr
    /// Set I = nnn.
    ///
    /// The value of register I is set to nnn.
    LoadI(U12),

    /// Bnnn - JP V0, addr
    /// Jump to location nnn + V0.
    ///
    /// The program counter is set to nnn plus the value of V0.
    JmpV0(U12),

    /// Cxkk - RND Vx, byte
    /// Set Vx = random byte AND kk.
    ///
    /// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk.
    /// The results are stored in Vx. See instruction 8xy2 for more information on AND.
    Rnd(U4, u8),

    /// Dxyn - DRW Vx, Vy, U4
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    ///
    /// The interpreter reads n bytes from memory, starting at the address stored in I.
    /// These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    /// Sprites are XORed onto the existing screen. If this causes any pixels to be erased,
    /// VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display,
    /// it wraps around to the opposite side of the screen.
    ///
    /// See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
    Drw(U4, U4, U4),

    /// Ex9E - SKP Vx
    /// Skip next instruction if key with the value of Vx is pressed.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
    SkipPressed(U4),

    /// ExA1 - SKNP Vx
    /// Skip next instruction if key with the value of Vx is not pressed.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    SkipNPressed(U4),

    /// Fx07 - LD Vx, DT
    /// Set Vx = delay timer value.
    ///
    /// The value of DT is placed into Vx.
    LoadDT(U4),

    /// Fx0A - LD Vx, K
    /// Wait for a key press, store the value of the key in Vx.
    ///
    /// All execution stops until a key is pressed, then the value of that key is stored in Vx.
    LoadKeyPress(U4),

    /// Fx15 - LD DT, Vx
    /// Set delay timer = Vx.
    ///
    /// DT is set equal to the value of Vx.
    SetDT(U4),

    /// Fx18 - LD ST, Vx
    /// Set sound timer = Vx.
    ///
    /// ST is set equal to the value of Vx.
    SetST(U4),

    /// Fx1E - ADD I, Vx
    /// Set I = I + Vx.
    ///
    /// The values of I and Vx are added, and the results are stored in I.
    AddI(U4),

    /// Fx29 - LD F, Vx
    /// Set I = location of sprite for digit Vx.
    ///
    /// The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.
    ///
    /// See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
    LoadSprite(U4),

    /// Fx33 - LD B, Vx
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    ///
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I,
    /// the tens digit at location I+1, and the ones digit at location I+2.
    LoadBCD(U4),

    /// Fx55 - LD [I], Vx
    /// Store registers V0 through Vx in memory starting at location I.
    ///
    /// The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    LoadAllI(U4),

    /// Fx65 - LD Vx, [I]
    /// Read registers V0 through Vx from memory starting at location I.
    ///
    /// The interpreter reads values from memory starting at location I into registers V0 through Vx.
    SetAllI(U4),
}

impl From<u16> for Instruction {
    fn from(ins: u16) -> Self {
        let t = u16_to_nibbles(ins);
        match t {
            (0x0, 0x0, 0xe, 0x0) => Instruction::Cls,
            (0x0, 0x0, 0xe, 0xe) => Instruction::Ret,
            (0x3, x, k1, k2) => Instruction::SkipEq(x, kk(k1, k2)),
            (0x4, x, k1, k2) => Instruction::SkipNEq(x, kk(k1, k2)),
            (0x5, x, y, 0x0) => Instruction::SkipEqV(x, y),
            (0x6, x, k1, k2) => Instruction::Set(x, kk(k1, k2)),
            (0x7, x, k1, k2) => Instruction::Add(x, kk(k1, k2)),
            (0x8, x, y, 0x0) => Instruction::Load(x, y),
            (0x8, x, y, 0x1) => Instruction::Or(x, y),
            (0x8, x, y, 0x2) => Instruction::And(x, y),
            (0x8, x, y, 0x3) => Instruction::Xor(x, y),
            (0x8, x, y, 0x4) => Instruction::AddCarry(x, y),
            (0x8, x, y, 0x5) => Instruction::SubCarry(x, y),
            (0x8, x, y, 0x6) => Instruction::Shr(x, y),
            (0x8, x, y, 0x7) => Instruction::SubN(x, y),
            (0x8, x, y, 0xe) => Instruction::Shl(x, y),
            (0x9, x, y, 0x0) => Instruction::Sne(x, y),
            (0xa, n1, n2, n3) => Instruction::LoadI(nnn(n1, n2, n3)),
            (0xb, n1, n2, n3) => Instruction::JmpV0(nnn(n1, n2, n3)),
            (0xc, x, k1, k2) => Instruction::Rnd(x, kk(k1, k2)),
            (0xd, x, y, n) => Instruction::Drw(x, y, n),
            (0xe, x, 0x9, 0xe) => Instruction::SkipPressed(x),
            (0xe, x, 0xa, 0x1) => Instruction::SkipNPressed(x),
            (0xf, x, 0x0, 0x7) => Instruction::LoadDT(x),
            (0xf, x, 0x0, 0xa) => Instruction::LoadKeyPress(x),
            (0xf, x, 0x1, 0x5) => Instruction::SetDT(x),
            (0xf, x, 0x1, 0x8) => Instruction::SetST(x),
            (0xf, x, 0x1, 0xe) => Instruction::AddI(x),
            (0xf, x, 0x2, 0x9) => Instruction::LoadSprite(x),
            (0xf, x, 0x3, 0x3) => Instruction::LoadBCD(x),
            (0xf, x, 0x5, 0x5) => Instruction::LoadAllI(x),
            (0xf, x, 0x6, 0x5) => Instruction::SetAllI(x),
            (0x0, n1, n2, n3) => Instruction::Sys(nnn(n1, n2, n3)),
            (0x1, n1, n2, n3) => Instruction::Jmp(nnn(n1, n2, n3)),
            (0x2, n1, n2, n3) => Instruction::Call(nnn(n1, n2, n3)),
            _ => unreachable!(),
        }
    }
}

fn u16_to_nibbles(n: u16) -> (U4, U4, U4, U4) {
    (
        (n >> 12) as U4,
        (n >> 8 & 0xf) as U4,
        (n >> 4 & 0xf) as U4,
        (n & 0xf) as U4,
    )
}

fn nnn(n1: U4, n2: U4, n3: U4) -> U12 {
    ((n1 as u16) << 8 | (n2 as u16) << 4 | n3 as u16) as U12
}

pub fn kk(k1: U4, k2: U4) -> u8 {
    k1 << 4 | k2
}

#[cfg(test)]
mod tests {
    use crate::instructions::{kk, nnn, u16_to_nibbles};

    #[test]
    fn test_u16_to_nibbles() {
        let r = u16_to_nibbles(0xfab4);
        assert_eq!(r, (0xf, 0xa, 0xb, 0x4));
    }

    #[test]
    fn test_nnn() {
        let r = nnn(0xf, 0xd, 0xe);
        assert_eq!(r, 0xfde);
    }

    #[test]
    fn test_kk() {
        let r = kk(0xf, 0xd);
        assert_eq!(r, 0xfd);
    }
}