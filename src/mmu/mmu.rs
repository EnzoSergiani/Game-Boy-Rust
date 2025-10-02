use crate::{
    cartridge::cartridge::Cartridge,
    cpu::cpu::CPU,
    mmu::address::{ADDRESS, Address, Size},
    mmu::boot_rom::BootROM,
    ppu::{ppu::PPU, tile::Tile},
};
use std::sync::{Mutex, MutexGuard, OnceLock};

pub type Byte = u8;
pub const DEFAULT_BYTE: Byte = 0x0000;

pub static FRAME_COUNTER: OnceLock<Mutex<Byte>> = OnceLock::new();

pub struct MMU {
    cpu: CPU,
    ppu: PPU,
    cartridge: Cartridge,
    wram: [Byte; ADDRESS::WRAM.size],
    hram: [Byte; ADDRESS::HRAM.size],
    io: [Byte; ADDRESS::IO.size],
}

impl MMU {
    pub fn new() -> Self {
        MMU {
            cpu: CPU::new(),
            ppu: PPU::new(),
            cartridge: Cartridge::eject(),
            wram: [DEFAULT_BYTE; ADDRESS::WRAM.size],
            hram: [DEFAULT_BYTE; ADDRESS::HRAM.size],
            io: [DEFAULT_BYTE; ADDRESS::IO.size],
        }
    }

    pub fn get_cpu(&mut self) -> &mut CPU {
        &mut self.cpu
    }

    pub fn get_ppu(&mut self) -> &mut PPU {
        &mut self.ppu
    }

    pub fn get_cartridge(&mut self) -> &Cartridge {
        &self.cartridge
    }

    fn read_wram(&self, address: Address) -> Byte {
        self.wram[address - ADDRESS::WRAM.start]
    }

    fn write_wram(&mut self, address: Address, value: Byte) {
        self.wram[address - ADDRESS::WRAM.start] = value;
    }

    fn read_hram(&self, address: Address) -> Byte {
        self.hram[address - ADDRESS::HRAM.start]
    }

    fn write_hram(&mut self, address: Address, value: Byte) {
        self.hram[address - ADDRESS::HRAM.start] = value;
    }

    fn read_io(&self, address: Address) -> Byte {
        self.io[address - ADDRESS::IO.start]
    }

    fn write_io(&mut self, address: Address, value: Byte) {
        self.io[address - ADDRESS::IO.start] = value;
    }

    pub fn read_memory(&self, address: Address) -> Byte {
        if address >= ADDRESS::ROM.start && address <= ADDRESS::ROM.end {
            self.cartridge.read_rom(address)
        } else if address >= ADDRESS::VRAM.start && address <= ADDRESS::VRAM.end {
            self.ppu.read_vram(address)
        } else if address >= ADDRESS::RAM.start && address <= ADDRESS::RAM.end {
            self.cartridge.read_ram(address)
        } else if address >= ADDRESS::WRAM.start && address <= ADDRESS::WRAM.end {
            self.read_wram(address)
        } else if address >= ADDRESS::ECHO.start && address <= ADDRESS::ECHO.end {
            0xFF
        } else if address >= ADDRESS::OAM.start && address <= ADDRESS::OAM.end {
            self.ppu.read_oam(address)
        } else if address >= ADDRESS::INVALID_OAM.start && address <= ADDRESS::INVALID_OAM.end {
            0xFF
        } else if address >= ADDRESS::IO.start && address <= ADDRESS::IO.end {
            self.read_io(address)
        } else if address >= ADDRESS::HRAM.start && address <= ADDRESS::HRAM.end {
            self.read_hram(address)
        } else if address >= ADDRESS::IE_REGISTER.start && address <= ADDRESS::IE_REGISTER.end {
            0xFF
        } else {
            panic!("Invalid memory read at address: 0x{:04X}", address);
        }
    }

