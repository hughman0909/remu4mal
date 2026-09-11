//! decode_inst.rs is responsible for decoding raw x86 machine code bytes into `Instruction` values,
//! parsing opcodes, ModRM bytes, SIB bytes, displacements, and immediates.

use core::panic;

use crate::emu::Emulator;
use crate::inst::{Instruction, Opecode, Operand, ModRM};

impl Emulator {

    /// Parses the ModRM byte at the current `rip` and advances `rip` past it and any
    /// trailing SIB byte or displacement bytes the encoding requires.
    pub fn parse_modrm(&mut self) -> ModRM {
        let code: u8 = self.get_code8(0);
        self.rip += 1;

        let mut modrm = ModRM::default();

        // ModRM byte layout: bits [7:6] = Mod, [5:3] = Reg/Opcode, [2:0] = R/M.
        modrm.m = (code & 0xC0) >> 6;
        modrm.regop = (code & 0x38) >> 3;
        modrm.rm = code & 0x07;

        // rm == 4 (when mod != 3) is not a real register: it signals that a SIB byte follows.
        if modrm.m != 3 && modrm.rm == 4 {
            modrm.sib = self.get_code8(0);
            self.rip += 1;
        }

        // mod == 0, rm == 5: no base register; 32-bit displacement is the entire address.
        // mod == 2: base register + 32-bit displacement.
        // mod == 1: base register + 8-bit displacement (sign-extended to 32 bits).
        if (modrm.m == 0 && modrm.rm == 5) || modrm.m == 2 {
            modrm.disp = self.get_sign_code32(0) as u32;
            self.rip += 4;
        } else if modrm.m == 1 {
            modrm.disp = self.get_sign_code8(0) as u32;
            self.rip += 1;
        }

        modrm
    }

    pub fn calc_memory_address(&self, modrm: ModRM) -> u32 {
        match modrm.m {
            0 => {
                match modrm.rm {
                    4 => { panic!("not implemented ModRM mod = 0, rm = 4\n"); }
                    5 => { modrm.disp }
                    _ => { self.get_register32(modrm.rm as usize) }
                }
            }
            1 => {
                match modrm.rm {
                    4 => { panic!("not implemented ModRM mod = 1, rm = 4\n"); }
                    _ => { self.get_register32(modrm.rm as usize) }
                }
            }
            2 => {
                match modrm.rm {
                    4 => { panic!("not implemented ModRM mod = 2, rm = 4\n"); }
                    _ => { self.get_register32(modrm.rm as usize) + modrm.disp }
                }
            }
            3 => { panic!("not implemented ModRM mod = 3\n"); }
            _ => { panic!("not implemented ModRM\n"); }
            
        }
    } 

    /// Resolves a ModRM r/m field to either a register or memory operand.
    /// mod == 3 → register; anything else → effective memory address.
    pub fn modrm_operand(&self, modrm: ModRM) -> Operand {
        if modrm.m == 3 {
            Operand::Reg(modrm.rm as usize)
        } else {
            Operand::Mem(self.calc_memory_address(modrm) as u64)
        }
    }

    // Decodes instruction, and return Instruction depending on the opecode.
    pub fn decode_instruction(&mut self, code: u8) -> Instruction {
        match code {
            // add_rm32_r32
            0x01 => {
                self.rip += 1;
                let modrm = self.parse_modrm();
                let reg = modrm.regop as usize;
                Instruction {
                    op: Opecode::Add,
                    dst: Some(self.modrm_operand(modrm)),
                    src: Some(Operand::Reg(reg)),
                }
            },
            0x83 => {
                self.rip += 1;
                let modrm = self.parse_modrm();
                let imm = self.get_code8(0);
                self.rip += 1;
                match modrm.regop {
                    // regop == 5 selects SUB in the 0x83 opcode group.
                    // sub_rm32_imm8
                    5 => Instruction {
                        op: Opecode::Sub,
                        dst: Some(self.modrm_operand(modrm)),
                        src: Some(Operand::Imm(imm as u64)),
                    },
                    _ => {
                        panic!("Not implemented: 0x83");
                    }
                }
            },
            // mov_rm32_r32
            0x89 => {
                self.rip += 1;
                let modrm = self.parse_modrm();
                let reg = modrm.regop as usize;
                Instruction {
                    op: Opecode::Mov,
                    dst: Some(self.modrm_operand(modrm)),
                    src: Some(Operand::Reg(reg)),
                }
            },

            // mov_r32_imm32
            // Opcodes 0xB8–0xBF embed the destination register in the low 3 bits of the opcode
            // byte itself.
            0xB8..=0xBF => {
                let reg = (self.get_code8(0) & 0x07) as usize;
                self.rip += 1;
                let imm = self.get_code32(0) as u64;
                self.rip += 4;
                Instruction { 
                    op: Opecode::Mov, 
                    dst: Some(Operand::Reg(reg)), 
                    src: Some(Operand::Imm(imm)), 
                }
            },
            
            // mov_rm32_imm32
            0xC7 => {
                self.rip += 1;
                let modrm = self.parse_modrm();
                let imm = self.get_code32(0) as u64;
                self.rip += 4;
                Instruction {
                    op: Opecode::Mov,
                    dst: Some(self.modrm_operand(modrm)),
                    src: Some(Operand::Imm(imm)),
                }
            },

            // near_jump
            0xE9 => {
                self.rip += 1;
                // rel32 is a signed 32-bit offset; cast to i32 first to sign-extend into i64.
                let rel = self.get_code32(0) as i32 as i64;
                self.rip += 4;
                Instruction {
                    op: Opecode::Jmp, 
                    dst: Some(Operand::Rel(rel)),
                    src: None, 
                }
            },

            // short_jump
            0xEB => {
                self.rip += 1;
                // rel8 is a signed 8-bit offset; cast to i8 first to sign-extend into i64.
                let rel = self.get_code8(0) as i8 as i64;
                self.rip += 1;
                Instruction {
                    op: Opecode::Jmp, 
                    dst: Some(Operand::Rel(rel)),
                    src: None, 
                }
            },
            // 0xFF => {}
            _ => panic!("not implemented: {:02x}", code),
        }
    }
}