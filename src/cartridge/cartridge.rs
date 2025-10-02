use std::fs;

use crate::mmu::address::{ADDRESS, Address, AddressRange, Size};
use crate::mmu::mmu::{Byte, DEFAULT_BYTE};

pub struct Cartridge {
    rom: [Byte; ADDRESS::ROM.size],
    ram: [Byte; ADDRESS::RAM.size],
    entry_point: Address,
    title: String,
    manufacturer_code: Byte,
    licence_code: LicenceCode,
    supports_sgb: bool,
    cartridge_type: CartridgeType,
    rom_size: usize,
    ram_size: usize,
    destination_code: DestinationCode,
    old_licence_code: OldLicenceCode,
    mask_rom_version_number: Byte,
    is_nintendo_logo: bool,
    is_header_checksum_valid: bool,
}

#[derive(Debug)]
enum LicenceCode {
    None,
    NintendoRAndD1,
    Capcom,
    EA,
    HudsonSoft,
    BAI,
    KSS,
    PlanningOfficeWADA,
    PCMComplete,
    SanX,
    Kemco,
    SETACorporation,
    Viacom,
    Nintendo,
    Bandai,
    OceanAcclaim,
    Konami,
    HectorSoft,
    Taito,
    Banpresto,
    UbiSoft,
    Atlus,
    MalibuInteractive,
    Angel,
    BulletProofSoftware,
    Irem,
    Absolute,
    Acclaim,
    Activision,
    SammyUSA,
    HiTechExpressions,
    LJN,
    Matchbox,
    Mattel,
    MiltonBradley,
    Titus,
    VirginGames,
    LucasfilmGames,
    Broderbund,
    SculpturedSoftware,
    TheSalesCurve,
    THQ,
    Accolade,
    MisawaEntertainment,
    Lozc,
    TokumaShoten,
    TsukudaOriginal,
    Chunsoft,
    VideoSystem,
    Varie,
    YonezawaSpal,
    Kaneko,
    PackInVideo,
    BottomUp,
    KonamiYuGiOh,
    MTO,
    Kodansha,
    Unknown(String),
}

#[derive(Debug)]
enum CartridgeType {
    ROMOnly,
    MBC1,
    MBC1RAM,
    MBC1RAMBattery,
    MBC2,
    MBC2Battery,
    ROMRAM,
    ROMRAMBattery,
    MMM01,
    MMM01RAM,
    MMM01RAMBattery,
    MBC3TimerBattery,
    MBC3TimerRAMBattery,
    MBC3,
    MBC3RAM,
    MBC3RAMBattery,
    MBC5,
    MBC5RAM,
    MBC5RAMBattery,
    MBC5Rumble,
    MBC5RumbleRAM,
    MBC5RumbleRAMBattery,
    PocketCamera,
    BandaiTAMA5,
    HuC3,
    HuC1RAMBattery,
    Unknown(Byte),
}

#[derive(Debug)]
enum DestinationCode {
    Japan,
    Overseas,
    Unknown(Byte),
}

