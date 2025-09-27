use crate::{
    cartridge::cartridge::Cartridge,
    cpu::cpu::CPU,
    ppu::{
        ppu::{ADDRESS_TILE_MAP, ADDRESS_TILE_SET, PPU},
        tile::Tile,
    },
};
use core::panic;

pub type Byte = u8;
pub type Address = usize;
pub type Size = usize;
pub const DEFAULT_BYTE: Byte = 0x0000;

pub struct AddressRange {
    pub start: Address,
    pub end: Address,
}

pub const ADDRESS_ROM: AddressRange = AddressRange {
    start: 0x0000,
    end: 0x7FFF,
};
pub const ADDRESS_VRAM: AddressRange = AddressRange {
    start: 0x8000,
    end: 0x9FFF,
};
pub const ADDRESS_RAM: AddressRange = AddressRange {
    start: 0xA000,
    end: 0xBFFF,
};
pub const ADDRESS_WRAM: AddressRange = AddressRange {
    start: 0xC000,
    end: 0xDFFF,
};
pub const ADDRESS_ECHO: AddressRange = AddressRange {
    start: 0xE000,
    end: 0xFDFF,
};
pub const ADDRESS_OAM: AddressRange = AddressRange {
    start: 0xFE00,
    end: 0xFE9F,
};
pub const ADDRESS_INVALID_OAM: AddressRange = AddressRange {
    start: 0xFEA0,
    end: 0xFEFF,
};
pub const ADDRESS_IO: AddressRange = AddressRange {
    start: 0xFF00,
    end: 0xFF7F,
};
pub const ADDRESS_HRAM: AddressRange = AddressRange {
    start: 0xFF80,
    end: 0xFFFE,
};
pub const ADDRESS_IE_REGISTER: AddressRange = AddressRange {
    start: 0xFFFF,
    end: 0xFFFF,
};
pub const ADDRESS_BOOT_ROM: AddressRange = AddressRange {
    start: 0x0000,
    end: 0x00FF,
};
pub const ADDRESS_NINTENDO_LOGO: AddressRange = AddressRange {
    start: 0x104,
    end: 0x133,
};

const SIZE_WRAM: Size = ADDRESS_WRAM.end - ADDRESS_WRAM.start + 1;
const SIZE_HRAM: Size = ADDRESS_HRAM.end - ADDRESS_HRAM.start + 1;
const SIZE_IO: Size = ADDRESS_IO.end - ADDRESS_IO.start + 1;
pub const SIZE_NINTENDO_LOGO: Size = ADDRESS_NINTENDO_LOGO.end - ADDRESS_NINTENDO_LOGO.start + 1;

pub struct MMU {
    cpu: CPU,
    ppu: PPU,
    cartridge: Cartridge,
    wram: [Byte; SIZE_WRAM],
    hram: [Byte; SIZE_HRAM],
    io: [Byte; SIZE_IO],
}

impl MMU {
    pub fn new() -> Self {
        MMU {
            cpu: CPU::new(),
            ppu: PPU::new(),
            cartridge: Cartridge::eject(),
            wram: [DEFAULT_BYTE; SIZE_WRAM],
            hram: [DEFAULT_BYTE; SIZE_HRAM],
            io: [DEFAULT_BYTE; SIZE_IO],
        }
    }

    fn read_wram(&self, address: Address) -> Byte {
        self.wram[address - ADDRESS_WRAM.start]
    }

    fn write_wram(&mut self, address: Address, value: Byte) {
        self.wram[address - ADDRESS_WRAM.start] = value;
    }

    fn read_hram(&self, address: Address) -> Byte {
        self.hram[address - ADDRESS_HRAM.start]
    }

    fn write_hram(&mut self, address: Address, value: Byte) {
        self.hram[address - ADDRESS_HRAM.start] = value;
    }

    fn read_io(&self, address: Address) -> Byte {
        self.io[address - ADDRESS_IO.start]
    }

    fn write_io(&mut self, address: Address, value: Byte) {
        self.io[address - ADDRESS_IO.start] = value;
    }

