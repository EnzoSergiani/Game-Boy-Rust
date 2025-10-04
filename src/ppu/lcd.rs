use crate::{
    common::{
        address::TILE_SET,
        types::{Address, Byte},
    },
    ppu::{
        palette::{Colors, Palette},
        ppu::PPU,
        tile::Tile,
    },
};
use cairo::{Context, Format, ImageSurface};

pub struct LCD {
    screen_height: i32,
    screen_width: i32,
    scale: i32,
    frame_count: Address,
}

impl LCD {
    pub fn new() -> Self {
        LCD {
            screen_width: 160,
            screen_height: 144,
            scale: 3,
            frame_count: 0,
        }
    }

    pub fn render_frame(&mut self, ppu: &mut PPU) -> ImageSurface {
        let surface: ImageSurface = ImageSurface::create(
            Format::Rgb24,
            (self.screen_width * self.scale) as i32,
            (self.screen_height * self.scale) as i32,
        )
        .unwrap();
        let context = Context::new(&surface).unwrap();
        context.scale(self.scale as f64, self.scale as f64);

        let palette: Palette = Palette::from_colors(
            Colors::White,
            Colors::LightGray,
            Colors::DarkGray,
            Colors::Black,
        );

        let (scx, scy) = ppu.get_screen_scroll();

        for screen_y in 0..self.screen_height {
            for screen_x in 0..self.screen_width {
                let mut map_x: i32 = scx as i32 + screen_x;
                let mut map_y: i32 = scy as i32 + screen_y;

                if map_x >= 256 {
                    map_x -= 256;
                } else if map_x < 0 {
                    map_x += 256;
                }

                if map_y >= 256 {
                    map_y -= 256;
                } else if map_y < 0 {
                    map_y += 256;
                }

                let tile_x: Address = (map_x / 8) as Address;
                let tile_y: Address = (map_y / 8) as Address;
                let pixel_x: Address = (map_x % 8) as Address;
                let pixel_y: Address = (map_y % 8) as Address;

                let tile_id: Address = ppu.get_tile_id(tile_x, tile_y);
                let tile: Tile = ppu.get_tile(tile_id);
                let color: Byte = tile.get_pixel(pixel_x, pixel_y);
                let color: Colors = palette.get_color(color);
                let (r, g, b) = color.to_tuple(false);

                context.set_source_rgb(r, g, b);
                context.rectangle(screen_x as f64, screen_y as f64, 1.0, 1.0);
                context.fill().unwrap();
            }
        }

        self.frame_count += 1;
        surface
    }

