use minifb::{
    Key, {Window, WindowOptions},
};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::{
    mmu::mmu::{Address, Byte, DEFAULT_BYTE, MMU},
    ppu::{
        palette::{Colors, Palette},
        ppu::ADDRESS_TILE_MAP,
        tile::Tile,
    },
};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct LCD {
    full_width: u32,
    full_height: u32,
    screen_height: u32,
    screen_width: u32,
    scale: u32,
    palette: Palette,
    frame_count: Address,
}

impl LCD {
    pub fn new() -> Self {
        LCD {
            full_width: 256,
            full_height: 256,
            screen_width: 160,
            screen_height: 144,
            scale: 3,
            palette: Palette::new(),
            frame_count: 0,
        }
    }

    pub fn init_screen(
        &mut self,
        mmu_arc: &Arc<Mutex<MMU>>,
        is_debug: bool,
        start_signal: Arc<AtomicBool>,
    ) {
        let (width, height) = if is_debug {
            (
                (self.full_width * self.scale) as Address,
                (self.full_height * self.scale) as Address,
            )
        } else {
            (
                self.screen_width as Address * self.scale as Address,
                self.screen_height as Address * self.scale as Address,
            )
        };

        let mut buffer: Vec<u32> = vec![0; (width * height) as usize];

        let window_title: &'static str = if is_debug { "Debug" } else { "Screen" };
        let mut window: Window = Window::new(
            window_title,
            width,
            height,
            WindowOptions {
                borderless: false,
                resize: true,
                title: true,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        while window.is_open() && !start_signal.load(Ordering::SeqCst) {
            window.update();
        }

        self.start_screen(
            mmu_arc,
            is_debug,
            window,
            &mut buffer,
            width as u32,
            height as u32,
        );
    }

    fn start_screen(
        &mut self,
        mmu_arc: &Arc<Mutex<MMU>>,
        is_debug: bool,
        mut window: Window,
        mut buffer: &mut [u32],
        width: u32,
        height: u32,
    ) {
        window.set_target_fps(60);
        while window.is_open() && !window.is_key_down(Key::Escape) {
            let mut mmu = mmu_arc.lock().unwrap();
            self.frame(&mut buffer, &mut mmu, is_debug);
            drop(mmu);
            window
                .update_with_buffer(&buffer, width as usize, height as usize)
                .unwrap();
        }
    }
    pub fn frame(&mut self, buffer: &mut [u32], mmu: &mut MMU, is_debug: bool) {
        self.frame_count += 1;
        mmu.on_frame();

        let (width, height, wx, wy) = if is_debug {
            (
                self.full_width as Address * self.scale as Address,
                self.full_height as Address * self.scale as Address,
                mmu.get_wx() as Address,
                mmu.get_wy() as Address,
            )
        } else {
            (
                self.screen_width as Address * self.scale as Address,
                self.screen_height as Address * self.scale as Address,
                mmu.get_wx() as Address,
                mmu.get_wy() as Address,
            )
        };

        let (tile_y_range, tile_x_range, tile_wx, tile_wy) = if is_debug {
            (0..32, 0..32, 0, 0)
        } else {
            (
                (wy / (8 * self.scale as Address))..((wy + height) / (8 * self.scale as Address)),
                (wx / (8 * self.scale as Address))..((wx + width) / (8 * self.scale as Address)),
                wx,
                wy,
            )
        };

        let tile_map: [[Byte; 32]; 32] = self.fetch_tile_map(mmu);
        for map_y in tile_y_range.clone() {
            for map_x in tile_x_range.clone() {
                let tile_id: Address = tile_map[map_y as usize][map_x as usize] as Address;
                let tile: Tile = mmu.get_tile(tile_id);
                let pixels: [[Byte; 8]; 8] = tile.get_pixels();

                self.draw_tile(
                    buffer,
                    map_x,
                    map_y,
                    &pixels,
                    Some(tile_wx),
                    Some(tile_wy),
                    Some(width),
                    Some(height),
                );
            }
        }

        if is_debug {
            self.draw_screen_border(buffer, wx, wy, Colors::Debug);
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

    fn draw_tile(
        &self,
        buffer: &mut [u32],
        map_x: Address,
        map_y: Address,
        pixels: &[[Byte; 8]; 8],
        wx: Option<Address>,
        wy: Option<Address>,
        width: Option<Address>,
        height: Option<Address>,
    ) {
        for y in 0..8 {
            for x in 0..8 {
                let color_index: Byte = pixels[y][x];
                let color: Colors = self.palette.get_color(color_index);

                if let (Some(wx), Some(wy), Some(width), Some(height)) = (wx, wy, width, height) {
                    let screen_x: Address = ((map_x * 8 + x) * self.scale as Address) - wx;
                    let screen_y: Address = ((map_y * 8 + y) * self.scale as Address) - wy;

                    for dy in 0..self.scale as Address {
                        for dx in 0..self.scale as Address {
                            let buf_x: Address = screen_x + dx;
                            let buf_y: Address = screen_y + dy;
                            if buf_x < width && buf_y < height {
                                buffer[(buf_y * width + buf_x) as usize] = color.to_rgb();
                            }
                        }
                    }
                } else {
                    self.draw_pixel(buffer, map_x, map_y, x, y, color);
                }
            }
        }
    }

    fn draw_pixel(
        &self,
        buffer: &mut [u32],
        map_x: Address,
        map_y: Address,
        x: Address,
        y: Address,
        color: Colors,
    ) {
        let screen_x: Address = (map_x * 8 + x) * self.scale as Address;
        let screen_y: Address = (map_y * 8 + y) * self.scale as Address;

        for dy in 0..self.scale as Address {
            for dx in 0..self.scale as Address {
                let buf_x: Address = screen_x + dx;
                let buf_y: Address = screen_y + dy;
                if buf_x < self.full_width as Address * self.scale as Address
                    && buf_y < self.full_height as Address * self.scale as Address
                {
                    buffer[buf_y * (self.full_width as Address * self.scale as Address) + buf_x] =
                        color.to_rgb();
                }
            }
        }
    }

    pub fn draw_screen_border(&self, buffer: &mut [u32], wx: Address, wy: Address, color: Colors) {
        let sw: Address = self.screen_width as Address;
        let sh: Address = self.screen_height as Address;
        let scale: Address = self.scale as Address;
        let max_x: Address = wx + sw * scale;
        let max_y: Address = wy + sh * scale;

        for x in wx..max_x {
            for dy in 0..scale {
                let y_top: Address = wy + dy;
                self.set_pixel(buffer, x, y_top, color);
                let y_bottom: Address = max_y - 1 - dy;
                self.set_pixel(buffer, x, y_bottom, color);
            }
        }
        for y in wy..max_y {
            for dx in 0..scale {
                let x_left: Address = wx + dx;
                self.set_pixel(buffer, x_left, y, color);
                let x_right: Address = max_x - 1 - dx;
                self.set_pixel(buffer, x_right, y, color);
            }
        }
    }

    fn set_pixel(&self, buffer: &mut [u32], x: Address, y: Address, color: Colors) {
        let width: Address = self.full_width as Address * self.scale as Address;
        let height: Address = self.full_height as Address * self.scale as Address;
        if x < width && y < height {
            buffer[y * width + x] = color.to_rgb();
        }
    }
}
