use crate::{cpu::cpu::Opcode, memory::address::*};
use core::panic;

pub struct Memory {}

impl Memory {
    pub fn read_memory(&self, address: Address) -> Opcode {
        if address >= ADDRESS_ROM.start && address <= ADDRESS_ROM.end {
            0
        } else if address >= ADDRESS_VRAM.start && address <= ADDRESS_VRAM.end {
            0
        } else if address >= ADDRESS_RAM.start && address <= ADDRESS_RAM.end {
            0
        } else if address >= ADDRESS_WRAM.start && address <= ADDRESS_WRAM.end {
            0
        } else if address >= ADDRESS_ECHO.start && address <= ADDRESS_ECHO.end {
            0
        } else if address >= ADDRESS_OAM.start && address <= ADDRESS_OAM.end {
            0
        } else if address >= ADDRESS_INVALID_OAM.start && address <= ADDRESS_INVALID_OAM.end {
            0
        } else if address >= ADDRESS_IO.start && address <= ADDRESS_IO.end {
            0
        } else if address >= ADDRESS_HRAM.start && address <= ADDRESS_HRAM.end {
            0
        } else if address >= ADDRESS_IE_REGISTER.start && address <= ADDRESS_IE_REGISTER.end {
            0
        } else {
            panic!("Invalid memory read at address: 0x{:04X}", address);
        }
    }
}
