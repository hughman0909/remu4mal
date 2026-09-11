use core::panic;

use crate::emu::Emulator;
use crate::inst::{Instruction, Opecode, Operand, ModRM};
// use crate::memory;

impl Emulator {
    pub fn parse_modrm(&mut self) -> ModRM {
        let code: u8 = self.get_code8(0);

        let mut modrm = ModRM::default();

        modrm.m = (code & 0xC0) >> 6;
        modrm.regop = (code & 0x38) >> 3;
        modrm.rm = code & 0x07;

        self.rip += 1;

        if modrm.m != 3 && modrm.regop == 4 {
            modrm.sib = self.get_code8(0);
            self.rip += 1;
        }

        if (modrm.m == 0 && modrm.rm == 5) || modrm.m == 2 {
            modrm.disp = self.get_sign_code32(0) as u32;
            self.rip += 4; 
        } else if modrm.m == 1 {
            modrm.disp = self.get_sign_code8(0) as u32;
            self.rip += 1;
        }

        return modrm;
    }

    pub fn calc_memory_address(&self, modrm: ModRM) -> u32 {
        match modrm.m {
            0 => {
                match modrm.rm {
                    4 => { panic!("not implemented ModRM mod = 0, rm = 4\n"); }
                    5 => { return modrm.disp; }
                    _ => { return self.get_register32(modrm.rm as usize); }
                }
            }
            1 => {
                match modrm.rm {
                    4 => { panic!("not implemented ModRM mod = 1, rm = 4\n"); }
                    _ => { return self.get_register32(modrm.rm as usize); }
                }
            }
            2 => {
                match modrm.rm {
                    4 => { panic!("not implemented ModRM mod = 2, rm = 4\n"); }
                    _ => { return self.get_register32(modrm.rm as usize) + modrm.disp; }
                }
            }
            3 => { panic!("not implemented ModRM mod = 3\n"); }
            _ => { panic!("not implemented ModRM\n"); }
            
        }
    } 

    pub fn modrm_operand(&self, modrm: ModRM) -> Operand {
        if modrm.m == 3 {
            Operand::Reg(modrm.rm as usize)
        } else {
            Operand::Mem(self.calc_memory_address(modrm) as u64)
        }
    }

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