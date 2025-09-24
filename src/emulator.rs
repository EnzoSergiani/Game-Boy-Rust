use crate::{
    cartridge::cartridge::Cartridge,
    mmu::mmu::{Address, Byte, MMU},
    mmu::mmu::{Address, MMU},
};
use crate::{cartridge::cartridge::Cartridge, lcd::lcd::LCD, mmu::mmu::MMU};

pub struct Emulator {
    mmu: MMU,
    lcd: LCD,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            mmu: MMU::new(),
            lcd: LCD::new(),
        }
    }

    pub fn start_cartridge(&mut self, path: &str) {
        let cartridge: Cartridge = Cartridge::insert(path);
        let entry_point: Address = cartridge.get_entry_point();
        self.mmu.set_cartridge(cartridge);
    }
}