#[derive(Debug)]
enum OldLicenceCode {
    None,
    Nintendo,
    Capcom,
    HotB,
    Jaleco,
    CoconutsJapan,
    EliteSystems,
    EA,
    HudsonSoft,
    ITCEntertainment,
    Yanoman,
    JapanClary,
    VirginGames,
    PCMComplete,
    SanX,
    Kemco,
    SETA,
    Infogrames,
    Bandai,
    NewLicenseeCode,
    Konami,
    HectorSoft,
    Banpresto,
    EntertainmentInteractive,
    Gremlin,
    UbiSoft,
    Atlus,
    MalibuInteractive,
    Angel,
    SpectrumHoloByte,
    Irem,
    USGold,
    Absolute,
    Acclaim,
    Activision,
    SammyUSA,
    GameTek,
    ParkPlace,
    LJN,
    Matchbox,
    MiltonBradley,
    Mindscape,
    Romstar,
    NaxatSoft,
    Tradewest,
    Titus,
    OceanSoftware,
    EA2,
    EliteSystems2,
    ElectroBrain,
    Infogrames2,
    Interplay,
    Broderbund,
    SculpturedSoftware,
    TheSalesCurve,
    THQ,
    Accolade,
    TriffixEntertainment,
    MicroProse,
    Kemco2,
    MisawaEntertainment,
    LOZC,
    TokumaShoten,
    BulletProofSoftware,
    VicTokai,
    ApeInc,
    IMax,
    Chunsoft,
    VideoSystem,
    TsubarayaProductions,
    Varie,
    YonezawaSpal,
    Kemco3,
    Arc,
    NihonBussan,
    Tecmo,
    Imagineer,
    Banpresto2,
    Nova,
    HoriElectric,
    Bandai2,
    Konami2,
    Kawada,
    Takara,
    TechnosJapan,
    Broderbund2,
    ToeiAnimation,
    Toho,
    Namco,
    Acclaim2,
    ASCIIorNexsoft,
    Bandai3,
    SquareEnix,
    HALLaboratory,
    SNK,
    PonyCanyon,
    CultureBrain,
    Sunsoft,
    SonyImagesoft,
    SammyCorporation,
    Taito,
    Kemco4,
    Square,
    TokumaShoten2,
    DataEast,
    TonkinHouse,
    Koei,
    UFL,
    UltraGames,
    VAP,
    UseCorporation,
    Meldac,
    PonyCanyon2,
    Angel2,
    Taito2,
    SOFEL,
    Quest,
    SigmaEnterprises,
    ASKKodansha,
    NaxatSoft2,
    CopyaSystem,
    Banpresto3,
    Tomy,
    LJN2,
    NipponComputerSystems,
    HumanEnt,
    Altron,
    Jaleco2,
    TowaChiki,
    Yutaka,
    Varie2,
    Epoch,
    Athena,
    AsmikAce,
    Natsume,
    KingRecords,
    Atlus2,
    EpicSonyRecords,
    IGS,
    AWave,
    ExtremeEntertainment,
    LJN3,
    Unknown(Byte),
}

