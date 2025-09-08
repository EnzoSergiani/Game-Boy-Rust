pub type Address = u16;

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
