use crate::{
    cartridge::cartridge::Cartridge,
    mmu::mmu::{Address, Byte, MMU},
    mmu::mmu::{Address, MMU},
};

pub struct Emulator {
    mmu: MMU,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator { mmu: MMU::new() }
        Emulator {
            mmu: MMU::new(),
        }
    }

    pub fn start_cartridge(&mut self, path: &str) {
        let cartridge: Cartridge = Cartridge::insert(path);
        let entry_point: Address = cartridge.get_entry_point();
        self.mmu.set_cartridge(cartridge);
    }
}
