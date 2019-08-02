use std::path::PathBuf;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{ self, Read, ErrorKind };
use std::fmt;
use log::{ info, error };

const START_ADDR: usize = 0x200;

use crate::instructions::{ Instruction };

#[allow(non_snake_case)]
pub struct VM {

    /// 4KB VM memory
    memory: [u8; 4096],
    pc: usize,

    /// VM Registers V0 to VF,  VF = carry flag
    regs: [u8; 16],

    /// VM Stack (used to store return addresses of subroutines calls)
    stack: [usize; 32],
    stack_ptr: usize,

    /// 60HZ timers (delay timer for events and sound timer for sound effects)
    delay_timer: u8,
    sound_timer: u8,

    /// 16 key keyboard from 0 to F
    input: [bool; 16],

    /// 64x32 pixels display from (OxO, OxO) to (Ox3f, 0x1f)
    display: [[bool; 64]; 32],

    /// 16bits address register (void pointer)
    i: u16,

    /// State of vm (on/off)
    state: bool
}


impl VM {
    pub fn update_key_state(&mut self, key: usize, state: bool) -> Result<(), &'static str> {
        if key > 15 {
            return Err("Key not supported");
        }
        self.input[key] = state;
        Ok(())
    }

    pub fn run(&self) -> bool {
        return self.state;
    }

    pub fn execute_next(&mut self) -> Result<Instruction, io::Error> {
        let bytes = (self.memory[self.pc], self.memory[self.pc + 1]);
        let instruction = Instruction::from(bytes);

        if instruction == Instruction::EndOfProgram {
            self.state = false;
            return Ok(instruction);
        }

        match instruction {
            Instruction::Clear => self.clear(),
            Instruction::Return => self.return_subroutine(),
            Instruction::Goto { addr } => self.goto(addr),
            Instruction::CallSubroutine { addr } => self.call_subroutine(addr),
            Instruction::SkipEqualU8 { x, value } => self.skip_equal(self.regs[x as usize], value),
            Instruction::SkipNotEqualU8 { x, value } => self.skip_not_equal(self.regs[x as usize], value),
            Instruction::SkipEqualReg { x, y } => self.skip_equal(self.regs[x as usize], self.regs[y as usize]),
            Instruction::SetFromU8 { x, value } => self.load(x, value),
            Instruction::AddU8 { x, value } => self.add(x, value),
            Instruction::SetFromReg { x, y } => self.load(x, self.regs[y as usize]),
            Instruction::OrReg { x, y } => self.or(x, y),
            Instruction::AndReg { x, y } => self.and(x, y),
            Instruction::XorReg { x, y } => self.xor(x, y),
            Instruction::AddReg { x, y } => self.add(x, self.regs[y as usize]),
            Instruction::SubReg { x, y } => self.sub(x, y),
            Instruction::RevSubReg { x, y } => self.revsub(x, y),
            _ => {
                error!(
                    "Error: (0x{:X}{:X} -> {}) Bad instruction",
                    bytes.0,
                    bytes.1,
                    instruction
                );
                return Err(io::Error::new(
                    ErrorKind::Interrupted,
                    "Bad instruction"
                ));
            }
        }

        info!("(0x{:X}{:X} -> {}) Instruction executed", bytes.0, bytes.1, instruction);
        self.pc += 2;
        Ok(instruction)
    }


    /// Instructions \\\

    fn clear(&mut self) {
        self.display = [[false; 64]; 32]
    }

    fn return_subroutine(&mut self) {
        self.pc = self.stack[self.stack_ptr];

        if self.stack_ptr == 0 {
            self.state = false;
        } else {
            self.stack_ptr -= 1;
        }
    }

    fn goto(&mut self, addr: u16) {
        self.pc = addr as usize;
    }

    fn call_subroutine(&mut self, addr: u16) {
        self.stack_ptr += 1;
        self.stack[self.stack_ptr] = self.pc;
        self.pc = addr as usize;
    }

    fn skip_equal(&mut self, v1: u8, v2: u8) {
        if v1 == v2 {
            self.pc += 2;
        }
    }

    fn skip_not_equal(&mut self, v1: u8, v2: u8) {
        if v1 != v2 {
            self.pc += 2;
        }
    }

    fn load(&mut self, idx: u8, value: u8) {
        self.regs[idx as usize] = value;
    }

    fn add(&mut self, idx: u8, value: u8) {
        self.regs[idx as usize] += value;
    }

    fn sub(&mut self, x: u8, y: u8) {
        let ix = x as usize;
        let iy = y as usize;
        let last = self.regs.len() - 1;

        self.regs[ix] -= self.regs[iy];
        self.regs[last] = if self.regs[ix] > self.regs[iy] { 1 } else  { 0 }
    }

    fn revsub(&mut self, x: u8, y: u8) {
        let ix = x as usize;
        let iy = y as usize;
        let last = self.regs.len() - 1;

        self.regs[ix] = self.regs[iy] - self.regs[ix];
        self.regs[last] = if self.regs[ix] > self.regs[iy] { 1 } else  { 0 }
    }

    fn or(&mut self, x: u8, y: u8) {
        let idx = x as usize;
        self.regs[idx] = self.regs[idx] | self.regs[y as usize];
    }

    fn and(&mut self, x: u8, y: u8) {
        let idx = x as usize;
        self.regs[idx] = self.regs[idx] & self.regs[y as usize];
    }

    fn xor(&mut self, x: u8, y: u8) {
        let idx = x as usize;
        self.regs[idx] = self.regs[idx] ^ self.regs[y as usize];
    }
}

/// Dump memory
impl fmt::Debug for VM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Memory:")?;
        for byte in self.memory.iter() {
            write!(f, "{:X} ", byte)?;
        }
        write!(f, "\n\nRegisters: ")?;
        for byte in self.regs.iter() {
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
            pc: START_ADDR,
            regs: [0; 16],
            stack: [0; 32],
            stack_ptr: 0,
            delay_timer: 0,
            sound_timer: 0,
            input: [false; 16],
            display: [[false; 64]; 32],
            i: 0,
            state: true
        })
    }
}

