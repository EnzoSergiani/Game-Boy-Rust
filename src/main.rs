mod cartridge;
pub mod common;
mod cpu;
mod emulator;
mod mmu;
mod ppu;
use crate::emulator::Emulator;

fn main() {
    let emulator: Emulator = Emulator::new();
    emulator.start();
}
