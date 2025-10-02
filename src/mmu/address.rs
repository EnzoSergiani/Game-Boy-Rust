pub type Address = usize;
pub type Size = usize;

pub struct AddressRange {
    pub start: Address,
    pub end: Address,
    pub size: Size,
}

pub struct ADDRESS;

impl ADDRESS {
    pub const ROM: AddressRange = AddressRange {
        start: 0x0000,
        end: 0x7FFF,
        size: 0x7FFF - 0x0000 + 1,
    };
    pub const VRAM: AddressRange = AddressRange {
        start: 0x8000,
        end: 0x9FFF,
        size: 0x9FFF - 0x8000 + 1,
    };
    pub const RAM: AddressRange = AddressRange {
        start: 0xA000,
        end: 0xBFFF,
        size: 0xBFFF - 0xA000 + 1,
    };
    pub const WRAM: AddressRange = AddressRange {
        start: 0xC000,
        end: 0xDFFF,
        size: 0xDFFF - 0xC000 + 1,
    };
    pub const ECHO: AddressRange = AddressRange {
        start: 0xE000,
        end: 0xFDFF,
        size: 0xFDFF - 0xE000 + 1,
    };
    pub const OAM: AddressRange = AddressRange {
        start: 0xFE00,
        end: 0xFE9F,
        size: 0xFE9F - 0xFE00 + 1,
    };
    pub const INVALID_OAM: AddressRange = AddressRange {
        start: 0xFEA0,
        end: 0xFEFF,
        size: 0xFEFF - 0xFEA0 + 1,
    };
    pub const IO: AddressRange = AddressRange {
        start: 0xFF00,
        end: 0xFF7F,
        size: 0xFF7F - 0xFF00 + 1,
    };
    pub const HRAM: AddressRange = AddressRange {
        start: 0xFF80,
        end: 0xFFFE,
        size: 0xFFFE - 0xFF80 + 1,
    };
    pub const IE_REGISTER: AddressRange = AddressRange {
        start: 0xFFFF,
        end: 0xFFFF,
        size: 0xFFFF - 0xFFFF + 1,
    };
    pub const BOOT_ROM: AddressRange = AddressRange {
        start: 0x0000,
        end: 0x00FF,
        size: 0x00FF - 0x0000 + 1,
    };
    pub const NINTENDO_LOGO: AddressRange = AddressRange {
        start: 0x104,
        end: 0x133,
        size: 0x133 - 0x104 + 1,
    };
    pub const TILE_SET: AddressRange = AddressRange {
        start: 0x8000,
        end: 0x97FF,
        size: 0x97FF - 0x8000 + 1,
    };

    pub const TILE_MAP: AddressRange = AddressRange {
        start: 0x9800,
        end: 0x9FFF,
        size: 0x9FFF - 0x9800 + 1,
    };
}