impl Cartridge {
    pub fn insert(path: &str) -> Self {
        let bytes_result: Result<Vec<Byte>, &'static str> = Cartridge::read(path);
        match bytes_result {
            Ok(bytes) => {
                let rom: [Byte; ADDRESS::ROM.size] = Cartridge::extract_rom(&bytes);
                let ram: [Byte; ADDRESS::RAM.size] = Cartridge::extract_ram(&bytes);
                let entry_point: Address = Cartridge::extract_entry_point();
                let title: String = Cartridge::extract_title(&bytes);
                let manufacturer_code: Byte = Cartridge::extract_manufacturer_code(&bytes);
                let licence_code: LicenceCode = Cartridge::extract_licence_code(&bytes);
                let supports_sgb: bool = Cartridge::extract_sgb_flag(&bytes);
                let cartridge_type: CartridgeType = Cartridge::extract_cartridge_type(&bytes);
                let rom_size: Size = Cartridge::extract_rom_size(&bytes);
                let ram_size: Size = Cartridge::extract_ram_size(&bytes);
                let destination_code: DestinationCode = Cartridge::extract_destination_code(&bytes);
                let old_licence_code: OldLicenceCode = Cartridge::extract_old_licence_code(&bytes);
                let mask_rom_version_number: Byte =
                    Cartridge::extract_mask_rom_version_number(&bytes);
                let is_nintendo_logo = Cartridge::is_nintendo_logo(&bytes);
                let is_header_checksum_valid: bool = Cartridge::is_header_checksum_valid(&bytes);
                Cartridge {
                    rom,
                    ram,
                    entry_point,
                    title,
                    manufacturer_code,
                    licence_code,
                    supports_sgb,
                    cartridge_type,
                    rom_size,
                    ram_size,
                    destination_code,
                    old_licence_code,
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

    fn is_nintendo_logo(bytes: &[Byte]) -> bool {
        let mut nintendo_logo_cartridge: [Byte; ADDRESS::NINTENDO_LOGO.size] =
            [DEFAULT_BYTE; ADDRESS::NINTENDO_LOGO.size];
        for address in ADDRESS::NINTENDO_LOGO.start..=ADDRESS::NINTENDO_LOGO.end {
            nintendo_logo_cartridge[address - ADDRESS::NINTENDO_LOGO.start] = bytes[address];
        }

        let valid_logo: [Byte; ADDRESS::NINTENDO_LOGO.size] = [
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];

        nintendo_logo_cartridge == valid_logo
    }

    fn extract_rom(bytes: &[Byte]) -> [Byte; ADDRESS::ROM.size] {
        let mut rom: [Byte; ADDRESS::ROM.size] = [DEFAULT_BYTE; ADDRESS::ROM.size];
        for address in ADDRESS::ROM.start..=ADDRESS::ROM.end {
            rom[address - ADDRESS::ROM.start] = bytes[address];
        }
        rom
    }

    fn extract_ram(bytes: &[Byte]) -> [Byte; ADDRESS::RAM.size] {
        let mut ram: [Byte; ADDRESS::RAM.size] = [DEFAULT_BYTE; ADDRESS::RAM.size];
        for address in ADDRESS::RAM.start..=ADDRESS::RAM.end {
            ram[address - ADDRESS::RAM.start] = bytes[address];
        }
        ram
    }

    fn extract_entry_point() -> Address {
        // const ENTRY_POINT: Address = 0x100;
        // bytes[ENTRY_POINT]
        0x100
    }

    fn extract_title(bytes: &[Byte]) -> String {
        const TITLE_ADDRESS_START: Address = 0x0134;
        const TITLE_ADDRESS_END: Address = 0x0143;

        let title_hex: &[Byte] = &bytes[TITLE_ADDRESS_START..TITLE_ADDRESS_END + 1];
        String::from_utf8_lossy(title_hex)
            .trim_end_matches('\0')
            .to_string()
    }

    fn extract_manufacturer_code(bytes: &[Byte]) -> Byte {
        const MANUFACTURER_CODE_START: Address = 0x013F;
        const MANUFACTURER_CODE_END: Address = 0x0142;

        let manufacturer_code_hex: &[u8] =
            &bytes[MANUFACTURER_CODE_START..MANUFACTURER_CODE_END + 1];

        manufacturer_code_hex[0]
    }

    fn extract_licence_code(bytes: &[Byte]) -> LicenceCode {
        const LICENCE_CODE_START: Address = 0x0144;
        const LICENCE_CODE_END: Address = 0x0145;

        let licence_code_hex: &[u8] = &bytes[LICENCE_CODE_START..LICENCE_CODE_END + 1];
        match licence_code_hex {
            [0x00, 0x00] => LicenceCode::None,
            [0x01, 0x00] => LicenceCode::NintendoRAndD1,
            [0x08, 0x00] => LicenceCode::Capcom,
            [0x13, 0x00] => LicenceCode::EA,
            [0x18, 0x00] => LicenceCode::HudsonSoft,
            [0x19, 0x00] => LicenceCode::BAI,
            [0x20, 0x00] => LicenceCode::KSS,
            [0x22, 0x00] => LicenceCode::PlanningOfficeWADA,
            [0x24, 0x00] => LicenceCode::PCMComplete,
            [0x25, 0x00] => LicenceCode::SanX,
            [0x28, 0x00] => LicenceCode::Kemco,
            [0x29, 0x00] => LicenceCode::SETACorporation,
            [0x30, 0x00] => LicenceCode::Viacom,
            [0x31, 0x00] => LicenceCode::Nintendo,
            [0x32, 0x00] => LicenceCode::Bandai,
            [0x33, 0x00] => LicenceCode::OceanAcclaim,
            [0x34, 0x00] => LicenceCode::Konami,
            [0x35, 0x00] => LicenceCode::HectorSoft,
            [0x38, 0x00] => LicenceCode::Taito,
            [0x39, 0x00] => LicenceCode::Banpresto,
            [0x41, 0x00] => LicenceCode::UbiSoft,
            [0x42, 0x00] => LicenceCode::Atlus,
            [0x44, 0x00] => LicenceCode::MalibuInteractive,
            [0x46, 0x00] => LicenceCode::Angel,
            [0x47, 0x00] => LicenceCode::BulletProofSoftware,
            [0x49, 0x00] => LicenceCode::Irem,
            [0x50, 0x00] => LicenceCode::Absolute,
            [0x51, 0x00] => LicenceCode::Acclaim,
            [0x52, 0x00] => LicenceCode::Activision,
            [0x53, 0x00] => LicenceCode::SammyUSA,
            [0x54, 0x00] => LicenceCode::HiTechExpressions,
            [0x55, 0x00] => LicenceCode::LJN,
            [0x56, 0x00] => LicenceCode::Matchbox,
            [0x57, 0x00] => LicenceCode::Mattel,
            [0x58, 0x00] => LicenceCode::MiltonBradley,
            [0x59, 0x00] => LicenceCode::Titus,
            [0x60, 0x00] => LicenceCode::VirginGames,
            [0x61, 0x00] => LicenceCode::LucasfilmGames,
            [0x64, 0x00] => LicenceCode::Broderbund,
            [0x67, 0x00] => LicenceCode::SculpturedSoftware,
            [0x69, 0x00] => LicenceCode::TheSalesCurve,
            [0x70, 0x00] => LicenceCode::THQ,
            [0x71, 0x00] => LicenceCode::Accolade,
            [0x72, 0x00] => LicenceCode::MisawaEntertainment,
            [0x73, 0x00] => LicenceCode::Lozc,
            [0x75, 0x00] => LicenceCode::TokumaShoten,
            [0x78, 0x00] => LicenceCode::TsukudaOriginal,
            [0x79, 0x00] => LicenceCode::Chunsoft,
            [0x80, 0x00] => LicenceCode::VideoSystem,
            [0x83, 0x00] => LicenceCode::Varie,
            [0x86, 0x00] => LicenceCode::YonezawaSpal,
            [0x87, 0x00] => LicenceCode::Kaneko,
            [0x91, 0x00] => LicenceCode::PackInVideo,
            [0x92, 0x00] => LicenceCode::BottomUp,
            [0x93, 0x00] => LicenceCode::KonamiYuGiOh,
            [0x95, 0x00] => LicenceCode::MTO,
            [0x96, 0x00] => LicenceCode::Kodansha,
            _ => LicenceCode::Unknown(format!(
                "{:02X}{:02X}",
                licence_code_hex[0], licence_code_hex[1]
            )),
        }
    }

    fn extract_sgb_flag(bytes: &[Byte]) -> bool {
        const SGB_FLAG_ADDRESS: Address = 0x0146;
        if bytes[SGB_FLAG_ADDRESS] == 0x03 {
            return true;
        }
        false
    }

    fn extract_cartridge_type(bytes: &[Byte]) -> CartridgeType {
        const CARTRIDGE_TYPE_ADDRESS: usize = 0x0147;
        match bytes[CARTRIDGE_TYPE_ADDRESS] {
            0x00 => CartridgeType::ROMOnly,
            0x01 => CartridgeType::MBC1,
            0x02 => CartridgeType::MBC1RAM,
            0x03 => CartridgeType::MBC1RAMBattery,
            0x05 => CartridgeType::MBC2,
            0x06 => CartridgeType::MBC2Battery,
            0x08 => CartridgeType::ROMRAM,
            0x09 => CartridgeType::ROMRAMBattery,
            0x0B => CartridgeType::MMM01,
            0x0C => CartridgeType::MMM01RAM,
            0x0D => CartridgeType::MMM01RAMBattery,
            0x0F => CartridgeType::MBC3TimerBattery,
            0x10 => CartridgeType::MBC3TimerRAMBattery,
            0x11 => CartridgeType::MBC3,
            0x12 => CartridgeType::MBC3RAM,
            0x13 => CartridgeType::MBC3RAMBattery,
            0x19 => CartridgeType::MBC5,
            0x1A => CartridgeType::MBC5RAM,
            0x1B => CartridgeType::MBC5RAMBattery,
            0x1C => CartridgeType::MBC5Rumble,
            0x1D => CartridgeType::MBC5RumbleRAM,
            0x1E => CartridgeType::MBC5RumbleRAMBattery,
            0x1F => CartridgeType::PocketCamera,
            0xFD => CartridgeType::BandaiTAMA5,
            0xFE => CartridgeType::HuC3,
            0xFF => CartridgeType::HuC1RAMBattery,
            other => CartridgeType::Unknown(other),
        }
    }

    fn extract_rom_size(bytes: &[Byte]) -> usize {
        const ROM_SIZE_ADDRESS: Address = 0x0148;
        match bytes[ROM_SIZE_ADDRESS] {
            0x00 => 32 * 1024,
            0x01 => 64 * 1024,
            0x02 => 128 * 1024,
            0x03 => 256 * 1024,
            0x04 => 512 * 1024,
            0x05 => 1 * 1024 * 1024,
            0x06 => 2 * 1024 * 1024,
            0x07 => 4 * 1024 * 1024,
            0x08 => 8 * 1024 * 1024,
            0x52 => 1152 * 1024,
            0x53 => 1280 * 1024,
            0x54 => 1536 * 1024,
            _ => 0,
        }
    }

    fn extract_ram_size(bytes: &[Byte]) -> usize {
        const RAM_SIZE_ADDRESS: Address = 0x0149;
        match bytes[RAM_SIZE_ADDRESS] {
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
        const DESTINATION_CODE_ADDRESS: Address = 0x014A;
        match bytes[DESTINATION_CODE_ADDRESS] {
            0x00 => DestinationCode::Japan,
            0x01 => DestinationCode::Overseas,
            other => DestinationCode::Unknown(other),
        }
    }

    fn extract_old_licence_code(bytes: &[Byte]) -> OldLicenceCode {
        const OLD_LICENCE_CODE: Address = 0x014B;
        match bytes[OLD_LICENCE_CODE] {
            0x00 => OldLicenceCode::None,
            0x01 => OldLicenceCode::Nintendo,
            0x08 => OldLicenceCode::Capcom,
            0x09 => OldLicenceCode::HotB,
            0x0A => OldLicenceCode::Jaleco,
            0x0B => OldLicenceCode::CoconutsJapan,
            0x0C => OldLicenceCode::EliteSystems,
            0x13 => OldLicenceCode::EA,
            0x18 => OldLicenceCode::HudsonSoft,
            0x19 => OldLicenceCode::ITCEntertainment,
            0x1A => OldLicenceCode::Yanoman,
            0x1D => OldLicenceCode::JapanClary,
            0x1F => OldLicenceCode::VirginGames,
            0x24 => OldLicenceCode::PCMComplete,
            0x25 => OldLicenceCode::SanX,
            0x28 => OldLicenceCode::Kemco,
            0x29 => OldLicenceCode::SETA,
            0x30 => OldLicenceCode::Infogrames,
            0x31 => OldLicenceCode::Nintendo,
            0x32 => OldLicenceCode::Bandai,
            0x33 => OldLicenceCode::NewLicenseeCode,
            0x34 => OldLicenceCode::Konami,
            0x35 => OldLicenceCode::HectorSoft,
            0x38 => OldLicenceCode::Capcom,
            0x39 => OldLicenceCode::Banpresto,
            0x3C => OldLicenceCode::EntertainmentInteractive,
            0x3E => OldLicenceCode::Gremlin,
            0x41 => OldLicenceCode::UbiSoft,
            0x42 => OldLicenceCode::Atlus,
            0x44 => OldLicenceCode::MalibuInteractive,
            0x46 => OldLicenceCode::Angel,
            0x47 => OldLicenceCode::SpectrumHoloByte,
            0x49 => OldLicenceCode::Irem,
            0x4A => OldLicenceCode::VirginGames,
            0x4D => OldLicenceCode::MalibuInteractive,
            0x4F => OldLicenceCode::USGold,
            0x50 => OldLicenceCode::Absolute,
            0x51 => OldLicenceCode::Acclaim,
            0x52 => OldLicenceCode::Activision,
            0x53 => OldLicenceCode::SammyUSA,
            0x54 => OldLicenceCode::GameTek,
            0x55 => OldLicenceCode::ParkPlace,
            0x56 => OldLicenceCode::LJN,
            0x57 => OldLicenceCode::Matchbox,
            0x59 => OldLicenceCode::MiltonBradley,
            0x5A => OldLicenceCode::Mindscape,
            0x5B => OldLicenceCode::Romstar,
            0x5C => OldLicenceCode::NaxatSoft,
            0x5D => OldLicenceCode::Tradewest,
            0x60 => OldLicenceCode::Titus,
            0x61 => OldLicenceCode::VirginGames,
            0x67 => OldLicenceCode::OceanSoftware,
            0x69 => OldLicenceCode::EA2,
            0x6E => OldLicenceCode::EliteSystems2,
            0x6F => OldLicenceCode::ElectroBrain,
            0x70 => OldLicenceCode::Infogrames2,
            0x71 => OldLicenceCode::Interplay,
            0x72 => OldLicenceCode::Broderbund,
            0x73 => OldLicenceCode::SculpturedSoftware,
            0x75 => OldLicenceCode::TheSalesCurve,
            0x78 => OldLicenceCode::THQ,
            0x79 => OldLicenceCode::Accolade,
            0x7A => OldLicenceCode::TriffixEntertainment,
            0x7C => OldLicenceCode::MicroProse,
            0x7F => OldLicenceCode::Kemco2,
            0x80 => OldLicenceCode::MisawaEntertainment,
            0x83 => OldLicenceCode::LOZC,
            0x86 => OldLicenceCode::TokumaShoten,
            0x8B => OldLicenceCode::BulletProofSoftware,
            0x8C => OldLicenceCode::VicTokai,
            0x8E => OldLicenceCode::ApeInc,
            0x8F => OldLicenceCode::IMax,
            0x91 => OldLicenceCode::Chunsoft,
            0x92 => OldLicenceCode::VideoSystem,
            0x93 => OldLicenceCode::TsubarayaProductions,
            0x95 => OldLicenceCode::Varie,
            0x96 => OldLicenceCode::YonezawaSpal,
            0x97 => OldLicenceCode::Kemco3,
            0x99 => OldLicenceCode::Arc,
            0x9A => OldLicenceCode::NihonBussan,
            0x9B => OldLicenceCode::Tecmo,
            0x9C => OldLicenceCode::Imagineer,
            0x9D => OldLicenceCode::Banpresto2,
            0x9F => OldLicenceCode::Nova,
            0xA1 => OldLicenceCode::HoriElectric,
            0xA2 => OldLicenceCode::Bandai2,
            0xA4 => OldLicenceCode::Konami2,
            0xA6 => OldLicenceCode::Kawada,
            0xA7 => OldLicenceCode::Takara,
            0xA9 => OldLicenceCode::TechnosJapan,
            0xAA => OldLicenceCode::Broderbund2,
            0xAC => OldLicenceCode::ToeiAnimation,
            0xAD => OldLicenceCode::Toho,
            0xAF => OldLicenceCode::Namco,
            0xB0 => OldLicenceCode::Acclaim2,
            0xB1 => OldLicenceCode::ASCIIorNexsoft,
            0xB2 => OldLicenceCode::Bandai3,
            0xB4 => OldLicenceCode::SquareEnix,
            0xB6 => OldLicenceCode::HALLaboratory,
            0xB7 => OldLicenceCode::SNK,
            0xB9 => OldLicenceCode::PonyCanyon,
            0xBA => OldLicenceCode::CultureBrain,
            0xBB => OldLicenceCode::Sunsoft,
            0xBD => OldLicenceCode::SonyImagesoft,
            0xBF => OldLicenceCode::SammyCorporation,
            0xC0 => OldLicenceCode::Taito,
            0xC2 => OldLicenceCode::Kemco4,
            0xC3 => OldLicenceCode::Square,
            0xC4 => OldLicenceCode::TokumaShoten2,
            0xC5 => OldLicenceCode::DataEast,
            0xC6 => OldLicenceCode::TonkinHouse,
            0xC8 => OldLicenceCode::Koei,
            0xC9 => OldLicenceCode::UFL,
            0xCA => OldLicenceCode::UltraGames,
            0xCB => OldLicenceCode::VAP,
            0xCC => OldLicenceCode::UseCorporation,
            0xCD => OldLicenceCode::Meldac,
            0xCE => OldLicenceCode::PonyCanyon2,
            0xCF => OldLicenceCode::Angel2,
            0xD0 => OldLicenceCode::Taito2,
            0xD1 => OldLicenceCode::SOFEL,
            0xD2 => OldLicenceCode::Quest,
            0xD3 => OldLicenceCode::SigmaEnterprises,
            0xD4 => OldLicenceCode::ASKKodansha,
            0xD6 => OldLicenceCode::NaxatSoft2,
            0xD7 => OldLicenceCode::CopyaSystem,
            0xD9 => OldLicenceCode::Banpresto3,
            0xDA => OldLicenceCode::Tomy,
            0xDB => OldLicenceCode::LJN2,
            0xDD => OldLicenceCode::NipponComputerSystems,
            0xDE => OldLicenceCode::HumanEnt,
            0xDF => OldLicenceCode::Altron,
            0xE0 => OldLicenceCode::Jaleco2,
            0xE1 => OldLicenceCode::TowaChiki,
            0xE2 => OldLicenceCode::Yutaka,
            0xE3 => OldLicenceCode::Varie2,
            0xE5 => OldLicenceCode::Epoch,
            0xE7 => OldLicenceCode::Athena,
            0xE8 => OldLicenceCode::AsmikAce,
            0xE9 => OldLicenceCode::Natsume,
            0xEA => OldLicenceCode::KingRecords,
            0xEB => OldLicenceCode::Atlus2,
            0xEC => OldLicenceCode::EpicSonyRecords,
            0xEE => OldLicenceCode::IGS,
            0xF0 => OldLicenceCode::AWave,
            0xF3 => OldLicenceCode::ExtremeEntertainment,
            0xFF => OldLicenceCode::LJN3,
            other => OldLicenceCode::Unknown(other),
        }
    }

    fn extract_mask_rom_version_number(bytes: &[Byte]) -> u8 {
        const MASK_ROM_VERSION_NUMBER: Address = 0x014C;
        bytes[MASK_ROM_VERSION_NUMBER]
    }

    fn is_header_checksum_valid(bytes: &[Byte]) -> bool {
        const HEADER_CHECKSUM: Address = 0x014D;
        const HEADER_CHECKSUM_START: Address = 0x0134;
        const HEADER_CHECKSUM_END: Address = 0x014C;

        let header_checksum: Byte = bytes[HEADER_CHECKSUM];
        let mut calculated_checksum: u8 = 0;

        for &byte in &bytes[HEADER_CHECKSUM_START..=HEADER_CHECKSUM_END] {
            calculated_checksum = calculated_checksum.wrapping_sub(byte).wrapping_sub(1);
        }

        header_checksum == calculated_checksum
    }

    pub fn eject() -> Self {
        Cartridge {
            rom: [DEFAULT_BYTE; ADDRESS::ROM.size],
            ram: [DEFAULT_BYTE; ADDRESS::RAM.size],
            entry_point: 0,
            title: "none".to_string(),
            manufacturer_code: 0,
            licence_code: LicenceCode::None,
            supports_sgb: false,
            cartridge_type: CartridgeType::ROMOnly,
            rom_size: 0,
            ram_size: 0,
            destination_code: DestinationCode::Japan,
            old_licence_code: OldLicenceCode::None,
            mask_rom_version_number: 0,
            is_nintendo_logo: false,
            is_header_checksum_valid: false,
        }
    }

    pub fn get_entry_point(&self) -> Address {
        self.entry_point
    }

    pub fn get_rom(&self) -> [Byte; ADDRESS::ROM.size] {
        self.rom
    }

    pub fn read_rom(&self, address: Address) -> Byte {
        self.rom[address - ADDRESS::ROM.start]
    }

    pub fn get_ram(&self) -> [Byte; ADDRESS::RAM.size] {
        self.ram
    }

    pub fn read_ram(&self, address: Address) -> Byte {
        let shift: Address = address - ADDRESS::RAM.start;
        self.ram[shift]
    }

    pub fn write_ram(&mut self, address: Address, value: Byte) {
        let shift: Address = address - ADDRESS::RAM.start;
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

    pub fn print_data(&self) {
        println!("Entry Point: 0x{:04X}", self.entry_point);
        println!("Title: {}", self.title);
        println!("Manufacturer Code: 0x{:04X}", self.manufacturer_code);
        println!("Licence Code: {:?}", self.licence_code);
        println!("Supports SGB: {}", self.supports_sgb);
        println!("Cartridge Type: {:?}", self.cartridge_type);
        println!("ROM Size: {} bytes", self.rom_size);
        println!("RAM Size: {} bytes", self.ram_size);
        println!("Destination Code: {:?}", self.destination_code);
        println!("Old Licence Code: {:?}", self.old_licence_code);
        println!(
            "Mask ROM Version Number: 0x{:04X}",
            self.mask_rom_version_number
        );
        println!("Is Nintendo logo: {}", self.is_nintendo_logo);
        println!("Header Checksum Valid: {}", self.is_header_checksum_valid);
        println!();
    }
}
