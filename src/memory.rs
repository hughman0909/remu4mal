use core::panic;

use crate::emu::Emulator;
use crate::inst::{Instruction, ModRM, Operand};

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
        // Get 32 bits from memory in little endian 
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

    pub fn set_rm32(&mut self, modrm: ModRM, value: u32) {        
        if modrm.m == 3 {
            self.set_register32(modrm.rm as usize, value);
        } else {
            let addr = self.calc_memory_address(modrm);
            self.set_memory32(addr as usize, value);
        }
    }

    pub fn get_rm32(&self, modrm: ModRM) -> u32 {        
        if modrm.m == 3 {
            return self.get_register32(modrm.rm as usize);
        } else {
            let addr = self.calc_memory_address(modrm);
            return self.get_memory32(addr as usize);
        }
    }

    pub fn set_r32(&mut self, modrm: ModRM, value: u32) {
        self.set_register32(modrm.regop as usize, value)
    }

    pub fn get_r32(&self, modrm: ModRM) -> u32 {
        self.get_register32(modrm.regop as usize)
    }

    pub fn write_operand32(&mut self, op: Operand, value: u64) {
        match op {
                    Operand::Reg(addr) => self.set_register32(addr, value as u32),
                    Operand::Mem(addr) => self.set_memory32(addr as usize, value as u32),
                    _ => panic!("cannnot write to {:?}", op),
                }
    }

    pub fn read_operand32(&self, op: Operand) -> u32{
        match op {
            Operand::Reg(addr) => self.get_register32(addr),
            Operand::Mem(addr) => self.get_memory32(addr as usize),
            Operand::Imm(value) => value as u32,
            Operand::Rel(_) => panic!("cannot read rel with this"),
        }
    } 

    pub fn read_rel(&self, inst: Instruction) -> i64 {
        match inst.dst.unwrap() {
            Operand::Rel(src) => src,
            _ => panic!("Not expected in rel func"),
        }
    }
}