    pub fn read_memory(&self, address: Address) -> Byte {
        if address >= ADDRESS_ROM.start && address <= ADDRESS_ROM.end {
            self.cartridge.read_rom(address)
        } else if address >= ADDRESS_VRAM.start && address <= ADDRESS_VRAM.end {
            self.ppu.read_vram(address)
        } else if address >= ADDRESS_RAM.start && address <= ADDRESS_RAM.end {
            self.cartridge.read_ram(address)
        } else if address >= ADDRESS_WRAM.start && address <= ADDRESS_WRAM.end {
            self.read_wram(address)
        } else if address >= ADDRESS_ECHO.start && address <= ADDRESS_ECHO.end {
            0xFF
        } else if address >= ADDRESS_OAM.start && address <= ADDRESS_OAM.end {
            self.ppu.read_oam(address)
        } else if address >= ADDRESS_INVALID_OAM.start && address <= ADDRESS_INVALID_OAM.end {
            0xFF
        } else if address >= ADDRESS_IO.start && address <= ADDRESS_IO.end {
            self.read_io(address)
        } else if address >= ADDRESS_HRAM.start && address <= ADDRESS_HRAM.end {
            self.read_hram(address)
        } else if address >= ADDRESS_IE_REGISTER.start && address <= ADDRESS_IE_REGISTER.end {
            0xFF
        } else {
            panic!("Invalid memory read at address: 0x{:04X}", address);
        }
    }

    pub fn write_memory(&mut self, address: Address, value: Byte) {
        if address >= ADDRESS_ROM.start && address <= ADDRESS_ROM.end {
            println!(
                "Tentative d'écriture en ROM à 0x{:04X} avec valeur 0x{:02X}",
                address, value
            );
            // panic!("Cannot write to ROM address: 0x{:04X}", address);
        } else if address >= ADDRESS_VRAM.start && address <= ADDRESS_VRAM.end {
            self.ppu.write_vram(address, value);
        } else if address >= ADDRESS_RAM.start && address <= ADDRESS_RAM.end {
            self.cartridge.write_ram(address, value);
        } else if address >= ADDRESS_WRAM.start && address <= ADDRESS_WRAM.end {
            self.write_wram(address, value);
        } else if address >= ADDRESS_ECHO.start && address <= ADDRESS_ECHO.end {
            // Do nothing
        } else if address >= ADDRESS_OAM.start && address <= ADDRESS_OAM.end {
            self.ppu.write_oam(address, value);
        } else if address >= ADDRESS_INVALID_OAM.start && address <= ADDRESS_INVALID_OAM.end {
            // Do nothing
        } else if address >= ADDRESS_IO.start && address <= ADDRESS_IO.end {
            self.write_io(address, value);
        } else if address >= ADDRESS_HRAM.start && address <= ADDRESS_HRAM.end {
            self.write_hram(address, value);
        } else if address >= ADDRESS_IE_REGISTER.start && address <= ADDRESS_IE_REGISTER.end {
            // Do nothing
        } else {
            panic!("Invalid memory write at address: 0x{:04X}", address);
        }
    }

