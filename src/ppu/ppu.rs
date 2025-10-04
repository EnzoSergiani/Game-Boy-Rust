use crate::{
    common::{
        address::{OAM, TILE_MAP, TILE_SET, VRAM},
        constant::DEFAULT_BYTE,
        types::{Address, Byte},
    },
    ppu::tile::Tile,
};

enum AddressingMethod {
    Method8000,
    Method8800,
}

pub struct PPU {
    vram: [Byte; VRAM.size],
    oam: [Byte; OAM.size],
    scx: Byte,
    scy: Byte,
    wx: Byte,
    wy: Byte,
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            vram: [DEFAULT_BYTE; VRAM.size],
            oam: [DEFAULT_BYTE; OAM.size],
            scx: DEFAULT_BYTE,
            scy: DEFAULT_BYTE,
            wx: DEFAULT_BYTE,
            wy: DEFAULT_BYTE,
        }
    }

    pub fn read_vram(&self, address: Address) -> Byte {
        self.vram[address - VRAM.start]
    }

    pub fn write_vram(&mut self, address: Address, value: Byte) {
        self.vram[address - VRAM.start] = value;
    }

    pub fn read_oam(&self, address: Address) -> Byte {
        self.oam[address - VRAM.start]
    }

    pub fn write_oam(&mut self, address: Address, value: Byte) {
        self.oam[address - VRAM.start] = value;
    }

    pub fn reset_vram(&mut self) {
        for address in VRAM.start..=VRAM.end {
            let value: Byte = DEFAULT_BYTE;
            self.write_vram(address, value);
        }
    }

    pub fn reset_oam(&mut self) {
        for address in OAM.start..=OAM.end {
            let value: Byte = DEFAULT_BYTE;
            self.write_oam(address, value);
        }
    }

    pub fn get_tile(&self, id: Address) -> Tile {
        let address: Address = TILE_SET.start + (id as Address) * 16;
        let mut bytes: [Byte; 16] = [DEFAULT_BYTE; 16];

        for i in 0..16 {
            bytes[i] = self.read_vram(address + i);
        }

        Tile::from_bytes(bytes)
    }

    pub fn get_tile_id(&self, x: Address, y: Address) -> Address {
        let tile_address: Address = TILE_MAP.start + (y * 32 + x);
        self.read_vram(tile_address) as Address
    }

    pub fn set_screen_scroll(&mut self, scx: Byte, scy: Byte) {
        self.scx = scx;
        self.scy = scy;
    }

    pub fn get_screen_scroll(&self) -> (f64, f64) {
        (self.scx as f64, self.scy as f64)
    }

    pub fn print_logo(&mut self, logo_nintendo: [[Byte; 12]; 32]) {
        let r_in_circle_bytes: [Byte; 32] = [
            0x3C, 0x3C, 0x42, 0x42, 0xB9, 0xB9, 0xA5, 0xA5, 0xB9, 0xB9, 0xA5, 0xA5, 0x42, 0x42,
            0x3C, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];

        let mut logo_matrix: [[Byte; 13]; 32] = [[DEFAULT_BYTE; 13]; 32];
        for row in 0..32 {
            for col in 0..12 {
                logo_matrix[row][col] = logo_nintendo[row][col];
            }
        }
        for row in 0..32 {
            logo_matrix[row][12] = r_in_circle_bytes[row];
        }

        for col in 0..13 {
            let mut column_bytes: [Byte; 32] = [DEFAULT_BYTE; 32];
            for row in 0..32 {
                column_bytes[row] = logo_matrix[row][col];
            }

            for (i, byte) in column_bytes.iter().enumerate() {
                self.write_vram(TILE_SET.start + col * 32 + i, *byte);
            }
        }

        let pox_x: Address = 4;
        let pox_y: Address = 8;

        for tile_id in 0..13 {
            let tile_map_addr_row1: Address = TILE_MAP.start + (pox_y * 32) + (pox_x + tile_id);
            let tile_map_addr_row2: Address =
                TILE_MAP.start + ((pox_y + 1) * 32) + (pox_x + tile_id);
            self.write_vram(tile_map_addr_row1, (tile_id * 2) as Byte);
            self.write_vram(tile_map_addr_row2, (tile_id * 2 + 1) as Byte);
        }
    }
}
