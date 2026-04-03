pub mod alu;
pub mod bus;
pub mod cu;
pub mod cpu;
pub mod flags;
pub mod io;
pub mod memory;
pub mod motherboard;
pub mod pins;
pub mod registers;
pub mod utils;

mod microcode {
    include!(concat!(env!("OUT_DIR"), "/microcode.rs"));
}

pub use microcode::{MicroOp, MICROCODE_ROM, T_STATE_COUNT, M_CYCLE_COUNT, B, C, D, E, H, L, M, A, PC, BC, DE, HL, SP, WZ};