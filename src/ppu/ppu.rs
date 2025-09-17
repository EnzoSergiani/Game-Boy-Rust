use crate::mmu::mmu::{Address, Byte};

pub struct PPU {
}

impl PPU {
    pub fn new() -> Self {
        PPU {}
    }

    pub fn read_vram(&self, address: Address) -> Byte {
        0
    }

    pub fn write_vram(&self, address: Address, value: Byte) {}

    pub fn read_oam(&self, address: Address) -> Byte {
        0
    }

    pub fn write_oam(&self, address: Address, value: Byte) {}
}
