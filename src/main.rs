mod cartridge;
mod cpu;
mod emulator;
mod mmu;
mod ppu;
use crate::emulator::Emulator;

fn main() {
    println!("Hello, world!");
    let mut emulator: Emulator = Emulator::new();
}
