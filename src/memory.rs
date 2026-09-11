//! memory.rs is responsible for all memory and register access operations,
//! including reading and writing 8/32-bit values, fetching instruction bytes, and resolving ModRM/operand addresses.

use crate::emu::Emulator;

impl Emulator {
    pub fn get_code8(&self, index: usize) -> u8 {
        return self.memory[self.rip as usize + index];
    }

    pub fn get_sign_code8(&self, index: usize) -> i8 {
        return self.get_code8(index) as i8;
    }

    /**
     * u32::from_le_bytes is alternative way to write
     * 
     * let c_rip = self.rip as usize + index
     * u32::from_le_bytes(memory[c_rip..c_rip + 4].try_into().unwrap())
     */
    pub fn get_code32(&self, index: usize) -> u32 {
        let mut ret: u32 = 0;
        // x86 is little-endian
        for i in 0..4 {
            ret |= (self.get_code8(index + i) as u32) << (i * 8);
        }
        ret
    }

    pub fn get_sign_code32(&self, index: usize) -> i32 {
        return self.get_code32(index) as i32;
    }

    pub fn get_register32(&self, index: usize) -> u32 {
        return self.regs[index] as u32;
    }

    pub fn set_register32(&mut self, index: usize, value: u32) {
        self.regs[index] = value as u64;
    }

    pub fn get_memory8(&self, addr: usize) -> u8 {
        return self.memory[addr];
    }

    pub fn set_memory8(&mut self, addr: usize, value: u8) {
        self.memory[addr] = value as u8;
    }

    pub fn get_memory32(&self, addr: usize) -> u32 {
        let mut ret = 0;
        for i in 0..3 {
            ret |= (self.get_memory8(addr + i) as u32) << (i * 8);
        }
        ret
    }

    pub fn set_memory32(&mut self, addr: usize, value: u32) {
        for i in 0..3 {
            self.set_memory8(addr + i, (value >> (i * 8)) as u8);
        }
    }
} 