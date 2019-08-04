use rand::{ self, Rng };

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
    fn store_address(&mut self, addr: u16);
    fn jump(&mut self, addr: u16);
    fn rand(&mut self, x: u8, value: u8);
    fn draw(&mut self, x: u8, y: u8, nibble: u8);
    fn skip_key_pressed(&mut self, x: u8);
    fn skip_not_key_pressed(&mut self, x: u8);
    fn set_delay_timer(&mut self, x: u8);
    fn wait_key_pressed(&mut self, x: u8);
    fn set_sound_timer(&mut self, x: u8);
    fn store_delay_timer(&mut self, x: u8);
    fn increment_addr_reg(&mut self, x: u8);
    fn store_sprite_addr(&mut self, x: u8);
    fn bcd(&mut self, x: u8);
    fn register_dump(&mut self, x: u8);
    fn register_load(&mut self, x: u8);
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

    fn store_address(&mut self, addr: u16) {
        self.i = addr;
    }

    fn jump(&mut self, addr: u16) {
        self.pc = self.regs[0] as usize + addr as usize;
    }

    fn rand(&mut self, x: u8, value: u8) {
        let mut rng = rand::thread_rng();
        self.regs[x as usize] = rng.gen_range(0, 255) & value;
    }

    fn draw(&mut self, x: u8, y: u8, nibble: u8) {
        // todo
    }

    fn skip_key_pressed(&mut self, x: u8) {
        let idx = self.regs[x as usize] as usize;

        if self.input[idx] {
            self.pc += 2;
        }
    }

    fn skip_not_key_pressed(&mut self, x: u8) {
        let idx = self.regs[x as usize] as usize;

        if !self.input[idx] {
            self.pc += 2;
        }
    }

    fn set_delay_timer(&mut self, x: u8) {
        self.delay_timer = self.regs[x as usize];
    }

    fn wait_key_pressed(&mut self, x: u8) {
        // todo
    }

    fn set_sound_timer(&mut self, x: u8) {
        self.sound_timer = self.regs[x as usize];
    }

    fn store_delay_timer(&mut self, x: u8) {
        self.regs[x as usize] = self.delay_timer;
    }

    fn increment_addr_reg(&mut self, x: u8) {
        self.i += self.regs[x as usize] as u16;
    }

    fn store_sprite_addr(&mut self, x: u8) {
        // todo
    }

    fn bcd(&mut self, x: u8) {
        let vx = self.regs[x as usize];
        let x_bcd = ((vx / 100) % 10, (vx / 10) % 10, vx % 10);
        let idx = self.i as usize;

        self.memory[idx] = x_bcd.0;
        self.memory[idx + 1] = x_bcd.1;
        self.memory[idx + 2] = x_bcd.2;
    }

    fn register_dump(&mut self, x: u8) {
        let mut idx = self.i as usize;
        for j in 0..(x as usize) {
            self.memory[idx] = self.regs[j];
            idx += 1;
        }
    }

    fn register_load(&mut self, x: u8) {
        let mut idx = self.i as usize;
        for j in 0..(x as usize) {
            self.regs[j] = self.memory[idx];
            idx += 1;
        }
    }
}

