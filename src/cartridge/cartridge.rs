use crate::common::{
    address::{
        CARTRIDGE_TYPE, DESTINATION_CODE, ENTRY_POINT, HEADER_CHECKSUM, MANUFACTURER_CODE,
        MASK_ROM_VERSION, NEW_LICENSEE_CODE, NINTENDO_LOGO, OLD_LICENSEE_CODE, RAM, RAM_SIZE, ROM,
        ROM_SIZE, SGB_FLAG, TITLE,
    },
    constant::DEFAULT_BYTE,
    types::{Address, Byte, Size},
};

use std::fs;

pub struct Cartridge {
    rom: [Byte; ROM.size],
    ram: [Byte; RAM.size],
    entry_point_values: [Byte; ENTRY_POINT.size],
    title: String,
    manufacturer_code: Byte,
    licensee_code: String,
    supports_sgb: bool,
    cartridge_type: CartridgeType,
    rom_size: usize,
    rom_number_banks: usize,
    ram_size: usize,
    destination_code: DestinationCode,
    mask_rom_version_number: Byte,
    is_nintendo_logo: bool,
    is_header_checksum_valid: bool,
}

#[derive(Debug)]
enum CartridgeType {
    ROM_Only,
    MBC1,
    MBC1_RAM,
    MBC1_RAM_Battery,
    MBC2,
    MBC2_Battery,
    ROM_RAM,
    ROM_RAM_Battery,
    MMM01,
    MMM01_RAM,
    MMM01_RAM_Battery,
    MBC3_Timer_Battery,
    MBC3_Timer_RAM_Battery,
    MBC3,
    MBC3_RAM,
    MBC3_RAM_Battery,
    MBC5,
    MBC5_RAM,
    MBC5_RAM_Battery,
    MBC5_Rumble,
    MBC5_Rumble_RAM,
    MBC5_Rumble_RAM_Battery,
    Pocket_Camera,
    Bandai_TAMA5,
    HuC3,
    HuC1_RAM_Battery,
    Unknown(Byte),
}

#[derive(Debug)]
enum DestinationCode {
    Japan,
    Overseas,
    Unknown(Byte),
}

impl Cartridge {
    pub fn insert(path: &str) -> Self {
        let bytes_result: Result<Vec<Byte>, &'static str> = Cartridge::read(path);
        match bytes_result {
            Ok(bytes) => {
                let rom: [Byte; ROM.size] = Cartridge::extract_rom(&bytes);
                let ram: [Byte; RAM.size] = Cartridge::extract_ram(&bytes);
                let entry_point_values: [Byte; ENTRY_POINT.size] =
                    Cartridge::extract_entry_point_values(&bytes);
                let title: String = Cartridge::extract_title(&bytes);
                let manufacturer_code: Byte = Cartridge::extract_manufacturer_code(&bytes);
                let licensee_code: String = Cartridge::extract_licensee_code(&bytes);
                let supports_sgb: bool = Cartridge::extract_sgb_flag(&bytes);
                let cartridge_type: CartridgeType = Cartridge::extract_cartridge_type(&bytes);
                let (rom_size, rom_number_banks): (Size, usize) =
                    Cartridge::extract_rom_size(&bytes);
                let ram_size: Size = Cartridge::extract_ram_size(&bytes);
                let destination_code: DestinationCode = Cartridge::extract_destination_code(&bytes);
                let mask_rom_version_number: Byte =
                    Cartridge::extract_mask_rom_version_number(&bytes);
                let is_nintendo_logo: bool = Cartridge::is_nintendo_logo(&bytes);
                let is_header_checksum_valid: bool = Cartridge::is_header_checksum_valid(&bytes);
                Cartridge {
                    rom,
                    ram,
                    entry_point_values,
                    title,
                    manufacturer_code,
                    licensee_code,
                    supports_sgb,
                    cartridge_type,
                    rom_size,
                    rom_number_banks,
                    ram_size,
                    destination_code,
                    mask_rom_version_number,
                    is_nintendo_logo,
                    is_header_checksum_valid,
                }
            }
            Err(_e) => Cartridge::eject(),
        }
    }

