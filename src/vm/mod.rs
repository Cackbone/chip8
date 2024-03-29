use std::path::PathBuf;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{ self, Read, ErrorKind };
use std::fmt;

use log::{ error };

use crate::instructions::{ Instruction };

mod vm_instructions;
use vm_instructions::*;

const START_ADDR: usize = 0x200;

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
    pub fn get_program(&self) -> Vec<String> {
        let len = self.memory.len() - 1;
        let mut program = vec![];
        for x in (START_ADDR..len).step_by(2) {
            let instruction = Instruction::from((self.memory[x], self.memory[x + 1]));

            if instruction == Instruction::EndOfProgram {
                break;
            }
            program.push(instruction.to_asm());
        }
        program
    }

    pub fn run(&self) -> bool {
        return self.state;
    }

    pub fn execute_next(&mut self) -> Result<Instruction, io::Error> {
        let bytes = (self.memory[self.pc], self.memory[self.pc + 1]);
        let instruction = Instruction::from(bytes);

        if instruction == Instruction::EndOfProgram {
            self.state = false;
            println!("end");
            return Ok(instruction);
        }
        self.execute(&instruction, bytes)?;
        self.pc += 2;
        Ok(instruction)
    }

    fn execute(&mut self, instruction: &Instruction, bytes: (u8, u8)) -> Result<(), io::Error> {
        match *instruction {
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
            Instruction::ShiftRight { x } => self.shift_right(x),
            Instruction::ShiftLeft { x } => self.shift_left(x),
            Instruction::SkipNotEqualReg { x, y } => self.skip_not_equal(self.regs[x as usize], self.regs[y as usize]),
            Instruction::StoreAddress { addr } => self.store_address(addr),
            Instruction::JumpToAddress { addr } => self.jump(addr),
            Instruction::Rand { x, value } => self.rand(x, value),
            Instruction::Draw { x, y, n } => self.draw(x, y, n),
            Instruction::SkipIfKeyPressed { x } => self.skip_key_pressed(x),
            Instruction::SkipIfNotKeyPressed { x } => self.skip_not_key_pressed(x),
            Instruction::SetDelayTimer { x } => self.set_delay_timer(x),
            Instruction::SetSoundTimer { x } => self.set_sound_timer(x),
            Instruction::SetFromDelayTimer { x } => self.store_delay_timer(x),
            Instruction::AwaitKeyPressed { x } => self.wait_key_pressed(x),
            Instruction::AddToI { x } => self.increment_addr_reg(x),
            Instruction::SetIToSpriteAddress { x } => self.store_sprite_addr(x),
            Instruction::StoreAtIAsDecimal { x } => self.bcd(x),
            Instruction::DumpToMemory { x } => self.register_dump(x),
            Instruction::LoadFromMemory { x } => self.register_load(x),
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
        Ok(())
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

