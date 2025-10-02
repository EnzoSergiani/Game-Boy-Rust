use crate::{
    mmu::mmu::{ADDRESS_OAM, ADDRESS_VRAM, Address, AddressRange, Byte, DEFAULT_BYTE},
    ppu::tile::Tile,
};

const SIZE_VRAM: Address = ADDRESS_VRAM.end - ADDRESS_VRAM.start + 1;
const SIZE_OAM: Address = ADDRESS_OAM.end - ADDRESS_OAM.start + 1;

pub const ADDRESS_TILE_SET: AddressRange = AddressRange {
    start: 0x8000,
    end: 0x97FF,
};

pub const ADDRESS_TILE_MAP: AddressRange = AddressRange {
    start: 0x9800,
    end: 0x9FFF,
};

enum AddressingMethod {
    Method8000,
    Method8800,
}

pub struct PPU {
    vram: [Byte; SIZE_VRAM],
    oam: [Byte; SIZE_OAM],
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            vram: [DEFAULT_BYTE; SIZE_VRAM],
            oam: [DEFAULT_BYTE; SIZE_OAM],
        }
    }

    pub fn read_vram(&self, address: Address) -> Byte {
        self.vram[address - ADDRESS_VRAM.start]
    }

    pub fn write_vram(&mut self, address: Address, value: Byte) {
        self.vram[address - ADDRESS_VRAM.start] = value;
    }

    pub fn read_oam(&self, address: Address) -> Byte {
        self.oam[address - ADDRESS_VRAM.start]
    }

    pub fn write_oam(&mut self, address: Address, value: Byte) {
        self.oam[address - ADDRESS_VRAM.start] = value;
    }

    pub fn reset_vram(&mut self) {
        for address in ADDRESS_VRAM.start..=ADDRESS_VRAM.end {
            let value: Byte = DEFAULT_BYTE;
            self.write_vram(address, value);
        }
    }

    pub fn reset_oam(&mut self) {
        for address in ADDRESS_OAM.start..=ADDRESS_OAM.end {
            let value: Byte = DEFAULT_BYTE;
            self.write_oam(address, value);
        }
    }
    pub fn get_tile(&self, id: Address) -> Tile {
        let address: Address = ADDRESS_TILE_SET.start + (id as Address) * 16;
        let mut bytes: [Byte; 16] = [DEFAULT_BYTE; 16];

        for i in 0..16 {
            bytes[i] = self.read_vram(address + i);
        }

        Tile::from_bytes(bytes)
    }
}
