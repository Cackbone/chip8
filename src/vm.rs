use std::path::PathBuf;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{ self, Read };
use std::fmt;

const START_ADDR: usize = 0x200;

#[allow(non_snake_case)]
pub struct VM {

    /// 4KB VM memory
    memory: [u8; 4096],
    pc: u8,

    /// VM Registers V0 to VF,  VF = carry flag
    regs: [u8; 16],

    /// VM Stack (used to store return addresses of subroutines calls)
    stack: [u8; 32],
    stack_ptr: u8,

    /// 60HZ timers (delay timer for events and sound timer for sound effects)
    delay_timer: u8,
    sound_timer: u8,

    /// 16 key keyboard from 0 to F
    input: [bool; 16],

    /// 64x32 pixels display from (OxO, OxO) to (Ox3f, 0x1f)
    display: [[bool; 64]; 32],

    // 16bits register (void pointer)
    i: u16
}


impl VM {
}

/// Dump memory
impl fmt::Debug for VM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.memory.iter() {
            write!(f, "{:X} ", byte)?;
        }
        Ok(())
    }
}

/// ROM loader
impl TryFrom<PathBuf> for VM {
    type Error = io::Error;

    fn try_from(file_path: PathBuf) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let mut vm_mem = [0; 4096];

        for (i, byte) in file.bytes().enumerate() {
            vm_mem[START_ADDR + i] = byte?;
        }

        Ok(VM {
            memory: vm_mem,
            pc: START_ADDR as u8,
            regs: [0; 16],
            stack: [0; 32],
            stack_ptr: 0,
            delay_timer: 0,
            sound_timer: 0,
            input: [false; 16],
            display: [[false; 64]; 32],
            i: 0
        })
    }
}
