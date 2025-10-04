use crate::{
    cartridge::cartridge::Cartridge,
    common::{
        address::{
            ECHO, HRAM, IE_REGISTER, INVALID_OAM, IO, NINTENDO_LOGO, OAM, RAM, ROM, VRAM, WRAM,
        },
        constant::DEFAULT_BYTE,
        types::{Address, Byte},
    },
    // cpu::cpu::CPU,
    mmu::boot_rom::BootROM,
    ppu::ppu::PPU,
};
use std::sync::{Mutex, MutexGuard, OnceLock};

pub static FRAME_COUNTER: OnceLock<Mutex<Byte>> = OnceLock::new();

pub struct MMU {
    // cpu: CPU,
    ppu: PPU,
    cartridge: Cartridge,
    wram: [Byte; WRAM.size],
    hram: [Byte; HRAM.size],
    io: [Byte; IO.size],
}

impl MMU {
    pub fn new() -> Self {
        MMU {
            // cpu: CPU::new(),
            ppu: PPU::new(),
            cartridge: Cartridge::eject(),
            wram: [DEFAULT_BYTE; WRAM.size],
            hram: [DEFAULT_BYTE; HRAM.size],
            io: [DEFAULT_BYTE; IO.size],
        }
    }

    // pub fn get_cpu(&mut self) -> &mut CPU {
    //     &mut self.cpu
    // }

    pub fn get_ppu(&mut self) -> &mut PPU {
        &mut self.ppu
    }

    pub fn get_cartridge(&mut self) -> &mut Cartridge {
        &mut self.cartridge
    }

    fn read_wram(&self, address: Address) -> Byte {
        self.wram[address - WRAM.start]
    }

    fn write_wram(&mut self, address: Address, value: Byte) {
        self.wram[address - WRAM.start] = value;
    }

    fn read_hram(&self, address: Address) -> Byte {
        self.hram[address - HRAM.start]
    }

    fn write_hram(&mut self, address: Address, value: Byte) {
        self.hram[address - HRAM.start] = value;
    }

    fn read_io(&self, address: Address) -> Byte {
        self.io[address - IO.start]
    }

    fn write_io(&mut self, address: Address, value: Byte) {
        self.io[address - IO.start] = value;
    }

    pub fn read_memory(&self, address: Address) -> Byte {
        if address >= ROM.start && address <= ROM.end {
            self.cartridge.read_rom(address)
        } else if address >= VRAM.start && address <= VRAM.end {
            self.ppu.read_vram(address)
        } else if address >= RAM.start && address <= RAM.end {
            self.cartridge.read_ram(address)
        } else if address >= WRAM.start && address <= WRAM.end {
            self.read_wram(address)
        } else if address >= ECHO.start && address <= ECHO.end {
            0xFF
        } else if address >= OAM.start && address <= OAM.end {
            self.ppu.read_oam(address)
        } else if address >= INVALID_OAM.start && address <= INVALID_OAM.end {
            0xFF
        } else if address >= IO.start && address <= IO.end {
            self.read_io(address)
        } else if address >= HRAM.start && address <= HRAM.end {
            self.read_hram(address)
        } else if address >= IE_REGISTER.start && address <= IE_REGISTER.end {
            0xFF
        } else {
            panic!("Invalid memory read at address: 0x{:04X}", address);
        }
    }

    pub fn write_memory(&mut self, address: Address, value: Byte) {
        if address >= ROM.start && address <= ROM.end {
            println!(
                "Tentative d'écriture en ROM à 0x{:04X} avec valeur 0x{:02X}",
                address, value
            );
            // panic!("Cannot write to ROM address: 0x{:04X}", address);
        } else if address >= VRAM.start && address <= VRAM.end {
            self.ppu.write_vram(address, value);
        } else if address >= RAM.start && address <= RAM.end {
            self.cartridge.write_ram(address, value);
        } else if address >= WRAM.start && address <= WRAM.end {
            self.write_wram(address, value);
        } else if address >= ECHO.start && address <= ECHO.end {
            // Do nothing
        } else if address >= OAM.start && address <= OAM.end {
            self.ppu.write_oam(address, value);
        } else if address >= INVALID_OAM.start && address <= INVALID_OAM.end {
            // Do nothing
        } else if address >= IO.start && address <= IO.end {
            self.write_io(address, value);
        } else if address >= HRAM.start && address <= HRAM.end {
            self.write_hram(address, value);
        } else if address >= IE_REGISTER.start && address <= IE_REGISTER.end {
            // Do nothing
        } else {
            panic!("Invalid memory write at address: 0x{:04X}", address);
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

    pub fn set_screen_scroll(&mut self, scx: Byte, scy: Byte) {
        self.ppu.set_screen_scroll(scx, scy);
    }

    pub fn get_screen_scroll(&self) -> (Address, Address) {
        (
            self.read_io(0xFF42) as Address,
            self.read_io(0xFF43) as Address,
        )
    }
}
