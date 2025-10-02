mod cartridge;
mod cpu;
mod emulator;
mod lcd;
mod mmu;
mod ppu;
use crate::emulator::Emulator;

fn main() {
    let emulator: Emulator = Emulator::new();
    emulator.start();
}
