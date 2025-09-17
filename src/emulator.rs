use crate::{
    cartridge::cartridge::Cartridge,
    mmu::mmu::{Address, Byte, MMU},
};

pub struct Emulator {
    mmu: MMU,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator { mmu: MMU::new() }
    }

    pub fn start_cartridge(&mut self, path: &str) {
        self.mmu.set_cartridge(Cartridge::insert(path));
    }
}
