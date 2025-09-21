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
        let shift: Address = address - ADDRESS_VRAM.start;
        self.vram[shift]
    }

    pub fn write_vram(&mut self, address: Address, value: Byte) {
        let shift: Address = address - ADDRESS_VRAM.start;
        self.vram[shift] = value;
    }

    pub fn read_oam(&self, address: Address) -> Byte {
        let shift: Address = address - ADDRESS_VRAM.start;
        self.oam[shift]
    }

    pub fn write_oam(&mut self, address: Address, value: Byte) {
        let shift: Address = address - ADDRESS_VRAM.start;
        self.oam[shift] = value;
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
        let bytes: [Byte; 16] = {
            let mut arr: [Byte; 16] = [0u8; 16];
            for i in 0..16 {
                arr[i] = self.read_vram(address + i as Address);
            }
            arr
        };
        Tile::from_bytes(bytes)
    }

    pub fn set_test_tile(&mut self) {
        let test_bytes: [Byte; 16] = [
            0x33, 0x55, 0x99, 0xAA, 0x99, 0xAA, 0x33, 0x55, 0x33, 0x55, 0x99, 0xAA, 0x99, 0xAA,
            0x33, 0x55,
        ];

        for i in 0..test_bytes.len() {
            self.write_vram(ADDRESS_VRAM.start + i, test_bytes[i]);
        }
    }

    pub fn debug(&mut self) {
        self.set_test_tile();

        println!("VRAM Dump:");
        for (i, byte) in self.vram.iter().enumerate() {
            if i % 16 == 0 {
                print!("\n{:04X}: ", ADDRESS_VRAM.start + i as Address);
            }
            print!("{:02X} ", byte);
        }
        println!();
        println!();

        println!("Tile set:");
        let vram_tile_count = (SIZE_VRAM / 16) as usize;
        let tile_count = usize::min(
            ((ADDRESS_TILE_SET.end - ADDRESS_TILE_SET.start + 1) / 16) as usize,
            vram_tile_count,
        );
        for chunk_start in (0..tile_count).step_by(11) {
            let chunk_end = usize::min(chunk_start + 11, tile_count);
            for i in chunk_start..chunk_end {
                print!("ID:{:03}             ", i);
            }
            println!();

            for row in 0..8 {
                for i in chunk_start..chunk_end {
                    let tile: Tile = self.get_tile(i);
                    let pixels: [[Byte; 8]; 8] = tile.get_pixels();
                    for pixel in &pixels[row] {
                        let ansi_color = match pixel {
                            0 => "\x1b[48;5;15m  \x1b[0m",
                            1 => "\x1b[48;5;250m  \x1b[0m",
                            2 => "\x1b[48;5;240m  \x1b[0m",
                            3 => "\x1b[48;5;0m  \x1b[0m",
                            _ => "  ",
                        };
                        print!("{}", ansi_color);
                    }
                    print!("   ");
                }
                println!();
            }
            println!();
        }
        println!();

        println!("Tile Map:");
        for address in ADDRESS_TILE_MAP.start..=ADDRESS_TILE_MAP.end {
            let byte = self.read_vram(address);
            print!("{:03X} ", byte);
            if (address - ADDRESS_TILE_MAP.start + 1) % 32 == 0 {
                println!();
            }
        }
        println!();
    }
}
