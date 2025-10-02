use crate::mmu::mmu::{Address, Byte};

#[derive(Copy, Clone)]
pub enum Colors {
    White,
    LightGray,
    DarkGray,
    Black,
    Debug,
}

#[derive(Clone)]
pub struct Palette {
    pub colors: [Colors; 4],
}

impl Colors {
    pub fn to_tuple(&self) -> (f64, f64, f64) {
        match self {
            Colors::White => (1.0, 1.0, 1.0),
            Colors::LightGray => (0.66, 0.66, 0.66),
            Colors::DarkGray => (0.33, 0.33, 0.33),
            Colors::Black => (0.0, 0.0, 0.0),
            Colors::Debug => (1.0, 0.0, 0.0),
        }
    }
}

impl Palette {
    pub fn new() -> Self {
        Palette {
            colors: [
                Colors::White,
                Colors::LightGray,
                Colors::DarkGray,
                Colors::Black,
            ],
        }
    }

    pub fn set_color(&mut self, index: Address, color: Colors) {
        if index < 4 {
            self.colors[index] = color;
        }
    }

    pub fn get_color(&self, value: Byte) -> Colors {
        self.colors[value as usize & 0x03]
    }

    pub fn from_colors(
        color_idx_0: Colors,
        color_idx_1: Colors,
        color_idx_2: Colors,
        color_idx_3: Colors,
    ) -> Palette {
        let mut palette: Palette = Palette::new();
        palette.set_color(0, color_idx_0);
        palette.set_color(1, color_idx_1);
        palette.set_color(2, color_idx_2);
        palette.set_color(3, color_idx_3);
        palette
    }
}
