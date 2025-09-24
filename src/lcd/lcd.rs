use minifb::{
    Key, {Window, WindowOptions},
};

use crate::{
    mmu::mmu::{Address, Byte, DEFAULT_BYTE, MMU},
    ppu::{
        palette::{Colors, Palette},
        ppu::ADDRESS_TILE_MAP,
        tile::Tile,
    },
};

pub struct LCD {
    full_width: u32,
    full_height: u32,
    scale: u32,
    palette: Palette,
}

impl LCD {
    pub fn new() -> Self {
        LCD {
            full_width: 256,
            full_height: 256,
            scale: 3,
            palette: Palette::new(),
        }
    }

    pub fn start(&mut self, mmu: &MMU) {
        let width: usize = (self.full_width * self.scale) as usize;
        let height: usize = (self.full_height * self.scale) as usize;
        let mut buffer: Vec<u32> = vec![0; width * height];

        let mut window: Window = Window::new(
            "Test - ESC to exit",
            width,
            height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        window.set_target_fps(60);

        while window.is_open() && !window.is_key_down(Key::Escape) {
            self.frame(&mut buffer, mmu);
            window.update_with_buffer(&buffer, width, height).unwrap();
        }
    }

    pub fn frame(&self, buffer: &mut [u32], mmu: &MMU) {
        let tile_map: [[Byte; 32]; 32] = self.fetch_tile_map(mmu);

        for map_y in 0..32 {
            for map_x in 0..32 {
                let tile_id: Address = tile_map[map_y][map_x] as Address;
                let tile: Tile = mmu.get_tile(tile_id);
                let pixels: [[Byte; 8]; 8] = tile.get_pixels();

                for y in 0..8 {
                    for x in 0..8 {
                        let color_index: Address = pixels[y][x] as Address;
                        let color: Colors = self.palette.get_color(color_index as u8);

                        let screen_x: Address = (map_x * 8 + x) * self.scale as Address;
                        let screen_y: Address = (map_y * 8 + y) * self.scale as Address;

                        for dy in 0..self.scale as Address {
                            for dx in 0..self.scale as Address {
                                let buf_x: Address = screen_x + dx;
                                let buf_y: Address = screen_y + dy;
                                if buf_x
                                    < buffer.len() / ((self.full_width * self.scale) as Address)
                                    && buf_y < (self.full_height * self.scale) as Address
                                {
                                    buffer[buf_y * (self.full_width * self.scale) as Address
                                        + buf_x] = color.to_rgb();
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn fetch_tile_map(&self, mmu: &MMU) -> [[Byte; 32]; 32] {
        let mut tile_map_2d: [[Byte; 32]; 32] = [[DEFAULT_BYTE; 32]; 32];
        for i in 0..32 {
            for j in 0..32 {
                let address: Address = ADDRESS_TILE_MAP.start + (i * 32 + j) as Address;
                tile_map_2d[i][j] = mmu.read_memory(address);
            }
        }
        tile_map_2d
    }
}
