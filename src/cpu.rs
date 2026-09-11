use crate::emu::Emulator;

const REG_COUNT: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Register { RAX, RCX, RDX, RBX, RSP, RBP, RSI, RDI, R8, R9, R10, R11, R12, R13, R14, R15 }

pub static REGISTER_NAME: &[&str] = &["RAX", "RCX", "RDX", "RBX", "RSP", "RBP", "RSI", "RDI", " R8", " R9", "R10", "R11", "R12", "R13", "R14", "R15"];

impl Emulator {
    pub fn dump_registers(&self) {
        for i in 0..REG_COUNT {
            println!("{} = 0x{:08x}", REGISTER_NAME[i], self.regs[i]);
        }

        println!("RIP = 0x{:08x}", self.rip);
    }
}