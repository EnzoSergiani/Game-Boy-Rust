mod cartridge;
mod cpu;
mod emulator;
mod lcd;
mod mmu;
mod ppu;
use crate::emulator::Emulator;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <chemin_du_fichier.gb>", args[0]);
        std::process::exit(1);
    }
    let gb_path: &String = &args[1];
    let mut emulator: Emulator = Emulator::new();
}
