use crate::vm::{ VM };

pub trait VmInstructions {
    fn clear(&mut self);
    fn return_subroutine(&mut self);
    fn goto(&mut self, addr: u16);
    fn call_subroutine(&mut self, addr: u16);
    fn skip_equal(&mut self, v1: u8, v2: u8);
    fn skip_not_equal(&mut self, v1: u8, v2: u8);
    fn load(&mut self, idx: u8, value: u8);
    fn add(&mut self, idx: u8, value: u8);
    fn sub(&mut self, x: u8, y: u8);
    fn revsub(&mut self, x: u8, y: u8);
    fn or(&mut self, x: u8, y: u8);
    fn and(&mut self, x: u8, y: u8);
    fn xor(&mut self, x: u8, y: u8);
    fn shift_right(&mut self, x: u8);
    fn shift_left(&mut self, x: u8);
}

impl VmInstructions for VM {
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

    fn shift_right(&mut self, x: u8) {
        let ix = x as usize;
        let last = self.regs.len() - 1;

        // Set carry to lsb of Vx
        self.regs[last] = self.regs[ix] & 1;
        self.regs[ix] >>= 1;
    }

    fn shift_left(&mut self, x: u8) {
        let ix = x as usize;
        let last = self.regs.len() - 1;

        // Set carry to lsb of Vx
        self.regs[last] = self.regs[ix] & 1;
        self.regs[ix] <<= 1;
    }
}

