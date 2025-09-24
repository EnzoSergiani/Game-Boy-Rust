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
        self.mmu.set_cartridge(cartridge);
        self.lcd.start(&self.mmu);
    }
}
