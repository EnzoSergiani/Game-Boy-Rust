use crate::{cartridge::cartridge::Cartridge, lcd::lcd::LCD, mmu::mmu::MMU};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        {Arc, Mutex},
    },
    thread,
};

pub struct Emulator {
    mmu: Arc<Mutex<MMU>>,
    lcd_debug: LCD,
    lcd_screen: LCD,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            mmu: Arc::new(Mutex::new(MMU::new())),
            lcd_debug: LCD::new(),
            lcd_screen: LCD::new(),
        }
    }

    pub fn start_cartridge(&mut self, path: &str) {
        let cartridge: Cartridge = Cartridge::insert(path);
        {
            let mut mmu: std::sync::MutexGuard<'_, MMU> = self.mmu.lock().unwrap();
            mmu.set_cartridge(cartridge);
        }

        let start_signal: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

        let mmu_debug: Arc<Mutex<MMU>> = Arc::clone(&self.mmu);
        let mmu_screen: Arc<Mutex<MMU>> = Arc::clone(&self.mmu);
        let signal_debug: Arc<AtomicBool> = Arc::clone(&start_signal);
        let signal_screen: Arc<AtomicBool> = Arc::clone(&start_signal);

        let mut lcd_debug: LCD = self.lcd_debug.clone();
        let mut lcd_screen: LCD = self.lcd_screen.clone();

        let handle_debug: thread::JoinHandle<()> = thread::spawn(move || {
            lcd_debug.init_screen(&mmu_debug, true, signal_debug);
        });

        let handle_screen: thread::JoinHandle<()> = thread::spawn(move || {
            lcd_screen.init_screen(&mmu_screen, false, signal_screen);
        });

        println!("Appuie sur une touche pour démarrer les deux fenêtres...");
        use std::io::{self, Read};
        let _ = io::stdin().read(&mut [0u8]).unwrap();

        start_signal.store(true, Ordering::SeqCst);

        handle_debug.join().unwrap();
        handle_screen.join().unwrap();
    }
}