    pub fn write_memory(&mut self, address: Address, value: Byte) {
        if address >= ADDRESS::ROM.start && address <= ADDRESS::ROM.end {
            println!(
                "Tentative d'écriture en ROM à 0x{:04X} avec valeur 0x{:02X}",
                address, value
            );
            // panic!("Cannot write to ROM address: 0x{:04X}", address);
        } else if address >= ADDRESS::VRAM.start && address <= ADDRESS::VRAM.end {
            self.ppu.write_vram(address, value);
        } else if address >= ADDRESS::RAM.start && address <= ADDRESS::RAM.end {
            self.cartridge.write_ram(address, value);
        } else if address >= ADDRESS::WRAM.start && address <= ADDRESS::WRAM.end {
            self.write_wram(address, value);
        } else if address >= ADDRESS::ECHO.start && address <= ADDRESS::ECHO.end {
            // Do nothing
        } else if address >= ADDRESS::OAM.start && address <= ADDRESS::OAM.end {
            self.ppu.write_oam(address, value);
        } else if address >= ADDRESS::INVALID_OAM.start && address <= ADDRESS::INVALID_OAM.end {
            // Do nothing
        } else if address >= ADDRESS::IO.start && address <= ADDRESS::IO.end {
            self.write_io(address, value);
        } else if address >= ADDRESS::HRAM.start && address <= ADDRESS::HRAM.end {
            self.write_hram(address, value);
        } else if address >= ADDRESS::IE_REGISTER.start && address <= ADDRESS::IE_REGISTER.end {
            // Do nothing
        } else {
            panic!("Invalid memory write at address: 0x{:04X}", address);
        }
    }

    pub fn get_nintendo_logo(&mut self) -> [[Byte; 12]; 32] {
        let mut logo_bytes_compressed: [Byte; ADDRESS::NINTENDO_LOGO.size] =
            [DEFAULT_BYTE; ADDRESS::NINTENDO_LOGO.size];
        for (idx, address) in
            (ADDRESS::NINTENDO_LOGO.start..=ADDRESS::NINTENDO_LOGO.end).enumerate()
        {
            logo_bytes_compressed[idx] = self.read_memory(address);
        }

        let logo_bytes_1: [Byte; ADDRESS::NINTENDO_LOGO.size / 2] =
            logo_bytes_compressed[0..24].try_into().unwrap();
        let logo_bytes_2: [Byte; ADDRESS::NINTENDO_LOGO.size / 2] =
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
                self.write_memory(ADDRESS::TILE_SET.start + col * 32 + i, *byte);
            }
        }

        let pox_x: Address = 4;
        let pox_y: Address = 8;

        for tile_id in 0..13 {
            let tile_map_addr_row1: Address =
                ADDRESS::TILE_MAP.start + (pox_y * 32) + (pox_x + tile_id);
            let tile_map_addr_row2: Address =
                ADDRESS::TILE_MAP.start + ((pox_y + 1) * 32) + (pox_x + tile_id);
            self.write_memory(tile_map_addr_row1, (tile_id * 2) as Byte);
            self.write_memory(tile_map_addr_row2, (tile_id * 2 + 1) as Byte);
        }
    }

    pub fn set_cartridge(&mut self, cartridge: Cartridge) {
        if !cartridge.is_valid() {
            panic!("Invalid cartridge inserted!");
        }
        self.cartridge = cartridge;
    }

    pub fn on_frame(&mut self) {
        self.boot_update_animation();
    }

    pub fn delay_frames(frames: u8) -> bool {
        let counter: &Mutex<u8> = FRAME_COUNTER.get_or_init(|| Mutex::new(frames));
        let mut count: MutexGuard<'_, u8> = counter.lock().unwrap();

        if *count > 0 {
            *count -= 1;
            false
        } else {
            true
        }
    }

    pub fn reset_delay() {
        if let Some(counter) = FRAME_COUNTER.get() {
            let mut count: MutexGuard<'_, u8> = counter.lock().unwrap();
            *count = 0;
        }
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

    pub fn set_screen_position(&mut self, x: Byte, y: Byte) {
        self.write_io(0xFF4B, x);
        self.write_io(0xFF4A, y);
    }

    pub fn get_screen_position(&self) -> (Address, Address) {
        (
            self.read_io(0xFF4B) as Address,
            self.read_io(0xFF4A) as Address,
        )
    }
}
