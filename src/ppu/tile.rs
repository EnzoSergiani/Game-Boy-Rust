use crate::mmu::mmu::{Byte, DEFAULT_BYTE};

pub struct Tile {
    pixels: [[Byte; 8]; 8],
}

impl Tile {
    pub fn empty() -> Tile {
        Tile {
            pixels: [[DEFAULT_BYTE; 8]; 8],
        }
    }

    pub fn from_bytes(bytes: [Byte; 16]) -> Tile {
        let mut pixels: [[Byte; 8]; 8] = [[DEFAULT_BYTE; 8]; 8];
        for row in 0..8 {
            let low: Byte = bytes[row * 2];
            let high: Byte = bytes[row * 2 + 1];
            for col in 0..8 {
                let bit: usize = 7 - col;
                let l: Byte = (low >> bit) & 1;
                let h: Byte = (high >> bit) & 1;
                pixels[row][col] = (l << 1) | h;
            }
        }
        Tile { pixels }
    }

    pub fn get_pixels(&self) -> [[Byte; 8]; 8] {
        self.pixels
    }
}
