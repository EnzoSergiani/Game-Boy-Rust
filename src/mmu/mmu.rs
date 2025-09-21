use crate::{
    cartridge::cartridge::Cartridge,
    cpu::cpu::{CPU, IME},
    ppu::ppu::PPU,
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

// From cartridge, usually a fixed bank
pub const ADDRESS_ROM: AddressRange = AddressRange {
    start: 0x0000,
    end: 0x7FFF,
};
// In CGB mode, switchable bank 0/1
pub const ADDRESS_VRAM: AddressRange = AddressRange {
    start: 0x8000,
    end: 0x9FFF,
};
// From cartridge, switchable bank if any
pub const ADDRESS_RAM: AddressRange = AddressRange {
    start: 0xA000,
    end: 0xBFFF,
};
// Work RAM
pub const ADDRESS_WRAM: AddressRange = AddressRange {
    start: 0xC000,
    end: 0xDFFF,
};
// Nintendo says use of this area is prohibited.
pub const ADDRESS_ECHO: AddressRange = AddressRange {
    start: 0xE000,
    end: 0xFDFF,
};
// Object Attribute Memory
pub const ADDRESS_OAM: AddressRange = AddressRange {
    start: 0xFE00,
    end: 0xFE9F,
};
// Nintendo says use of this area is prohibited.
pub const ADDRESS_INVALID_OAM: AddressRange = AddressRange {
    start: 0xFEA0,
    end: 0xFEFF,
};
// I/O Registers
pub const ADDRESS_IO: AddressRange = AddressRange {
    start: 0xFF00,
    end: 0xFF7F,
};
// High RAM
pub const ADDRESS_HRAM: AddressRange = AddressRange {
    start: 0xFF80,
    end: 0xFFFE,
};
// Interrupt Enable Register
pub const ADDRESS_IE_REGISTER: AddressRange = AddressRange {
    start: 0xFFFF,
    end: 0xFFFF,
};

const SIZE_WRAM: Size = ADDRESS_WRAM.end - ADDRESS_WRAM.start + 1;
const SIZE_HRAM: Size = ADDRESS_HRAM.end - ADDRESS_HRAM.start + 1;

pub struct MMU {
    cpu: CPU,
    ppu: PPU,
    cartridge: Cartridge,
    wram: [Byte; SIZE_WRAM],
    hram: [Byte; SIZE_HRAM],
}

impl MMU {
    pub fn new() -> Self {
        MMU {
            cpu: CPU::new(),
            ppu: PPU::new(),
            cartridge: Cartridge::eject(),
            wram: [DEFAULT_BYTE; SIZE_WRAM],
            hram: [DEFAULT_BYTE; SIZE_HRAM],
        }
    }

    pub fn set_cartridge(&mut self, cartridge: Cartridge) {
        cartridge.print_data();
        if !cartridge.is_valid() {
            panic!("Invalid cartridge inserted!");
        }
        self.cartridge = cartridge;
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
            0xFF
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
            // Do nothing
        } else if address >= ADDRESS_HRAM.start && address <= ADDRESS_HRAM.end {
            self.write_hram(address, value);
        } else if address >= ADDRESS_IE_REGISTER.start && address <= ADDRESS_IE_REGISTER.end {
            // Do nothing
        } else {
            panic!("Invalid memory write at address: 0x{:04X}", address);
        }
    }

    pub fn print_vram(&mut self) {
        self.ppu.debug();
    }

    pub fn start_cpu(&mut self, entry_point: Address) {
        self.cpu.start(entry_point);
    }

    pub fn run(&mut self) {
        loop {
            let mut cpu: CPU = std::mem::replace(&mut self.cpu, CPU::new());
            cpu.step(self);
            self.cpu = cpu;
        }
    }
}
