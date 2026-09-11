//! main.rs is responsible for the CLI entry point of the emulator,
//! parsing command-line arguments, loading a binary image into memory, and running the fetch-decode-execute loop.

use std::env;

use remu4emu::emu::Emulator;

const MEMORY_SIZE: usize = 1024 * 1024; 
const RIP: u64 = 0x7c00;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: cargo run -- filename");
        std::process::exit(1);
    }

    let binary = std::fs::read(&args[1]).expect("failed to read file");
    let mut emu = Emulator::new(MEMORY_SIZE, RIP, 0x7c00);
    emu.memory[RIP as usize ..((binary.len() + RIP as usize))].copy_from_slice(&binary);

    while emu.rip < MEMORY_SIZE as u64 {
        let code = emu.get_code8( 0);
        
        println!("RIP = 0x{:08x}, Code = 0x{:02x}", emu.rip, code);

        let decoded = emu.decode_instruction(code);

        emu.exec_instruction(decoded);

        if emu.rip == 0x00000000 {
            println!("\n\nend of program\n\n");
            break;
        }
    }

    emu.dump_registers();
    std::process::exit(0);
}
