pub struct Emulator {
    pub regs: [u64; 16],
    pub rflags: u64,
    pub memory: Vec<u8>,
    pub rip: u64,
}

// This fn creates emulator. In the original emulator, it was in C so, 
// destroy_emu was needed but deleted in Rust ver 
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