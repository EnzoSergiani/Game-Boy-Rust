use crate::{
    mmu::{
        address::{ADDRESS, Address},
        mmu::{Byte, DEFAULT_BYTE},
    },
    ppu::tile::Tile,
};

enum AddressingMethod {
    Method8000,
    Method8800,
}

pub struct PPU {
    vram: [Byte; ADDRESS::VRAM.size],
    oam: [Byte; ADDRESS::OAM.size],
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            vram: [DEFAULT_BYTE; ADDRESS::VRAM.size],
            oam: [DEFAULT_BYTE; ADDRESS::OAM.size],
        }
    }

    pub fn read_vram(&self, address: Address) -> Byte {
        self.vram[address - ADDRESS::VRAM.start]
    }

    pub fn write_vram(&mut self, address: Address, value: Byte) {
        self.vram[address - ADDRESS::VRAM.start] = value;
    }

    pub fn read_oam(&self, address: Address) -> Byte {
        self.oam[address - ADDRESS::VRAM.start]
    }

    pub fn write_oam(&mut self, address: Address, value: Byte) {
        self.oam[address - ADDRESS::VRAM.start] = value;
    }

    pub fn reset_vram(&mut self) {
        for address in ADDRESS::VRAM.start..=ADDRESS::VRAM.end {
            let value: Byte = DEFAULT_BYTE;
            self.write_vram(address, value);
        }
    }

    pub fn reset_oam(&mut self) {
        for address in ADDRESS::OAM.start..=ADDRESS::OAM.end {
            let value: Byte = DEFAULT_BYTE;
            self.write_oam(address, value);
        }
    }
    pub fn get_tile(&self, id: Address) -> Tile {
        let address: Address = ADDRESS::TILE_SET.start + (id as Address) * 16;
        let mut bytes: [Byte; 16] = [DEFAULT_BYTE; 16];

        for i in 0..16 {
            bytes[i] = self.read_vram(address + i);
        }

        Tile::from_bytes(bytes)
    }
}
