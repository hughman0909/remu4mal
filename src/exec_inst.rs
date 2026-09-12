//! exec_inst.rs is responsible for executing decoded instructions on the emulator,
//! dispatching each opcode (MOV, JMP, ADD, SUB, etc.) to the appropriate operation.

use crate::emu::Emulator;
use crate::inst::{Instruction, Opecode, Operand};

impl Emulator {
   pub fn exec_instruction(&mut self, inst: Instruction) {
        match inst.op {
            Opecode::Mov => {
                let dst = inst.dst.unwrap();
                let src = self.read_operand32(inst.src.unwrap());
                self.write_operand32(dst, src);
            },

            Opecode::Jmp => {
                // Cast through u32 to truncate the result to a 32-bit address space.
                // Appropriate for 32-bit protected mode; will need to drop the `as u32`
                // cast when x64 long-mode support is added.
                self.rip = self.rip.wrapping_add_signed(self.read_rel(inst.dst.unwrap())) as u32 as u64;
            },

            Opecode::Add => {
                let dst = inst.dst.unwrap();
                let augend = self.read_operand32(dst);
                let addend = self.read_operand32(inst.src.unwrap());
                self.write_operand32(dst, augend.wrapping_add(addend));
            },

            Opecode::Sub => {
                let dst = inst.dst.unwrap();
                let minuend = self.read_operand32(dst);
                let subtrahend = self.read_operand32(inst.src.unwrap());
                self.write_operand32(dst, minuend.wrapping_sub(subtrahend));
            },

            Opecode::Inc => {
                let dst = inst.dst.unwrap();
                let augend = self.read_operand32(dst);
                self.write_operand32(dst, augend.wrapping_add(1));
            }
        }
    }

    pub fn write_operand32(&mut self, op: Operand, value: u32) {
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

    pub fn read_rel(&self, op: Operand) -> i64 {
        match op {
            Operand::Rel(src) => src,
            _ => panic!("Not expected in rel func"),
        }
    }
}