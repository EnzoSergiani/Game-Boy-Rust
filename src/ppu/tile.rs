use crate::mmu::{
    address::{ADDRESS, Address},
    mmu::{Byte, DEFAULT_BYTE, MMU},
};

pub struct Tile {
    pixels: [[Byte; 8]; 8],
}

impl Tile {
    pub fn from_bytes(bytes: [Byte; 16]) -> Tile {
        let mut pixels: [[Byte; 8]; 8] = [[DEFAULT_BYTE; 8]; 8];
        for row in 0..8 {
            let low: Byte = bytes[row * 2];
            let high: Byte = bytes[row * 2 + 1];
            for col in 0..8 {
                let bit: usize = 7 - col;
                let l: Byte = (low >> bit) & 1;
                let h: Byte = (high >> bit) & 1;
                pixels[row][col] = (h << 1) | l;
            }
        }
        Tile { pixels }
    }

    pub fn from_address(mmu: &mut MMU, tile_id: Address) -> Tile {
        let address_start: Address = ADDRESS::TILE_SET.start + tile_id * 16;
        let mut pixels: [[Byte; 8]; 8] = [[DEFAULT_BYTE; 8]; 8];
        for row in 0..8 {
            let low: Byte = mmu.read_memory(address_start + row * 2);
            let high: Byte = mmu.read_memory(address_start + row * 2 + 1);
            for col in 0..8 {
                let bit: usize = 7 - col;
                let l: Byte = (low >> bit) & 1;
                let h: Byte = (high >> bit) & 1;
                pixels[row][col] = (h << 1) | l;
            }
        }
        Tile { pixels }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Byte {
        self.pixels[y][x]
    }

    pub fn get_pixels(&self) -> [[Byte; 8]; 8] {
        self.pixels
    }
}
