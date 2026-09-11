//! cpu.rs is responsible for defining the CPU register set and providing utilities
//! such as register name mappings and register state dumping for debugging.

use strum::{Display, EnumIter, IntoEnumIterator};

use crate::emu::Emulator;

/// General-purpose registers of the x86-64 integer register file.
///
/// Discriminant values 0–7 match the 3-bit register encoding used in ModRM, SIB,
/// and opcode bytes for the legacy registers (Intel SDM Vol.2 Table 2-1), so a
/// cast `reg as usize` indexes `Emulator::regs` directly without any remapping.
/// R8–R15 are encoded as 0–7 with a REX.B/REX.R bit set; they occupy slots 8–15
/// here and are not reachable from 32-bit-mode instructions without a REX prefix.
#[derive(Debug, Display, EnumIter, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Register { RAX, RCX, RDX, RBX, RSP, RBP, RSI, RDI, R8, R9, R10, R11, R12, R13, R14, R15 }

impl Emulator {
    // dumps registers for debugging
    pub fn dump_registers(&self) {
        for r in Register::iter() {
            println!("{:<3} = 0x{:08x}", r, self.regs[r as usize]);
        }

        println!("RIP = 0x{:08x}", self.rip);
    }
}