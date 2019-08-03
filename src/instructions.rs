/**
 *
 * NNN: address,
 * NN: 8bit constant,
 * N: 4bit constant,
 * X and Y: 4bit register identifier,
 * PC: Program Counter
 * I: 16bit register
 * VN: One of the 16 available variables. N from 0 to F
 *
 **/

use std::cmp::PartialEq;
use std::fmt;
use std::string::ToString;
use strum_macros::{ Display };

// todo documentation
#[derive(Display)]
pub enum Instruction {

    /// Call program at address NNN
    CallProgram { addr: u16 },

    /// Clear screen
    Clear,

    /// Return from a subroutine
    Return,

    /// Jump to address NNN
    Goto { addr: u16 },

    /// Calls subroutine at NNN
    CallSubroutine { addr: u16 },

    /// Skips the next instruction if VX equal NN
    SkipEqualU8 { x: u8, value: u8 },

    /// Skips the next instruction if VX not equal NN
    SkipNotEqualU8 { x: u8, value: u8 },

    /// Skips the next instruction if VX equal VY
    SkipEqualReg { x: u8, y: u8 },

    /// Sets VX to NN
    SetFromU8 { x: u8, value: u8 },

    /// Adds NN to VX
    AddU8 { x: u8, value: u8 },

    /// Sets VX to the value of VY
    SetFromReg { x: u8, y: u8 },

    /// Set VX to VX or VY
    OrReg { x: u8, y: u8 },

    /// Set VX to VX and VY
    AndReg { x: u8, y: u8 },

    /// Set VX to VX xor VY
    XorReg { x: u8, y: u8 },

    /// Adds VY to VX, VF is set to 1 when there's a carry and to 0 when there isn't
    AddReg { x: u8, y: u8 },

    /// VY is substracted from VX, VF is set to 1 when there's a carry and to 0 when there isn't
    SubReg { x: u8, y: u8 },

    /// Stores the least significant bit of VX in VF and then shifts VX to the right by 1
    ShiftRight { x: u8 },

    /// Sets VX to VY minus VX, VF is set to 1 when there's a carry and to 0 when there isn't
    RevSubReg { x: u8, y: u8 },

    /// Stores the most significant bit of VX in VF and then shifts VX to the left by 1
    ShiftLeft { x: u8 },

    /// Skips the next instruction if VX equal VY
    SkipNotEqualReg { x: u8, y: u8 },

    /// Sets I to the address NNN
    StoreAddress { addr: u16 },

    /// Jumps to the address NNN
    JumpToAddress { addr: u16 },

    /// Set VX to bitwise and operation on random number and NN
    Rand { x: u8, value: u8 },

    /// Draw a sprite at (VX, VY) of width 8 and height N
    Draw  { x: u8, y: u8, n: u8 },

    /// Skips next instruction if the key stored in VX is pressed
    SkipIfKeyPressed { x: u8 },

    /// Skips next instruction if the key stored in VX is not pressed
    SkipIfNotKeyPressed { x: u8 },

    /// Sets VX to the value of the delay timer
    SetFromDelayTimer { x: u8 },

    /// Await key press and store to VX
    AwaitKeyPressed { x: u8 },

    /// Sets delay timer to vx
    SetDelayTimer { x: u8 },

    /// Sets sound timer to vx
    SetSoundTimer { x: u8 },

    /// Adds VX to I
    AddToI { x: u8 },

    /// Sets I to the location of the sprite for the character in VX
    SetIToSpriteAddress { x: u8 },

    /// Store decimal representation of VX at I in memory
    StoreAtIAsDecimal { x: u8 },

    /// Dump to memory at I
    DumpToMemory { x: u8 },

    /// Load from memory at I
    LoadFromMemory { x: u8 },

    /// End of program
    EndOfProgram,

    /// Unknown
    UnknownInstruction
}