    pub fn get_nintendo_logo(&mut self) -> [[Byte; 12]; 32] {
        let mut logo_bytes_compressed: [Byte; SIZE_NINTENDO_LOGO] =
            [DEFAULT_BYTE; SIZE_NINTENDO_LOGO];
        for (idx, address) in (ADDRESS_NINTENDO_LOGO.start..=ADDRESS_NINTENDO_LOGO.end).enumerate()
        {
            logo_bytes_compressed[idx] = self.read_memory(address);
        }

        let logo_bytes_1: [Byte; SIZE_NINTENDO_LOGO / 2] =
            logo_bytes_compressed[0..24].try_into().unwrap();
        let logo_bytes_2: [Byte; SIZE_NINTENDO_LOGO / 2] =
            logo_bytes_compressed[24..48].try_into().unwrap();

        let mut logo_matrix_compressed: [[Byte; 6]; 8] = [[DEFAULT_BYTE; 6]; 8];
        for block in 0..2 {
            let logo_bytes_block: &[u8; _] = if block == 0 {
                &logo_bytes_1
            } else {
                &logo_bytes_2
            };
            for col in 0..12 {
                for row in 0..4 {
                    let idx = col * 4 + row;
                    let byte = logo_bytes_block[idx / 2];
                    let nibble = if idx % 2 == 0 { byte >> 4 } else { byte & 0x0F };
                    let matrix_row = row + block * 4;
                    let matrix_col = col / 2;
                    if col % 2 == 0 && matrix_col < 6 {
                        logo_matrix_compressed[matrix_row][matrix_col] = nibble << 4;
                    } else if col % 2 == 1 && matrix_col < 6 {
                        logo_matrix_compressed[matrix_row][matrix_col] |= nibble;
                    }
                }
            }
        }

        let mut logo_matrix_decompressed: [[Byte; 12]; 32] = [[DEFAULT_BYTE; 12]; 32];
        for row in 0..8 {
            for col in 0..6 {
                let byte = logo_matrix_compressed[row][col];
                for bit in 0..8 {
                    let pixel_on = (byte >> (7 - bit)) & 1;
                    let out_row = row * 4;
                    let out_col = col * 2 + (bit / 4);
                    let bit_in_byte = (bit % 4) * 2;

                    if pixel_on == 1 {
                        for dr in 0..4 {
                            logo_matrix_decompressed[out_row + dr][out_col] |=
                                0b11 << (6 - bit_in_byte);
                        }
                    }
                }
            }
        }

        logo_matrix_decompressed
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
                self.write_memory(ADDRESS_TILE_SET.start + col * 32 + i, *byte);
            }
        }

        let pox_x: Address = 4;
        let pox_y: Address = 8;

        for tile_id in 0..13 {
            let tile_map_addr_row1: Address =
                ADDRESS_TILE_MAP.start + (pox_y * 32) + (pox_x + tile_id);
            let tile_map_addr_row2: Address =
                ADDRESS_TILE_MAP.start + ((pox_y + 1) * 32) + (pox_x + tile_id);
            self.write_memory(tile_map_addr_row1, (tile_id * 2) as Byte);
            self.write_memory(tile_map_addr_row2, (tile_id * 2 + 1) as Byte);
        }
    }

    pub fn set_cartridge(&mut self, cartridge: Cartridge) {
        if !cartridge.is_valid() {
            panic!("Invalid cartridge inserted!");
        }
        self.cartridge = cartridge;
        self.boot_ROM();
    }

    pub fn boot_ROM(&mut self) {
        // self.cartridge.print_data();
        self.ppu.reset_vram();

        const SIZE_BOOT_ROM: Size = ADDRESS_BOOT_ROM.end - ADDRESS_BOOT_ROM.start + 1;

        let mut dump: [Byte; SIZE_BOOT_ROM] = [DEFAULT_BYTE; SIZE_BOOT_ROM];
        for idx in ADDRESS_BOOT_ROM.start..=ADDRESS_BOOT_ROM.end {
            dump[idx - ADDRESS_BOOT_ROM.start] = self.read_memory(idx)
        }

        for idx in ADDRESS_TILE_MAP.start..ADDRESS_TILE_MAP.end {
            self.write_memory(idx, 99);
        }

        let logo_nintendo: [[Byte; 12]; 32] = self.get_nintendo_logo();
        self.print_logo(logo_nintendo);

        self.set_wx(0);
        self.set_wy(160);

        // self.ppu.debug();
    }

    pub fn on_frame(&mut self) {
        let new_wx: Byte = self.get_wx() as Byte;
        let mut new_wy: Byte = self.get_wy() as Byte;
        if new_wy > 0 {
            new_wy -= 1;
        }

        self.set_wx(new_wx);
        self.set_wy(new_wy);
    }

    pub fn get_tile(&self, id: Address) -> Tile {
        self.ppu.get_tile(id)
    }

    pub fn print_hex(dump: &[Byte], start_addr: Address, title: &str) {
        print!("{}:", title);
        for (i, byte) in dump.iter().enumerate() {
            if i % 16 == 0 {
                print!("\n{:04X}: ", start_addr + i);
            }
            print!("{:02X} ", byte);
        }
        println!("\n");
    }

    pub fn get_wy(&self) -> Address {
        self.read_io(0xFF4A) as Address
    }

    pub fn get_wx(&self) -> Address {
        self.read_io(0xFF4B) as Address
    }

    pub fn set_wx(&mut self, value: Byte) {
        self.write_io(0xFF4B, value);
    }

    pub fn set_wy(&mut self, value: Byte) {
        self.write_io(0xFF4A, value);
    }
}
