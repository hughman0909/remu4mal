//! lib.rs is responsible for declaring and re-exporting all public modules that make up the emulator library.

pub mod cpu;
pub mod emu;
pub mod memory;

pub mod inst;
pub mod decode_inst;
pub mod exec_inst;

pub mod io;
pub mod trace;
pub mod error;