    pub fn render_debug_tile_map(&mut self, ppu: &mut PPU) -> ImageSurface {
        let surface: ImageSurface = ImageSurface::create(Format::Rgb24, 256, 256).unwrap();
        let context: Context = Context::new(&surface).unwrap();

        context.set_source_rgb(1.0, 1.0, 1.0);
        context.paint().unwrap();

        let palette: Palette = Palette::from_colors(
            Colors::White,
            Colors::LightGray,
            Colors::DarkGray,
            Colors::Black,
        );

        for y in 0..32 {
            for x in 0..32 {
                let tile_id: Address = ppu.get_tile_id(x, y);
                let tile: Tile = Tile::from_address(ppu, tile_id);

                for ty in 0..8 {
                    for tx in 0..8 {
                        let color: Byte = tile.get_pixel(tx, ty);
                        let color: Colors = palette.get_color(color);
                        let (r, g, b) = color.to_tuple(false);

                        context.set_source_rgb(r, g, b);
                        context.rectangle((x * 8 + tx) as f64, (y * 8 + ty) as f64, 1.0, 1.0);
                        context.fill().unwrap();
                    }
                }
            }
        }

        let tile_set_x: f64 = 256.0 + 20.0;
        let vram_tile_count: usize = (TILE_SET.end - TILE_SET.start + 1) / 16;
        let tiles_per_row: usize = 16;
        let tile_size: f64 = 16.0;

        for tile_id in 0..vram_tile_count {
            let row: f64 = (tile_id / tiles_per_row) as f64;
            let col: f64 = (tile_id % tiles_per_row) as f64;
            let x: f64 = tile_set_x + (col * tile_size);
            let y: f64 = row * tile_size;

            let tile: Tile = ppu.get_tile(tile_id);
            let pixels: [[Byte; 8]; 8] = tile.get_pixels();

            for (py, row) in pixels.iter().enumerate() {
                for (px, &pixel) in row.iter().enumerate() {
                    let color: Colors = palette.get_color(pixel);
                    let (r, g, b) = color.to_tuple(false);

                    context.set_source_rgb(r, g, b);
                    context.rectangle(x + (px as f64 * 2.0), y + (py as f64 * 2.0), 2.0, 2.0);
                    context.fill().unwrap();
                }
            }
        }

        context.set_source_rgb(1.0, 0.0, 0.0);
        context.set_line_width(1.0);

        let draw_border = |x: f64, y: f64, width: f64, height: f64| {
            let x: f64 = if x < 0.0 { x + 256.0 } else { x };
            let y: f64 = if y < 0.0 { y + 256.0 } else { y };

            if x + width > 256.0 && y + height > 256.0 {
                let first_width: f64 = 256.0 - x;
                let first_height: f64 = 256.0 - y;

                context.rectangle(x + 0.5, y + 0.5, first_width, first_height);
                context.rectangle(0.5, y + 0.5, width - first_width, first_height);
                context.rectangle(x + 0.5, 0.5, first_width, height - first_height);
                context.rectangle(0.5, 0.5, width - first_width, height - first_height);
            } else if x + width > 256.0 {
                let first_width = 256.0 - x;
                context.rectangle(x + 0.5, y + 0.5, first_width, height);
                context.rectangle(0.5, y + 0.5, width - first_width, height);
            } else if y + height > 256.0 {
                let first_height = 256.0 - y;
                context.rectangle(x + 0.5, y + 0.5, width, first_height);
                context.rectangle(x + 0.5, 0.5, width, height - first_height);
            } else {
                context.rectangle(x + 0.5, y + 0.5, width, height);
            }
        };

        let (scx, scy) = ppu.get_screen_scroll();

        draw_border(scx, scy, 1.0, self.screen_height as f64);
        draw_border(
            scx + self.screen_width as f64 - 1.0,
            scy,
            1.0,
            self.screen_height as f64,
        );
        draw_border(scx, scy, self.screen_width as f64, 1.0);
        draw_border(
            scx,
            scy + self.screen_height as f64 - 1.0,
            self.screen_width as f64,
            1.0,
        );

        context.stroke().unwrap();

        surface
    }

    pub fn render_tile_set(&mut self, ppu: &mut PPU) -> ImageSurface {
        let tile_size: f64 = 16.0;
        let tiles_per_row: usize = 16;
        let surface_width: i32 = (tiles_per_row as f64 * tile_size) as i32;
        let surface_height: i32 = 256;

        let surface: ImageSurface =
            ImageSurface::create(Format::Rgb24, surface_width, surface_height).unwrap();
        let context: Context = Context::new(&surface).unwrap();

        context.set_source_rgb(0.0, 0.0, 0.0);
        context.paint().unwrap();

        let palette: Palette = Palette::from_colors(
            Colors::White,
            Colors::LightGray,
            Colors::DarkGray,
            Colors::Black,
        );

        let vram_tile_count: usize = (TILE_SET.end - TILE_SET.start + 1) / 16;

        for tile_id in 0..vram_tile_count {
            let row: f64 = (tile_id / tiles_per_row) as f64;
            let col: f64 = (tile_id % tiles_per_row) as f64;
            let x: f64 = col * tile_size;
            let y: f64 = row * tile_size;

            let tile: Tile = ppu.get_tile(tile_id);
            let pixels: [[Byte; 8]; 8] = tile.get_pixels();

            for (py, row) in pixels.iter().enumerate() {
                for (px, &pixel) in row.iter().enumerate() {
                    let color = palette.get_color(pixel);
                    let (r, g, b) = color.to_tuple(false);

                    context.set_source_rgb(r, g, b);
                    context.rectangle(x + (px as f64 * 2.0), y + (py as f64 * 2.0), 2.0, 2.0);
                    context.fill().unwrap();
                }
            }
        }

        surface
    }

    pub fn get_screen_dimensions(&self) -> (i32, i32) {
        (
            self.screen_width * self.scale,
            self.screen_height * self.scale,
        )
    }

    pub fn get_frame_count(&self) -> Address {
        self.frame_count
    }
}