impl From<(u8, u8)> for Instruction {
    fn from(bytes: (u8, u8)) -> Self {
        let splitted = (
            (bytes.0 & 0xF0) >> 4,
            bytes.0 & 0x0F,
            (bytes.1 & 0xF0) >> 4,
            bytes.1 & 0x000F
        );

        match splitted {
            (0x0, 0x0, 0x0, 0x0) => Instruction::EndOfProgram,
            (0x0, 0x0, 0xE, 0x0) => Instruction::Clear,
            (0x0, 0x0, 0xE, 0xE) => Instruction::Return,
            (0x0, n1, n2, n3) => Instruction::CallProgram { addr: Instruction::address_from(n1, n2, n3) },
            (0x1, n1, n2, n3) => Instruction::Goto { addr: Instruction::address_from(n1, n2, n3) },
            (0x2, n1, n2, n3) => Instruction::CallSubroutine { addr: Instruction::address_from(n1, n2, n3) },
            (0x3, x, n1, n2) => Instruction::SkipEqualU8 { x, value: Instruction::value_from(n1, n2) },
            (0x4, x, n1, n2) => Instruction::SkipNotEqualU8 { x, value: Instruction::value_from(n1, n2) },
            (0x5, x, y, 0x0) => Instruction::SkipEqualReg { x, y },
            (0x6, x, n1, n2) => Instruction::SetFromU8 { x, value: Instruction::value_from(n1, n2) },
            (0x7, x, n1, n2) => Instruction::AddU8 { x, value: Instruction::value_from(n1, n2) },
            (0x8, x, y, 0x0) => Instruction::SetFromReg { x, y },
            (0x8, x, y, 0x1) => Instruction::OrReg { x, y },
            (0x8, x, y, 0x2) => Instruction::AndReg { x, y },
            (0x8, x, y, 0x3) => Instruction::XorReg { x, y },
            (0x8, x, y, 0x4) => Instruction::AddReg { x, y },
            (0x8, x, y, 0x5) => Instruction::SubReg { x, y },
            (0x8, x, _, 0x6) => Instruction::ShiftRight { x },
            (0x8, x, y, 0x7) => Instruction::RevSubReg { x, y },
            (0x8, x, _, 0xE) => Instruction::ShiftLeft { x },
            (0x9, x, y, 0x0) => Instruction::SkipNotEqualReg { x, y },
            (0xA, n1, n2, n3) => Instruction::StoreAddress { addr: Instruction::address_from(n1, n2, n3) },
            (0xB, n1, n2, n3) => Instruction::JumpToAddress { addr: Instruction::address_from(n1, n2, n3) },
            (0xC, x, n1, n2) => Instruction::Rand { x, value: Instruction::value_from(n1, n2) },
            (0xD, x, y, n) => Instruction::Draw { x, y, n },
            (0xE, x, 0x9, 0xE) => Instruction::SkipIfKeyPressed { x },
            (0xE, x, 0xA, 0x1) => Instruction::SkipIfNotKeyPressed { x },
            (0xF, x, 0x0, 0x7) => Instruction::SetFromDelayTimer { x },
            (0xF, x, 0x0, 0xA) => Instruction::AwaitKeyPressed { x },
            (0xF, x, 0x1, 0x5) => Instruction::SetDelayTimer { x },
            (0xF, x, 0x1, 0x8) => Instruction::SetSoundTimer { x },
            (0xF, x, 0x1, 0xE) => Instruction::AddToI { x },
            (0xF, x, 0x2, 0x9) => Instruction::SetIToSpriteAddress { x },
            (0xF, x, 0x3, 0x3) => Instruction::StoreAtIAsDecimal { x },
            (0xF, x, 0x5, 0x5) => Instruction::DumpToMemory { x },
            (0xF, x, 0x6, 0x5) => Instruction::LoadFromMemory { x },
            _ => Instruction::UnknownInstruction
        }
    }
}


impl From<u16> for Instruction {
    fn from(bytes: u16) -> Self {
        Instruction::from((
            ((bytes & 0xFF00) >> 8) as u8,
            (bytes & 0x00FF) as u8
        ))
    }
}


impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}


impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}


impl Instruction {
    fn value_from(n1: u8, n2: u8) -> u8 {
        ((n1 << 4) & 0xF0) | (n2 & 0xF)
    }

    fn address_from(n1: u8, n2: u8, n3: u8) -> u16{
        (((n1 as u16) << 8) & 0xF00) | Instruction::value_from(n2, n3) as u16
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u16_to_instruction() {
        let instruction = Instruction::from(0x00E0);
        assert_eq!(instruction, Instruction::Clear);
    }

    #[test]
    fn test_2xu8_to_instruction() {
        let instruction = Instruction::from((0x0, 0xE0));
        assert_eq!(instruction, Instruction::Clear);
    }
}
