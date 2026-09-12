//! inst.rs is responsible for defining the data types that represent x86 instructions,
//! including opcodes, operands, the `Instruction` struct, and the `ModRM` byte layout.

#[derive(Clone, Copy)]
pub enum Opecode { Mov, Jmp, Add, Sub, Inc }
#[derive(Clone, Copy, Debug)]
pub enum Operand { Reg(usize), Mem(u64), Imm(u64), Rel(i64) }

#[derive(Clone, Copy)]
pub struct Instruction {
    pub op: Opecode,
    pub dst: Option<Operand>,
    pub src: Option<Operand>,
}

#[derive(Clone, Copy, Default)]
pub struct ModRM {
    pub m: u8,
    pub regop: u8,
    pub rm: u8,
    pub sib: u8,
    pub disp: u32,
}