    fn read(path: &str) -> Result<Vec<Byte>, &'static str> {
        match fs::read(path) {
            Ok(bytes) => Ok(bytes),
            Err(_e) => Err("Failed to read cartridge"),
        }
    }

    fn extract_rom(bytes: &[Byte]) -> [Byte; ROM.size] {
        let mut rom: [Byte; ROM.size] = [DEFAULT_BYTE; ROM.size];
        for address in ROM.start..=ROM.end {
            rom[address - ROM.start] = bytes[address];
        }
        rom
    }

    fn extract_ram(bytes: &[Byte]) -> [Byte; RAM.size] {
        let mut ram: [Byte; RAM.size] = [DEFAULT_BYTE; RAM.size];
        for address in RAM.start..=RAM.end {
            ram[address - RAM.start] = bytes[address];
        }
        ram
    }

    fn extract_entry_point_values(bytes: &[Byte]) -> [Byte; ENTRY_POINT.size] {
        let mut entry_point_value: [Byte; ENTRY_POINT.size] = [DEFAULT_BYTE; ENTRY_POINT.size];
        for address in ENTRY_POINT.start..=ENTRY_POINT.end {
            entry_point_value[address - ENTRY_POINT.start] = bytes[address];
        }
        entry_point_value
    }

    fn is_nintendo_logo(bytes: &[Byte]) -> bool {
        let mut nintendo_logo_cartridge: [Byte; NINTENDO_LOGO.size] =
            [DEFAULT_BYTE; NINTENDO_LOGO.size];
        for address in NINTENDO_LOGO.start..=NINTENDO_LOGO.end {
            nintendo_logo_cartridge[address - NINTENDO_LOGO.start] = bytes[address];
        }

        let valid_logo: [Byte; NINTENDO_LOGO.size] = [
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];

        nintendo_logo_cartridge == valid_logo
    }

    fn extract_title(bytes: &[Byte]) -> String {
        let title_hex: &[Byte] = &bytes[TITLE.start..TITLE.end + 1];
        String::from_utf8_lossy(title_hex)
            .trim_end_matches('\0')
            .to_string()
    }

    fn extract_manufacturer_code(bytes: &[Byte]) -> Byte {
        let manufacturer_code_hex: &[u8] =
            &bytes[MANUFACTURER_CODE.start..MANUFACTURER_CODE.end + 1];

        manufacturer_code_hex[0]
    }

    fn extract_licensee_code(bytes: &[Byte]) -> String {
        if bytes[OLD_LICENSEE_CODE.offset] != 0x33 {
            match bytes[OLD_LICENSEE_CODE.offset] {
                0x00 => "None".to_string(),
                0x01 => "Nintendo".to_string(),
                0x08 => "Capcom".to_string(),
                0x09 => "HOT-B".to_string(),
                0x0A => "Jaleco".to_string(),
                0x0B => "Coconuts Japan".to_string(),
                0x0C => "Elite Systems".to_string(),
                0x13 => "EA (Electronic Arts)".to_string(),
                0x18 => "Hudson Soft".to_string(),
                0x19 => "ITC Entertainment".to_string(),
                0x1A => "Yanoman".to_string(),
                0x1D => "Japan Clary".to_string(),
                0x1F => "Virgin Games Ltd.".to_string(),
                0x24 => "PCM Complete".to_string(),
                0x25 => "San-X".to_string(),
                0x28 => "Kemco".to_string(),
                0x29 => "SETA Corporation".to_string(),
                0x30 => "Infogrames".to_string(),
                0x31 => "Nintendo".to_string(),
                0x32 => "Bandai".to_string(),
                0x34 => "Konami".to_string(),
                0x35 => "HectorSoft".to_string(),
                0x38 => "Capcom".to_string(),
                0x39 => "Banpresto".to_string(),
                0x3C => "Entertainment Interactive".to_string(),
                0x3E => "Gremlin".to_string(),
                0x41 => "Ubi Soft".to_string(),
                0x42 => "Atlus".to_string(),
                0x44 => "Malibu Interactive".to_string(),
                0x46 => "Angel".to_string(),
                0x47 => "Spectrum HoloByte".to_string(),
                0x49 => "Irem".to_string(),
                0x4A => "Virgin Games Ltd.".to_string(),
                0x4D => "Malibu Interactive".to_string(),
                0x4F => "U.S. Gold".to_string(),
                0x50 => "Absolute".to_string(),
                0x51 => "Acclaim Entertainment".to_string(),
                0x52 => "Activision".to_string(),
                0x53 => "Sammy USA Corporation".to_string(),
                0x54 => "GameTek".to_string(),
                0x55 => "Park Place".to_string(),
                0x56 => "LJN".to_string(),
                0x57 => "Matchbox".to_string(),
                0x59 => "Milton Bradley Company".to_string(),
                0x5A => "Mindscape".to_string(),
                0x5B => "Romstar".to_string(),
                0x5C => "Naxat Soft".to_string(),
                0x5D => "Tradewest".to_string(),
                0x60 => "Titus Interactive".to_string(),
                0x61 => "Virgin Games Ltd.".to_string(),
                0x67 => "Ocean Software".to_string(),
                0x69 => "EA (Electronic Arts)".to_string(),
                0x6E => "Elite Systems".to_string(),
                0x6F => "Electro Brain".to_string(),
                0x70 => "Infogrames".to_string(),
                0x71 => "Interplay Entertainment".to_string(),
                0x72 => "Broderbund".to_string(),
                0x73 => "Sculptured Software".to_string(),
                0x75 => "The Sales Curve Limited".to_string(),
                0x78 => "THQ".to_string(),
                0x79 => "Accolade".to_string(),
                0x7A => "Triffix Entertainment".to_string(),
                0x7C => "MicroProse".to_string(),
                0x7F => "Kemco".to_string(),
                0x80 => "Misawa Entertainment".to_string(),
                0x83 => "LOZC G.".to_string(),
                0x86 => "Tokuma Shoten".to_string(),
                0x8B => "Bullet-Proof Software".to_string(),
                0x8C => "Vic Tokai Corp.".to_string(),
                0x8E => "Ape Inc.".to_string(),
                0x8F => "I’Max".to_string(),
                0x91 => "Chunsoft Co.".to_string(),
                0x92 => "Video System".to_string(),
                0x93 => "Tsubaraya Productions".to_string(),
                0x95 => "Varie".to_string(),
                0x96 => "Yonezawa/S’Pal".to_string(),
                0x97 => "Kemco".to_string(),
                0x99 => "Arc".to_string(),
                0x9A => "Nihon Bussan".to_string(),
                0x9B => "Tecmo".to_string(),
                0x9C => "Imagineer".to_string(),
                0x9D => "Banpresto".to_string(),
                0x9F => "Nova".to_string(),
                0xA1 => "Hori Electric".to_string(),
                0xA2 => "Bandai".to_string(),
                0xA4 => "Konami".to_string(),
                0xA6 => "Kawada".to_string(),
                0xA7 => "Takara".to_string(),
                0xA9 => "Technos Japan".to_string(),
                0xAA => "Broderbund".to_string(),
                0xAC => "Toei Animation".to_string(),
                0xAD => "Toho".to_string(),
                0xAF => "Namco".to_string(),
                0xB0 => "Acclaim Entertainment".to_string(),
                0xB1 => "ASCII Corporation or Nexsoft".to_string(),
                0xB2 => "Bandai".to_string(),
                0xB4 => "Square Enix".to_string(),
                0xB6 => "HAL Laboratory".to_string(),
                0xB7 => "SNK".to_string(),
                0xB9 => "Pony Canyon".to_string(),
                0xBA => "Culture Brain".to_string(),
                0xBB => "Sunsoft".to_string(),
                0xBD => "Sony Imagesoft".to_string(),
                0xBF => "Sammy Corporation".to_string(),
                0xC0 => "Taito".to_string(),
                0xC2 => "Kemco".to_string(),
                0xC3 => "Square".to_string(),
                0xC4 => "Tokuma Shoten".to_string(),
                0xC5 => "Data East".to_string(),
                0xC6 => "Tonkin House".to_string(),
                0xC8 => "Koei".to_string(),
                0xC9 => "UFL".to_string(),
                0xCA => "Ultra Games".to_string(),
                0xCB => "VAP, Inc.".to_string(),
                0xCC => "Use Corporation".to_string(),
                0xCD => "Meldac".to_string(),
                0xCE => "Pony Canyon".to_string(),
                0xCF => "Angel".to_string(),
                0xD0 => "Taito".to_string(),
                0xD1 => "SOFEL (Software Engineering Lab)".to_string(),
                0xD2 => "Quest".to_string(),
                0xD3 => "Sigma Enterprises".to_string(),
                0xD4 => "ASK Kodansha Co.".to_string(),
                0xD6 => "Naxat Soft".to_string(),
                0xD7 => "Copya System".to_string(),
                0xD9 => "Banpresto".to_string(),
                0xDA => "Tomy".to_string(),
                0xDB => "LJN".to_string(),
                0xDD => "Nippon Computer Systems".to_string(),
                0xDE => "Human Ent.".to_string(),
                0xDF => "Altron".to_string(),
                0xE0 => "Jaleco".to_string(),
                0xE1 => "Towa Chiki".to_string(),
                0xE2 => "Yutaka".to_string(),
                0xE3 => "Varie".to_string(),
                0xE5 => "Epoch".to_string(),
                0xE7 => "Athena".to_string(),
                0xE8 => "Asmik Ace Entertainment".to_string(),
                0xE9 => "Natsume".to_string(),
                0xEA => "King Records".to_string(),
                0xEB => "Atlus".to_string(),
                0xEC => "Epic/Sony Records".to_string(),
                0xEE => "IGS".to_string(),
                0xF0 => "A Wave".to_string(),
                0xF3 => "Extreme Entertainment".to_string(),
                0xFF => "LJN".to_string(),
                other => format!("Unknown (0x{:02X})", other),
            }
        } else {
            let ascii_licensee_code =
                std::str::from_utf8(&bytes[NEW_LICENSEE_CODE.start..=NEW_LICENSEE_CODE.end])
                    .unwrap_or("00");
            match ascii_licensee_code {
                "00" => "None".to_string(),
                "01" => "Nintendo Research & Development 1".to_string(),
                "08" => "Capcom".to_string(),
                "13" => "EA (Electronic Arts)".to_string(),
                "18" => "Hudson Soft".to_string(),
                "19" => "B-AI".to_string(),
                "20" => "KSS".to_string(),
                "22" => "Planning Office WADA".to_string(),
                "24" => "PCM Complete".to_string(),
                "25" => "San-X".to_string(),
                "28" => "Kemco".to_string(),
                "29" => "SETA Corporation".to_string(),
                "30" => "Viacom".to_string(),
                "31" => "Nintendo".to_string(),
                "32" => "Bandai".to_string(),
                "33" => "Ocean Software/Acclaim Entertainment".to_string(),
                "34" => "Konami".to_string(),
                "35" => "HectorSoft".to_string(),
                "37" => "Taito".to_string(),
                "38" => "Hudson Soft".to_string(),
                "39" => "Banpresto".to_string(),
                "41" => "Ubi Soft".to_string(),
                "42" => "Atlus".to_string(),
                "44" => "Malibu Interactive".to_string(),
                "46" => "Angel".to_string(),
                "47" => "Bullet-Proof Software".to_string(),
                "49" => "Irem".to_string(),
                "50" => "Absolute".to_string(),
                "51" => "Acclaim Entertainment".to_string(),
                "52" => "Activision".to_string(),
                "53" => "Sammy USA Corporation".to_string(),
                "54" => "Konami".to_string(),
                "55" => "Hi Tech Expressions".to_string(),
                "56" => "LJN".to_string(),
                "57" => "Matchbox".to_string(),
                "58" => "Mattel".to_string(),
                "59" => "Milton Bradley Company".to_string(),
                "60" => "Titus Interactive".to_string(),
                "61" => "Virgin Games Ltd.".to_string(),
                "64" => "Lucasfilm Games".to_string(),
                "67" => "Ocean Software".to_string(),
                "69" => "EA (Electronic Arts)".to_string(),
                "70" => "Infogrames".to_string(),
                "71" => "Interplay Entertainment".to_string(),
                "72" => "Broderbund".to_string(),
                "73" => "Sculptured Software".to_string(),
                "75" => "The Sales Curve Limited".to_string(),
                "78" => "THQ".to_string(),
                "79" => "Accolade".to_string(),
                "80" => "Misawa Entertainment".to_string(),
                "83" => "LOZC G.".to_string(),
                "86" => "Tokuma Shoten".to_string(),
                "87" => "Tsukuda Original".to_string(),
                "91" => "Chunsoft Co.".to_string(),
                "92" => "Video System".to_string(),
                "93" => "Ocean Software/Acclaim Entertainment".to_string(),
                "95" => "Varie".to_string(),
                "96" => "Yonezawa/S’Pal".to_string(),
                "97" => "Kaneko".to_string(),
                "99" => "Pack-In-Video".to_string(),
                "9H" => "Bottom Up".to_string(),
                "A4" => "Konami (Yu-Gi-Oh!)".to_string(),
                "BL" => "MTO".to_string(),
                "DK" => "Kodansha".to_string(),
                other => other.to_string(),
            }
        }
    }

    fn extract_sgb_flag(bytes: &[Byte]) -> bool {
        if bytes[SGB_FLAG.offset] == 0x03 {
            return true;
        }
        false
    }

    fn extract_cartridge_type(bytes: &[Byte]) -> CartridgeType {
        match bytes[CARTRIDGE_TYPE.offset] {
            0x00 => CartridgeType::ROM_Only,
            0x01 => CartridgeType::MBC1,
            0x02 => CartridgeType::MBC1_RAM,
            0x03 => CartridgeType::MBC1_RAM_Battery,
            0x05 => CartridgeType::MBC2,
            0x06 => CartridgeType::MBC2_Battery,
            0x08 => CartridgeType::ROM_RAM,
            0x09 => CartridgeType::ROM_RAM_Battery,
            0x0B => CartridgeType::MMM01,
            0x0C => CartridgeType::MMM01_RAM,
            0x0D => CartridgeType::MMM01_RAM_Battery,
            0x0F => CartridgeType::MBC3_Timer_Battery,
            0x10 => CartridgeType::MBC3_Timer_RAM_Battery,
            0x11 => CartridgeType::MBC3,
            0x12 => CartridgeType::MBC3_RAM,
            0x13 => CartridgeType::MBC3_RAM_Battery,
            0x19 => CartridgeType::MBC5,
            0x1A => CartridgeType::MBC5_RAM,
            0x1B => CartridgeType::MBC5_RAM_Battery,
            0x1C => CartridgeType::MBC5_Rumble,
            0x1D => CartridgeType::MBC5_Rumble_RAM,
            0x1E => CartridgeType::MBC5_Rumble_RAM_Battery,
            0x1F => CartridgeType::Pocket_Camera,
            0xFD => CartridgeType::Bandai_TAMA5,
            0xFE => CartridgeType::HuC3,
            0xFF => CartridgeType::HuC1_RAM_Battery,
            other => CartridgeType::Unknown(other),
        }
    }

    fn extract_rom_size(bytes: &[Byte]) -> (usize, usize) {
        match bytes[ROM_SIZE.offset] {
            0x00 => (32 * 1024, 2),
            0x01 => (64 * 1024, 4),
            0x02 => (128 * 1024, 8),
            0x03 => (256 * 1024, 16),
            0x04 => (512 * 1024, 32),
            0x05 => (1 * 1024 * 1024, 64),
            0x06 => (2 * 1024 * 1024, 128),
            0x07 => (4 * 1024 * 1024, 256),
            0x08 => (8 * 1024 * 1024, 512),
            0x52 => (1152 * 1024, 72),
            0x53 => (1280 * 1024, 80),
            0x54 => (1536 * 1024, 96),
            _ => (0, 0),
        }
    }

    fn extract_ram_size(bytes: &[Byte]) -> usize {
        println!(
            "RAM Size Code: 0x{:04X} -> 0x{:02X}",
            RAM_SIZE.offset, bytes[RAM_SIZE.offset]
        );
        match bytes[RAM_SIZE.offset] {
            0x00 => 0,
            0x01 => 2 * 1024,
            0x02 => 8 * 1024,
            0x03 => 32 * 1024,
            0x04 => 128 * 1024,
            0x05 => 64 * 1024,
            _ => 0,
        }
    }

    fn extract_destination_code(bytes: &[Byte]) -> DestinationCode {
        match bytes[DESTINATION_CODE.offset] {
            0x00 => DestinationCode::Japan,
            0x01 => DestinationCode::Overseas,
            other => DestinationCode::Unknown(other),
        }
    }

    fn extract_mask_rom_version_number(bytes: &[Byte]) -> Byte {
        bytes[MASK_ROM_VERSION.offset]
    }

    fn is_header_checksum_valid(bytes: &[Byte]) -> bool {
        let mut checksum: Byte = DEFAULT_BYTE;
        for address in TITLE.start..=MASK_ROM_VERSION.offset {
            checksum = checksum.wrapping_sub(bytes[address]).wrapping_sub(1);
        }
        checksum == bytes[HEADER_CHECKSUM.offset]
    }

    pub fn eject() -> Self {
        Cartridge {
            rom: [DEFAULT_BYTE; ROM.size],
            ram: [DEFAULT_BYTE; RAM.size],
            entry_point_values: [DEFAULT_BYTE; ENTRY_POINT.size],
            title: "none".to_string(),
            manufacturer_code: DEFAULT_BYTE,
            licensee_code: "none".to_string(),
            supports_sgb: false,
            cartridge_type: CartridgeType::ROM_Only,
            rom_size: 0,
            rom_number_banks: 0,
            ram_size: 0,
            destination_code: DestinationCode::Japan,
            mask_rom_version_number: DEFAULT_BYTE,
            is_nintendo_logo: false,
            is_header_checksum_valid: false,
        }
    }

    pub fn get_rom(&self) -> [Byte; ROM.size] {
        self.rom
    }

    pub fn read_rom(&self, address: Address) -> Byte {
        self.rom[address - ROM.start]
    }

    pub fn get_ram(&self) -> [Byte; RAM.size] {
        self.ram
    }

    pub fn read_ram(&self, address: Address) -> Byte {
        let shift: Address = address - RAM.start;
        self.ram[shift]
    }

    pub fn write_ram(&mut self, address: Address, value: Byte) {
        let shift: Address = address - RAM.start;
        self.ram[shift] = value;
    }

    pub fn is_valid(&self) -> bool {
        self.is_nintendo_logo && self.is_header_checksum_valid
    }

    pub fn get_title(&self) -> String {
        let title_bytes: &[u8] = self.title.as_bytes();
        title_bytes
            .iter()
            .take_while(|&&b| b != 0)
            .map(|&b| b as char)
            .collect()
    }

    pub fn get_nintendo_logo(&mut self) -> [[Byte; 12]; 32] {
        let mut logo_bytes_compressed: [Byte; NINTENDO_LOGO.size] =
            [DEFAULT_BYTE; NINTENDO_LOGO.size];
        for (idx, address) in (NINTENDO_LOGO.start..=NINTENDO_LOGO.end).enumerate() {
            logo_bytes_compressed[idx] = self.read_rom(address);
        }

        let logo_bytes_1: [Byte; NINTENDO_LOGO.size / 2] =
            logo_bytes_compressed[0..24].try_into().unwrap();
        let logo_bytes_2: [Byte; NINTENDO_LOGO.size / 2] =
            logo_bytes_compressed[24..48].try_into().unwrap();

        let mut logo_matrix_compressed: [[Byte; 6]; 8] = [[DEFAULT_BYTE; 6]; 8];
        for block in 0..2 {
            let logo_bytes_block: &[u8; _] = if block == 0 {
                &logo_bytes_1
            } else {
                &logo_bytes_2
            };
            for col in 0..12 {
                for row in 0..4 {
                    let idx = col * 4 + row;
                    let byte = logo_bytes_block[idx / 2];
                    let nibble = if idx % 2 == 0 { byte >> 4 } else { byte & 0x0F };
                    let matrix_row = row + block * 4;
                    let matrix_col = col / 2;
                    if col % 2 == 0 && matrix_col < 6 {
                        logo_matrix_compressed[matrix_row][matrix_col] = nibble << 4;
                    } else if col % 2 == 1 && matrix_col < 6 {
                        logo_matrix_compressed[matrix_row][matrix_col] |= nibble;
                    }
                }
            }
        }

        let mut logo_matrix_decompressed: [[Byte; 12]; 32] = [[DEFAULT_BYTE; 12]; 32];
        for row in 0..8 {
            for col in 0..6 {
                let byte = logo_matrix_compressed[row][col];
                for bit in 0..8 {
                    let pixel_on = (byte >> (7 - bit)) & 1;
                    let out_row = row * 4;
                    let out_col = col * 2 + (bit / 4);
                    let bit_in_byte = (bit % 4) * 2;

                    if pixel_on == 1 {
                        for dr in 0..4 {
                            logo_matrix_decompressed[out_row + dr][out_col] |=
                                0b11 << (6 - bit_in_byte);
                        }
                    }
                }
            }
        }

        logo_matrix_decompressed
    }

    pub fn print_data(&self) {
        print!("Entry Point Values: [");
        for (i, byte) in self.entry_point_values.iter().enumerate() {
            if i != 0 {
                print!(", ");
            }
            print!("0x{:02X}", byte);
        }
        println!("]");
        println!("Title: {}", self.title);
        println!("Manufacturer Code: {}", self.manufacturer_code);
        println!("Licensee Code: {}", self.licensee_code);
        println!("Supports SGB: {}", self.supports_sgb);
        println!("Cartridge Type: {:?}", self.cartridge_type);
        println!("ROM Size: {} bytes", self.rom_size);
        println!("ROM Number of Banks: {}", self.rom_number_banks);
        println!("RAM Size: {} bytes", self.ram_size);
        println!("Destination Code: {:?}", self.destination_code);
        println!(
            "Mask ROM Version Number: 0x{:04X}",
            self.mask_rom_version_number
        );
        println!("Is Nintendo logo: {}", self.is_nintendo_logo);
        println!("Header Checksum Valid: {}", self.is_header_checksum_valid);
        println!();
    }
}
