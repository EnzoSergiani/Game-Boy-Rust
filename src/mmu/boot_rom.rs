use super::{
    address::ADDRESS,
    mmu::{Byte, MMU},
};
use std::sync::{Mutex, MutexGuard, OnceLock};

pub struct BootAnimation {
    current_y: Byte,
    target_y: Byte,
    speed: Byte,
    is_complete: bool,
    is_active: bool,
}

pub static BOOT_ANIMATION: OnceLock<Mutex<BootAnimation>> = OnceLock::new();

impl BootAnimation {
    pub fn new() -> Self {
        Self {
            current_y: 80,
            target_y: 0,
            speed: 1,
            is_complete: false,
            is_active: false,
        }
    }

    pub fn start(&mut self) {
        self.is_active = true;
    }

    pub fn update(&mut self) -> Byte {
        if !self.is_active {
            return self.current_y;
        }

        if self.current_y > self.target_y {
            self.current_y = self.current_y.saturating_sub(self.speed);
        } else {
            self.is_complete = true;
        }
        self.current_y
    }

    pub fn is_complete(&self) -> bool {
        self.is_complete
    }
}

pub trait BootROM {
    fn boot_init_sequence(&mut self);
    fn boot_update_animation(&mut self) -> bool;
}

impl BootROM for MMU {
    fn boot_init_sequence(&mut self) {
        self.get_cartridge().print_data();
        self.get_ppu().reset_vram();

        for idx in ADDRESS::TILE_MAP.start..ADDRESS::TILE_MAP.end {
            self.write_memory(idx, 99);
        }

        let logo_nintendo: [[u8; 12]; 32] = self.get_nintendo_logo();
        self.print_logo(logo_nintendo);

        self.set_screen_position(0, 80);

        let animation_mutex: &Mutex<BootAnimation> =
            BOOT_ANIMATION.get_or_init(|| Mutex::new(BootAnimation::new()));
        let mut animation: MutexGuard<'_, BootAnimation> = animation_mutex.lock().unwrap();
        animation.start();
    }

    fn boot_update_animation(&mut self) -> bool {
        let (wx, _) = self.get_screen_position();
        let animation_mutex: &Mutex<BootAnimation> =
            BOOT_ANIMATION.get_or_init(|| Mutex::new(BootAnimation::new()));

        let mut animation: MutexGuard<'_, BootAnimation> = animation_mutex.lock().unwrap();
        let new_y: Byte = animation.update();
        self.set_screen_position(wx as Byte, new_y);

        if animation.is_complete() {
            if MMU::delay_frames(30) {
                self.get_ppu().reset_vram();
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
