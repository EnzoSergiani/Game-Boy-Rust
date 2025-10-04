use crate::{
    common::{
        address::TILE_SET,
        constant::DEFAULT_BYTE,
        types::{Address, Byte},
    },
    ppu::ppu::PPU,
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

    pub fn from_address(ppu: &PPU, tile_id: Address) -> Tile {
        let address_start: Address = TILE_SET.start + tile_id * 16;
        let mut bytes: [Byte; 16] = [DEFAULT_BYTE; 16];
        for i in 0..16 {
            bytes[i] = ppu.read_vram(address_start + i);
        }
        Tile::from_bytes(bytes)
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Byte {
        self.pixels[y][x]
    }

    pub fn get_pixels(&self) -> [[Byte; 8]; 8] {
        self.pixels
    }
}
