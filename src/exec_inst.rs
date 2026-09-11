use crate::emu::Emulator;
use crate::inst::{Instruction, Opecode};

impl Emulator {
   pub fn exec_instruction(&mut self, inst: Instruction) {
        match inst.op {
            Opecode::Mov => {
                self.write_operand32(inst.dst.unwrap(), self.read_operand32(inst.src.unwrap()) as u64);
            }

            Opecode::Jmp => {
                self.rip = self.rip.wrapping_add_signed(self.read_rel(inst)) as u32 as u64;
            } 

            Opecode::Add => {
                let dst = inst.dst.unwrap();
                let augend = self.read_operand32(dst);
                let addend = self.read_operand32(inst.src.unwrap());
                self.write_operand32(dst, augend.wrapping_add(addend) as u64);
            }

            Opecode::Sub => {
                let dst = inst.dst.unwrap();
                let minuend = self.read_operand32(dst);
                let subtrahend = self.read_operand32(inst.src.unwrap());
                self.write_operand32(dst, minuend.wrapping_sub(subtrahend) as u64);
            }
        }
    }
}