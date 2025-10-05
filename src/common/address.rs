use crate::common::types::{Address, Size};

pub struct AddressRange {
    pub start: Address,
    pub end: Address,
    pub size: Size,
}
pub struct AddressOffset {
    pub offset: Address,
}

pub struct ADDRESS;

/* === Memory Map ===  */
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

/* === Cartridge header ===  */
pub const ENTRY_POINT: AddressRange = AddressRange {
    start: 0x0100,
    end: 0x0103,
    size: 0x0103 - 0x0100 + 1,
};
pub const NINTENDO_LOGO: AddressRange = AddressRange {
    start: 0x0104,
    end: 0x0133,
    size: 0x0133 - 0x0104 + 1,
};
pub const TITLE: AddressRange = AddressRange {
    start: 0x0134,
    end: 0x0143,
    size: 0x0143 - 0x0134 + 1,
};
pub const MANUFACTURER_CODE: AddressRange = AddressRange {
    start: 0x013F,
    end: 0x0142,
    size: 0x0142 - 0x013F + 1,
};
pub const NEW_LICENSEE_CODE: AddressRange = AddressRange {
    start: 0x0144,
    end: 0x0145,
    size: 0x0145 - 0x0144 + 1,
};
pub const SGB_FLAG: AddressOffset = AddressOffset { offset: 0x0146 };
pub const CARTRIDGE_TYPE: AddressOffset = AddressOffset { offset: 0x0147 };
pub const ROM_SIZE: AddressOffset = AddressOffset { offset: 0x0148 };
pub const RAM_SIZE: AddressOffset = AddressOffset { offset: 0x0149 };
pub const DESTINATION_CODE: AddressOffset = AddressOffset { offset: 0x014A };
pub const OLD_LICENSEE_CODE: AddressOffset = AddressOffset { offset: 0x014B };
pub const MASK_ROM_VERSION: AddressOffset = AddressOffset { offset: 0x014C };
pub const HEADER_CHECKSUM: AddressOffset = AddressOffset { offset: 0x014D };
pub const GLOBAL_CHECKSUM: AddressRange = AddressRange {
    start: 0x014E,
    end: 0x014F,
    size: 0x014F - 0x014E + 1,
};

/* ===  ===  */
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
