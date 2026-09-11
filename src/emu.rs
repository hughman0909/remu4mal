//! emu.rs is responsible for defining the core `Emulator` struct and its constructor,
//! holding the CPU registers, flags, memory, and instruction pointer.

pub struct Emulator {
    pub regs: [u64; 16],
    pub rflags: u64,
    pub memory: Vec<u8>,
    pub rip: u64,
}

impl Emulator {
    pub fn new(size: usize, rip: u64, rsp: u64) -> Self {
       Emulator { 
        regs: [0, 0, 0, 0, rsp, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
        rflags: 0, 
        memory: vec![0; size], 
        rip 
        } 
    }
}