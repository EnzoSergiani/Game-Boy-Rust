use crate::{
    common::{
        address::{
            ECHO, HRAM, IE_REGISTER, INVALID_OAM, IO, NINTENDO_LOGO, OAM, RAM, ROM, TILE_MAP,
            TILE_SET, VRAM, WRAM,
        },
        constant::DEFAULT_BYTE,
        types::{Address, Byte},
    },
    cpu::{
        cpu::{CPU, IME},
        registers::{Flags, Register8, Register16, Registers8, Registers16},
    },
};

#[allow(non_camel_case_types)]
type n8 = i8;
#[allow(non_camel_case_types)]
type n16 = i16;
#[allow(non_camel_case_types)]
type e8 = i8;
#[allow(non_camel_case_types)]
type u3 = u8;

#[allow(non_camel_case_types)]
pub enum Instructions {
    NONE,
    PREFIX,
    // Load
    LD_r8_r8(Registers8, Registers8),
    LD_r8_n8(Registers8),
    LD_r16_r16(Registers16, Registers16),
    LD_r16_n16(Registers16),
    LD_HL_r8(Registers8),
    LD_HL_n8,
    LD_r8_HL(Registers8),
    LD_r16_A(Registers16),
    LD_n16_A,
    LDH_n16_A,
    LDH_C_A,
    LD_A_r16(Registers16),
    LD_A_n16,
    LDH_A_n16,
    LDH_A_C,
    LD_HLI_A,
    LD_HLD_A,
    LD_A_HLI,
    LD_A_HLD,
    // 8-bit arithmetic
    ADC_A_r8(Registers8),
    ADC_A_HL,
    ADC_A_n8,
    ADD_A_r8(Registers8),
    ADD_A_HL,
    ADD_A_n8,
    CP_A_r8(Registers8),
    CP_A_HL,
    CP_A_n8,
    DEC_r8(Registers8),
    DEC_HL,
    INC_r8(Registers8),
    INC_HL,
    SBC_A_r8(Registers8),
    SBC_A_HL,
    SBC_A_n8,
    SUB_A_r8(Registers8),
    SUB_A_HL,
    SUB_A_n8,
    // 16-bit arithmetic
    ADD_HL_r16(Registers16),
    DEC_r16(Registers16),
    INC_r16(Registers16),
    // Bitwise logic
    AND_A_r8(Registers8),
    AND_A_HL,
    AND_A_n8,
    CPL,
    OR_A_r8(Registers8),
    OR_A_HL,
    OR_A_n8,
    XOR_A_r8(Registers8),
    XOR_A_HL,
    XOR_A_n8,
    // Bit flag
    BIT_u3_r8(u3, Registers8),
    BIT_u3_HL(u3),
    RES_u3_r8(u3, Registers8),
    RES_u3_HL(u3),
    SET_u3_r8(u3, Registers8),
    SET_u3_HL(u3),
    // Bit shift
    RL_r8(Registers8),
    RL_HL,
    RLA,
    RLC_r8(Registers8),
    RLC_HL,
    RLCA,
    RR_r8(Registers8),
    RR_HL,
    RRA,
    RRC_r8(Registers8),
    RRC_HL,
    RRCA,
    SLA_r8(Registers8),
    SLA_HL,
    SRA_r8(Registers8),
    SRA_HL,
    SRL_r8(Registers8),
    SRL_HL,
    SWAP_r8(Registers8),
    SWAP_HL,
    // Jump and subroutine
    CALL_n16,
    CALL_cc_n16,
    JP_HL,
    JP_n16,
    JP_cc_n16,
    JR_n16,
    JR_cc_n16,
    RET_cc,
    RET,
    RETI,
    RST,
    // Carry flag
    CCF,
    SCF,
    // Stack manipulation
    ADD_HL_SP,
    ADD_SP_e8,
    DEC_SP,
    INC_SP,
    LD_SP_n16,
    LD_n16_SP,
    LD_HL_SP_e8,
    LD_SP_HL,
    POP_AF,
    POP_r16(Registers16),
    PUSH_AF,
    PUSH_r16(Registers16),
    // Interrupt-related
    DI,
    EI,
    HALT,
    // Miscellaneous
    DAA,
    NOP,
    STOP,
}

impl Instructions {
    pub fn get_instruction(opcode: Byte) -> Instructions {
        match opcode {
            0x00 => Instructions::NOP,
            0x01 => Instructions::LD_r8_r8(Registers8::B, Registers8::C),
            0x02 => Instructions::LD_r16_A(Registers16::BC),
            0x03 => Instructions::INC_r16(Registers16::BC),
            0x04 => Instructions::INC_r8(Registers8::B),
            0x05 => Instructions::DEC_r8(Registers8::B),
            0x06 => Instructions::LD_r8_n8(Registers8::B),
            0x07 => Instructions::RLCA,
            0x08 => Instructions::LD_n16_SP,
            0x09 => Instructions::ADD_HL_r16(Registers16::BC),
            0x0A => Instructions::LD_A_r16(Registers16::BC),
            0x0B => Instructions::DEC_r16(Registers16::BC),
            0x0C => Instructions::INC_r8(Registers8::C),
            0x0D => Instructions::DEC_r8(Registers8::C),
            0x0E => Instructions::LD_r8_n8(Registers8::C),
            0x0F => Instructions::RRCA,
            0x10 => Instructions::STOP,
            0x11 => Instructions::LD_r16_n16(Registers16::DE),
            0x12 => Instructions::LD_r16_A(Registers16::DE),
            0x13 => Instructions::INC_r16(Registers16::DE),
            0x14 => Instructions::INC_r8(Registers8::D),
            0x15 => Instructions::DEC_r8(Registers8::D),
            0x16 => Instructions::LD_r8_n8(Registers8::D),
            0x17 => Instructions::RLA,
            0x18 => Instructions::JR_n16,
            0x19 => Instructions::ADD_HL_r16(Registers16::DE),
            0x1A => Instructions::LD_A_r16(Registers16::DE),
            0x1B => Instructions::DEC_r16(Registers16::DE),
            0x1C => Instructions::INC_r8(Registers8::E),
            0x1D => Instructions::DEC_r8(Registers8::E),
            0x1E => Instructions::LD_r8_n8(Registers8::E),
            0x1F => Instructions::RRA,
            0x20 => Instructions::JR_cc_n16,
            0x21 => Instructions::LD_r16_n16(Registers16::HL),
            0x22 => Instructions::LD_HLI_A,
            0x23 => Instructions::INC_r16(Registers16::HL),
            0x24 => Instructions::INC_r8(Registers8::H),
            0x25 => Instructions::DEC_r8(Registers8::H),
            0x26 => Instructions::LD_r8_n8(Registers8::H),
            0x27 => Instructions::DAA,
            0x28 => Instructions::JR_cc_n16,
            0x29 => Instructions::ADD_HL_r16(Registers16::HL),
            0x2A => Instructions::LD_A_HLI,
            0x2B => Instructions::DEC_r16(Registers16::HL),
            0x2C => Instructions::INC_r8(Registers8::L),
            0x2D => Instructions::DEC_r8(Registers8::L),
            0x2E => Instructions::LD_r8_n8(Registers8::L),
            0x2F => Instructions::CPL,
            0x30 => Instructions::JR_cc_n16,
            0x31 => Instructions::LD_r16_n16(Registers16::SP),
            0x32 => Instructions::LD_HLD_A,
            0x33 => Instructions::INC_r16(Registers16::SP),
            0x34 => Instructions::INC_HL,
            0x35 => Instructions::DEC_HL,
            0x36 => Instructions::LD_HL_n8,
            0x37 => Instructions::SCF,
            0x38 => Instructions::JR_cc_n16,
            0x39 => Instructions::ADD_HL_r16(Registers16::SP),
            0x3A => Instructions::LD_A_HLD,
            0x3B => Instructions::DEC_r16(Registers16::SP),
            0x3C => Instructions::INC_r8(Registers8::A),
            0x3D => Instructions::DEC_r8(Registers8::A),
            0x3E => Instructions::LD_r8_n8(Registers8::A),
            0x3F => Instructions::CCF,
            0x40 => Instructions::LD_r8_r8(Registers8::B, Registers8::B),
            0x41 => Instructions::LD_r8_r8(Registers8::B, Registers8::C),
            0x42 => Instructions::LD_r8_r8(Registers8::B, Registers8::D),
            0x43 => Instructions::LD_r8_r8(Registers8::B, Registers8::E),
            0x44 => Instructions::LD_r8_r8(Registers8::B, Registers8::H),
            0x45 => Instructions::LD_r8_r8(Registers8::B, Registers8::L),
            0x46 => Instructions::LD_r8_HL(Registers8::B),
            0x47 => Instructions::LD_r8_r8(Registers8::B, Registers8::A),
            0x48 => Instructions::LD_r8_r8(Registers8::C, Registers8::B),
            0x49 => Instructions::LD_r8_r8(Registers8::C, Registers8::C),
            0x4A => Instructions::LD_r8_r8(Registers8::C, Registers8::D),
            0x4B => Instructions::LD_r8_r8(Registers8::C, Registers8::E),
            0x4C => Instructions::LD_r8_r8(Registers8::C, Registers8::H),
            0x4D => Instructions::LD_r8_r8(Registers8::C, Registers8::L),
            0x4E => Instructions::LD_r8_HL(Registers8::C),
            0x4F => Instructions::LD_r8_r8(Registers8::C, Registers8::A),
            0x50 => Instructions::LD_r8_r8(Registers8::D, Registers8::B),
            0x51 => Instructions::LD_r8_r8(Registers8::D, Registers8::C),
            0x52 => Instructions::LD_r8_r8(Registers8::D, Registers8::D),
            0x53 => Instructions::LD_r8_r8(Registers8::D, Registers8::E),
            0x54 => Instructions::LD_r8_r8(Registers8::D, Registers8::H),
            0x55 => Instructions::LD_r8_r8(Registers8::D, Registers8::L),
            0x56 => Instructions::LD_r8_HL(Registers8::D),
            0x57 => Instructions::LD_r8_r8(Registers8::D, Registers8::A),
            0x58 => Instructions::LD_r8_r8(Registers8::E, Registers8::B),
            0x59 => Instructions::LD_r8_r8(Registers8::E, Registers8::C),
            0x5A => Instructions::LD_r8_r8(Registers8::E, Registers8::D),
            0x5B => Instructions::LD_r8_r8(Registers8::E, Registers8::E),
            0x5C => Instructions::LD_r8_r8(Registers8::E, Registers8::H),
            0x5D => Instructions::LD_r8_r8(Registers8::E, Registers8::L),
            0x5E => Instructions::LD_r8_HL(Registers8::E),
            0x5F => Instructions::LD_r8_r8(Registers8::E, Registers8::A),
            0x60 => Instructions::LD_r8_r8(Registers8::H, Registers8::B),
            0x61 => Instructions::LD_r8_r8(Registers8::H, Registers8::C),
            0x62 => Instructions::LD_r8_r8(Registers8::H, Registers8::D),
            0x63 => Instructions::LD_r8_r8(Registers8::H, Registers8::E),
            0x64 => Instructions::LD_r8_r8(Registers8::H, Registers8::H),
            0x65 => Instructions::LD_r8_r8(Registers8::H, Registers8::L),
            0x66 => Instructions::LD_r8_HL(Registers8::H),
            0x67 => Instructions::LD_r8_r8(Registers8::H, Registers8::A),
            0x68 => Instructions::LD_r8_r8(Registers8::L, Registers8::B),
            0x69 => Instructions::LD_r8_r8(Registers8::L, Registers8::C),
            0x6A => Instructions::LD_r8_r8(Registers8::L, Registers8::D),
            0x6B => Instructions::LD_r8_r8(Registers8::L, Registers8::E),
            0x6C => Instructions::LD_r8_r8(Registers8::L, Registers8::H),
            0x6D => Instructions::LD_r8_r8(Registers8::L, Registers8::L),
            0x6E => Instructions::LD_r8_HL(Registers8::L),
            0x6F => Instructions::LD_r8_r8(Registers8::L, Registers8::A),
            0x70 => Instructions::LD_HL_r8(Registers8::B),
            0x71 => Instructions::LD_HL_r8(Registers8::C),
            0x72 => Instructions::LD_HL_r8(Registers8::D),
            0x73 => Instructions::LD_HL_r8(Registers8::E),
            0x74 => Instructions::LD_HL_r8(Registers8::H),
            0x75 => Instructions::LD_HL_r8(Registers8::L),
            0x76 => Instructions::HALT,
            0x77 => Instructions::LD_HL_r8(Registers8::A),
            0x78 => Instructions::LD_r8_r8(Registers8::A, Registers8::B),
            0x79 => Instructions::LD_r8_r8(Registers8::A, Registers8::C),
            0x7A => Instructions::LD_r8_r8(Registers8::A, Registers8::D),
            0x7B => Instructions::LD_r8_r8(Registers8::A, Registers8::E),
            0x7C => Instructions::LD_r8_r8(Registers8::A, Registers8::H),
            0x7D => Instructions::LD_r8_r8(Registers8::A, Registers8::L),
            0x7E => Instructions::LD_r8_HL(Registers8::A),
            0x7F => Instructions::LD_r8_r8(Registers8::A, Registers8::A),
            0x80 => Instructions::ADD_A_r8(Registers8::B),
            0x81 => Instructions::ADD_A_r8(Registers8::C),
            0x82 => Instructions::ADD_A_r8(Registers8::D),
            0x83 => Instructions::ADD_A_r8(Registers8::E),
            0x84 => Instructions::ADD_A_r8(Registers8::H),
            0x85 => Instructions::ADD_A_r8(Registers8::L),
            0x86 => Instructions::ADD_A_HL,
            0x87 => Instructions::ADD_A_r8(Registers8::A),
            0x88 => Instructions::ADC_A_r8(Registers8::B),
            0x89 => Instructions::ADC_A_r8(Registers8::C),
            0x8A => Instructions::ADC_A_r8(Registers8::D),
            0x8B => Instructions::ADC_A_r8(Registers8::E),
            0x8C => Instructions::ADC_A_r8(Registers8::H),
            0x8D => Instructions::ADC_A_r8(Registers8::L),
            0x8E => Instructions::ADC_A_HL,
            0x8F => Instructions::ADC_A_r8(Registers8::A),
            0x90 => Instructions::SUB_A_r8(Registers8::B),
            0x91 => Instructions::SUB_A_r8(Registers8::C),
            0x92 => Instructions::SUB_A_r8(Registers8::D),
            0x93 => Instructions::SUB_A_r8(Registers8::E),
            0x94 => Instructions::SUB_A_r8(Registers8::H),
            0x95 => Instructions::SUB_A_r8(Registers8::L),
            0x96 => Instructions::SUB_A_HL,
            0x97 => Instructions::SUB_A_r8(Registers8::A),
            0x98 => Instructions::SBC_A_r8(Registers8::B),
            0x99 => Instructions::SBC_A_r8(Registers8::C),
            0x9A => Instructions::SBC_A_r8(Registers8::D),
            0x9B => Instructions::SBC_A_r8(Registers8::E),
            0x9C => Instructions::SBC_A_r8(Registers8::H),
            0x9D => Instructions::SBC_A_r8(Registers8::L),
            0x9E => Instructions::SBC_A_HL,
            0x9F => Instructions::SBC_A_r8(Registers8::A),
            0xA0 => Instructions::AND_A_r8(Registers8::B),
            0xA1 => Instructions::AND_A_r8(Registers8::C),
            0xA2 => Instructions::AND_A_r8(Registers8::D),
            0xA3 => Instructions::AND_A_r8(Registers8::E),
            0xA4 => Instructions::AND_A_r8(Registers8::H),
            0xA5 => Instructions::AND_A_r8(Registers8::L),
            0xA6 => Instructions::AND_A_HL,
            0xA7 => Instructions::AND_A_r8(Registers8::A),
            0xA8 => Instructions::XOR_A_r8(Registers8::B),
            0xA9 => Instructions::XOR_A_r8(Registers8::C),
            0xAA => Instructions::XOR_A_r8(Registers8::D),
            0xAB => Instructions::XOR_A_r8(Registers8::E),
            0xAC => Instructions::XOR_A_r8(Registers8::H),
            0xAD => Instructions::XOR_A_r8(Registers8::L),
            0xAE => Instructions::XOR_A_HL,
            0xAF => Instructions::XOR_A_r8(Registers8::A),
            0xB0 => Instructions::OR_A_r8(Registers8::B),
            0xB1 => Instructions::OR_A_r8(Registers8::C),
            0xB2 => Instructions::OR_A_r8(Registers8::D),
            0xB3 => Instructions::OR_A_r8(Registers8::E),
            0xB4 => Instructions::OR_A_r8(Registers8::H),
            0xB5 => Instructions::OR_A_r8(Registers8::L),
            0xB6 => Instructions::OR_A_HL,
            0xB7 => Instructions::OR_A_r8(Registers8::A),
            0xB8 => Instructions::CP_A_r8(Registers8::B),
            0xB9 => Instructions::CP_A_r8(Registers8::C),
            0xBA => Instructions::CP_A_r8(Registers8::D),
            0xBB => Instructions::CP_A_r8(Registers8::E),
            0xBC => Instructions::CP_A_r8(Registers8::H),
            0xBD => Instructions::CP_A_r8(Registers8::L),
            0xBE => Instructions::CP_A_HL,
            0xBF => Instructions::CP_A_r8(Registers8::A),
            0xC0 => Instructions::RET_cc,
            0xC1 => Instructions::POP_r16(Registers16::BC),
            0xC2 => Instructions::JP_cc_n16,
            0xC3 => Instructions::JP_n16,
            0xC4 => Instructions::CALL_cc_n16,
            0xC5 => Instructions::PUSH_r16(Registers16::BC),
            0xC6 => Instructions::ADD_A_n8,
            0xC7 => Instructions::RST,
            0xC8 => Instructions::RET_cc,
            0xC9 => Instructions::RET,
            0xCA => Instructions::JP_cc_n16,
            0xCB => Instructions::PREFIX,
            0xCC => Instructions::CALL_cc_n16,
            0xCD => Instructions::CALL_n16,
            0xCE => Instructions::ADC_A_n8,
            0xCF => Instructions::RST,
            0xD0 => Instructions::RET_cc,
            0xD1 => Instructions::POP_r16(Registers16::DE),
            0xD2 => Instructions::JP_cc_n16,
            0xD3 => Instructions::NONE,
            0xD4 => Instructions::CALL_cc_n16,
            0xD5 => Instructions::PUSH_r16(Registers16::DE),
            0xD6 => Instructions::SUB_A_n8,
            0xD7 => Instructions::RST,
            0xD8 => Instructions::RET_cc,
            0xD9 => Instructions::RETI,
            0xDA => Instructions::JP_cc_n16,
            0xDB => Instructions::NONE,
            0xDC => Instructions::CALL_cc_n16,
            0xDD => Instructions::NONE,
            0xDE => Instructions::SBC_A_n8,
            0xDF => Instructions::RST,
            0xE0 => Instructions::LDH_n16_A,
            0xE1 => Instructions::POP_r16(Registers16::HL),
            0xE2 => Instructions::LDH_C_A,
            0xE3 => Instructions::NONE,
            0xE4 => Instructions::NONE,
            0xE5 => Instructions::PUSH_r16(Registers16::HL),
            0xE6 => Instructions::AND_A_n8,
            0xE7 => Instructions::RST,
            0xE8 => Instructions::ADD_SP_e8,
            0xE9 => Instructions::JP_HL,
            0xEA => Instructions::LD_n16_A,
            0xEB => Instructions::NONE,
            0xEC => Instructions::NONE,
            0xED => Instructions::NONE,
            0xEE => Instructions::XOR_A_n8,
            0xEF => Instructions::RST,
            0xF0 => Instructions::LDH_A_n16,
            0xF1 => Instructions::POP_AF,
            0xF2 => Instructions::LDH_A_C,
            0xF3 => Instructions::DI,
            0xF4 => Instructions::NONE,
            0xF5 => Instructions::PUSH_AF,
            0xF6 => Instructions::OR_A_n8,
            0xF7 => Instructions::RST,
            0xF8 => Instructions::LD_HL_SP_e8,
            0xF9 => Instructions::LD_SP_HL,
            0xFA => Instructions::LD_A_n16,
            0xFB => Instructions::EI,
            0xFC => Instructions::NONE,
            0xFD => Instructions::NONE,
            0xFE => Instructions::CP_A_n8,
            0xFF => Instructions::RST,
        }
    }

    pub fn get_instruction_prefixed(opcode: Byte) -> Instructions {
        match opcode {
            0x00 => Instructions::RLC_r8(Registers8::B),
            0x01 => Instructions::RLC_r8(Registers8::C),
            0x02 => Instructions::RLC_r8(Registers8::D),
            0x03 => Instructions::RLC_r8(Registers8::E),
            0x04 => Instructions::RLC_r8(Registers8::H),
            0x05 => Instructions::RLC_r8(Registers8::L),
            0x06 => Instructions::RLC_HL,
            0x07 => Instructions::RLC_r8(Registers8::A),
            0x08 => Instructions::RRC_r8(Registers8::B),
            0x09 => Instructions::RRC_r8(Registers8::C),
            0x0A => Instructions::RRC_r8(Registers8::D),
            0x0B => Instructions::RRC_r8(Registers8::E),
            0x0C => Instructions::RRC_r8(Registers8::H),
            0x0D => Instructions::RRC_r8(Registers8::L),
            0x0E => Instructions::RRC_HL,
            0x0F => Instructions::RRC_r8(Registers8::A),
            0x10 => Instructions::RL_r8(Registers8::B),
            0x11 => Instructions::RL_r8(Registers8::C),
            0x12 => Instructions::RL_r8(Registers8::D),
            0x13 => Instructions::RL_r8(Registers8::E),
            0x14 => Instructions::RL_r8(Registers8::H),
            0x15 => Instructions::RL_r8(Registers8::L),
            0x16 => Instructions::RL_HL,
            0x17 => Instructions::RL_r8(Registers8::A),
            0x18 => Instructions::RR_r8(Registers8::B),
            0x19 => Instructions::RR_r8(Registers8::C),
            0x1A => Instructions::RR_r8(Registers8::D),
            0x1B => Instructions::RR_r8(Registers8::E),
            0x1C => Instructions::RR_r8(Registers8::H),
            0x1D => Instructions::RR_r8(Registers8::L),
            0x1E => Instructions::RR_HL,
            0x1F => Instructions::RR_r8(Registers8::A),
            0x20 => Instructions::SLA_r8(Registers8::B),
            0x21 => Instructions::SLA_r8(Registers8::C),
            0x22 => Instructions::SLA_r8(Registers8::D),
            0x23 => Instructions::SLA_r8(Registers8::E),
            0x24 => Instructions::SLA_r8(Registers8::H),
            0x25 => Instructions::SLA_r8(Registers8::L),
            0x26 => Instructions::SLA_HL,
            0x27 => Instructions::SLA_r8(Registers8::A),
            0x28 => Instructions::SRA_r8(Registers8::B),
            0x29 => Instructions::SRA_r8(Registers8::C),
            0x2A => Instructions::SRA_r8(Registers8::D),
            0x2B => Instructions::SRA_r8(Registers8::E),
            0x2C => Instructions::SRA_r8(Registers8::H),
            0x2D => Instructions::SRA_r8(Registers8::L),
            0x2E => Instructions::SRA_HL,
            0x2F => Instructions::SRA_r8(Registers8::A),
            0x30 => Instructions::SWAP_r8(Registers8::B),
            0x31 => Instructions::SWAP_r8(Registers8::C),
            0x32 => Instructions::SWAP_r8(Registers8::D),
            0x33 => Instructions::SWAP_r8(Registers8::E),
            0x34 => Instructions::SWAP_r8(Registers8::H),
            0x35 => Instructions::SWAP_r8(Registers8::L),
            0x36 => Instructions::SWAP_HL,
            0x37 => Instructions::SWAP_r8(Registers8::A),
            0x38 => Instructions::SRL_r8(Registers8::B),
            0x39 => Instructions::SRL_r8(Registers8::C),
            0x3A => Instructions::SRL_r8(Registers8::D),
            0x3B => Instructions::SRL_r8(Registers8::E),
            0x3C => Instructions::SRL_r8(Registers8::H),
            0x3D => Instructions::SRL_r8(Registers8::L),
            0x3E => Instructions::SRL_HL,
            0x3F => Instructions::SRL_r8(Registers8::A),
            0x40 => Instructions::BIT_u3_r8(0, Registers8::B),
            0x41 => Instructions::BIT_u3_r8(0, Registers8::C),
            0x42 => Instructions::BIT_u3_r8(0, Registers8::D),
            0x43 => Instructions::BIT_u3_r8(0, Registers8::E),
            0x44 => Instructions::BIT_u3_r8(0, Registers8::H),
            0x45 => Instructions::BIT_u3_r8(0, Registers8::L),
            0x46 => Instructions::BIT_u3_HL(0),
            0x47 => Instructions::BIT_u3_r8(0, Registers8::A),
            0x48 => Instructions::BIT_u3_r8(1, Registers8::B),
            0x49 => Instructions::BIT_u3_r8(1, Registers8::C),
            0x4A => Instructions::BIT_u3_r8(1, Registers8::D),
            0x4B => Instructions::BIT_u3_r8(1, Registers8::E),
            0x4C => Instructions::BIT_u3_r8(1, Registers8::H),
            0x4D => Instructions::BIT_u3_r8(1, Registers8::L),
            0x4E => Instructions::BIT_u3_HL(1),
            0x4F => Instructions::BIT_u3_r8(1, Registers8::A),
            0x50 => Instructions::BIT_u3_r8(2, Registers8::B),
            0x51 => Instructions::BIT_u3_r8(2, Registers8::C),
            0x52 => Instructions::BIT_u3_r8(2, Registers8::D),
            0x53 => Instructions::BIT_u3_r8(2, Registers8::E),
            0x54 => Instructions::BIT_u3_r8(2, Registers8::H),
            0x55 => Instructions::BIT_u3_r8(2, Registers8::L),
            0x56 => Instructions::BIT_u3_HL(2),
            0x57 => Instructions::BIT_u3_r8(2, Registers8::A),
            0x58 => Instructions::BIT_u3_r8(3, Registers8::B),
            0x59 => Instructions::BIT_u3_r8(3, Registers8::C),
            0x5A => Instructions::BIT_u3_r8(3, Registers8::D),
            0x5B => Instructions::BIT_u3_r8(3, Registers8::E),
            0x5C => Instructions::BIT_u3_r8(3, Registers8::H),
            0x5D => Instructions::BIT_u3_r8(3, Registers8::L),
            0x5E => Instructions::BIT_u3_HL(3),
            0x5F => Instructions::BIT_u3_r8(3, Registers8::A),
            0x60 => Instructions::BIT_u3_r8(4, Registers8::B),
            0x61 => Instructions::BIT_u3_r8(4, Registers8::C),
            0x62 => Instructions::BIT_u3_r8(4, Registers8::D),
            0x63 => Instructions::BIT_u3_r8(4, Registers8::E),
            0x64 => Instructions::BIT_u3_r8(4, Registers8::H),
            0x65 => Instructions::BIT_u3_r8(4, Registers8::L),
            0x66 => Instructions::BIT_u3_HL(4),
            0x67 => Instructions::BIT_u3_r8(4, Registers8::A),
            0x68 => Instructions::BIT_u3_r8(5, Registers8::B),
            0x69 => Instructions::BIT_u3_r8(5, Registers8::C),
            0x6A => Instructions::BIT_u3_r8(5, Registers8::D),
            0x6B => Instructions::BIT_u3_r8(5, Registers8::E),
            0x6C => Instructions::BIT_u3_r8(5, Registers8::H),
            0x6D => Instructions::BIT_u3_r8(5, Registers8::L),
            0x6E => Instructions::BIT_u3_HL(5),
            0x6F => Instructions::BIT_u3_r8(5, Registers8::A),
            0x70 => Instructions::RES_u3_r8(6, Registers8::B),
            0x71 => Instructions::RES_u3_r8(6, Registers8::C),
            0x72 => Instructions::RES_u3_r8(6, Registers8::D),
            0x73 => Instructions::RES_u3_r8(6, Registers8::E),
            0x74 => Instructions::RES_u3_r8(6, Registers8::H),
            0x75 => Instructions::RES_u3_r8(6, Registers8::L),
            0x76 => Instructions::RES_u3_HL(6),
            0x77 => Instructions::RES_u3_r8(6, Registers8::A),
            0x78 => Instructions::RES_u3_r8(7, Registers8::B),
            0x79 => Instructions::RES_u3_r8(7, Registers8::C),
            0x7A => Instructions::RES_u3_r8(7, Registers8::D),
            0x7B => Instructions::RES_u3_r8(7, Registers8::E),
            0x7C => Instructions::RES_u3_r8(7, Registers8::H),
            0x7D => Instructions::RES_u3_r8(7, Registers8::L),
            0x7E => Instructions::RES_u3_HL(7),
            0x7F => Instructions::RES_u3_r8(7, Registers8::A),
            0x80 => Instructions::RES_u3_r8(0, Registers8::B),
            0x81 => Instructions::RES_u3_r8(0, Registers8::C),
            0x82 => Instructions::RES_u3_r8(0, Registers8::D),
            0x83 => Instructions::RES_u3_r8(0, Registers8::E),
            0x84 => Instructions::RES_u3_r8(0, Registers8::H),
            0x85 => Instructions::RES_u3_r8(0, Registers8::L),
            0x86 => Instructions::RES_u3_HL(0),
            0x87 => Instructions::RES_u3_r8(0, Registers8::A),
            0x88 => Instructions::RES_u3_r8(1, Registers8::B),
            0x89 => Instructions::RES_u3_r8(1, Registers8::C),
            0x8A => Instructions::RES_u3_r8(1, Registers8::D),
            0x8B => Instructions::RES_u3_r8(1, Registers8::E),
            0x8C => Instructions::RES_u3_r8(1, Registers8::H),
            0x8D => Instructions::RES_u3_r8(1, Registers8::L),
            0x8E => Instructions::RES_u3_HL(1),
            0x8F => Instructions::RES_u3_r8(1, Registers8::A),
            0x90 => Instructions::RES_u3_r8(2, Registers8::B),
            0x91 => Instructions::RES_u3_r8(2, Registers8::C),
            0x92 => Instructions::RES_u3_r8(2, Registers8::D),
            0x93 => Instructions::RES_u3_r8(2, Registers8::E),
            0x94 => Instructions::RES_u3_r8(2, Registers8::H),
            0x95 => Instructions::RES_u3_r8(2, Registers8::L),
            0x96 => Instructions::RES_u3_HL(2),
            0x97 => Instructions::RES_u3_r8(2, Registers8::A),
            0x98 => Instructions::RES_u3_r8(3, Registers8::B),
            0x99 => Instructions::RES_u3_r8(3, Registers8::C),
            0x9A => Instructions::RES_u3_r8(3, Registers8::D),
            0x9B => Instructions::RES_u3_r8(3, Registers8::E),
            0x9C => Instructions::RES_u3_r8(3, Registers8::H),
            0x9D => Instructions::RES_u3_r8(3, Registers8::L),
            0x9E => Instructions::RES_u3_HL(3),
            0x9F => Instructions::RES_u3_r8(3, Registers8::A),
            0xA0 => Instructions::RES_u3_r8(4, Registers8::B),
            0xA1 => Instructions::RES_u3_r8(4, Registers8::C),
            0xA2 => Instructions::RES_u3_r8(4, Registers8::D),
            0xA3 => Instructions::RES_u3_r8(4, Registers8::E),
            0xA4 => Instructions::RES_u3_r8(4, Registers8::H),
            0xA5 => Instructions::RES_u3_r8(4, Registers8::L),
            0xA6 => Instructions::RES_u3_HL(4),
            0xA7 => Instructions::RES_u3_r8(4, Registers8::A),
            0xA8 => Instructions::RES_u3_r8(5, Registers8::B),
            0xA9 => Instructions::RES_u3_r8(5, Registers8::C),
            0xAA => Instructions::RES_u3_r8(5, Registers8::D),
            0xAB => Instructions::RES_u3_r8(5, Registers8::E),
            0xAC => Instructions::RES_u3_r8(5, Registers8::H),
            0xAD => Instructions::RES_u3_r8(5, Registers8::L),
            0xAE => Instructions::RES_u3_HL(5),
            0xAF => Instructions::RES_u3_r8(5, Registers8::A),
            0xB0 => Instructions::RES_u3_r8(6, Registers8::B),
            0xB1 => Instructions::RES_u3_r8(6, Registers8::C),
            0xB2 => Instructions::RES_u3_r8(6, Registers8::D),
            0xB3 => Instructions::RES_u3_r8(6, Registers8::E),
            0xB4 => Instructions::RES_u3_r8(6, Registers8::H),
            0xB5 => Instructions::RES_u3_r8(6, Registers8::L),
            0xB6 => Instructions::RES_u3_HL(6),
            0xB7 => Instructions::RES_u3_r8(6, Registers8::A),
            0xB8 => Instructions::RES_u3_r8(7, Registers8::B),
            0xB9 => Instructions::RES_u3_r8(7, Registers8::C),
            0xBA => Instructions::RES_u3_r8(7, Registers8::D),
            0xBB => Instructions::RES_u3_r8(7, Registers8::E),
            0xBC => Instructions::RES_u3_r8(7, Registers8::H),
            0xBD => Instructions::RES_u3_r8(7, Registers8::L),
            0xBE => Instructions::RES_u3_HL(7),
            0xBF => Instructions::RES_u3_r8(7, Registers8::A),
            0xC0 => Instructions::SET_u3_r8(0, Registers8::B),
            0xC1 => Instructions::SET_u3_r8(0, Registers8::C),
            0xC2 => Instructions::SET_u3_r8(0, Registers8::D),
            0xC3 => Instructions::SET_u3_r8(0, Registers8::E),
            0xC4 => Instructions::SET_u3_r8(0, Registers8::H),
            0xC5 => Instructions::SET_u3_r8(0, Registers8::L),
            0xC6 => Instructions::SET_u3_HL(0),
            0xC7 => Instructions::SET_u3_r8(0, Registers8::A),
            0xC8 => Instructions::SET_u3_r8(1, Registers8::B),
            0xC9 => Instructions::SET_u3_r8(1, Registers8::C),
            0xCA => Instructions::SET_u3_r8(1, Registers8::D),
            0xCB => Instructions::SET_u3_r8(1, Registers8::E),
            0xCC => Instructions::SET_u3_r8(1, Registers8::H),
            0xCD => Instructions::SET_u3_r8(1, Registers8::L),
            0xCE => Instructions::SET_u3_HL(1),
            0xCF => Instructions::SET_u3_r8(1, Registers8::A),
            0xD0 => Instructions::SET_u3_r8(2, Registers8::B),
            0xD1 => Instructions::SET_u3_r8(2, Registers8::C),
            0xD2 => Instructions::SET_u3_r8(2, Registers8::D),
            0xD3 => Instructions::SET_u3_r8(2, Registers8::E),
            0xD4 => Instructions::SET_u3_r8(2, Registers8::H),
            0xD5 => Instructions::SET_u3_r8(2, Registers8::L),
            0xD6 => Instructions::SET_u3_HL(2),
            0xD7 => Instructions::SET_u3_r8(2, Registers8::A),
            0xD8 => Instructions::SET_u3_r8(3, Registers8::B),
            0xD9 => Instructions::SET_u3_r8(3, Registers8::C),
            0xDA => Instructions::SET_u3_r8(3, Registers8::D),
            0xDB => Instructions::SET_u3_r8(3, Registers8::E),
            0xDC => Instructions::SET_u3_r8(3, Registers8::H),
            0xDD => Instructions::SET_u3_r8(3, Registers8::L),
            0xDE => Instructions::SET_u3_HL(3),
            0xDF => Instructions::SET_u3_r8(3, Registers8::A),
            0xE0 => Instructions::SET_u3_r8(4, Registers8::B),
            0xE1 => Instructions::SET_u3_r8(4, Registers8::C),
            0xE2 => Instructions::SET_u3_r8(4, Registers8::D),
            0xE3 => Instructions::SET_u3_r8(4, Registers8::E),
            0xE4 => Instructions::SET_u3_r8(4, Registers8::H),
            0xE5 => Instructions::SET_u3_r8(4, Registers8::L),
            0xE6 => Instructions::SET_u3_HL(4),
            0xE7 => Instructions::SET_u3_r8(4, Registers8::A),
            0xE8 => Instructions::SET_u3_r8(5, Registers8::B),
            0xE9 => Instructions::SET_u3_r8(5, Registers8::C),
            0xEA => Instructions::SET_u3_r8(5, Registers8::D),
            0xEB => Instructions::SET_u3_r8(5, Registers8::E),
            0xEC => Instructions::SET_u3_r8(5, Registers8::H),
            0xED => Instructions::SET_u3_r8(5, Registers8::L),
            0xEE => Instructions::SET_u3_HL(5),
            0xEF => Instructions::SET_u3_r8(5, Registers8::A),
            0xF0 => Instructions::SET_u3_r8(6, Registers8::B),
            0xF1 => Instructions::SET_u3_r8(6, Registers8::C),
            0xF2 => Instructions::SET_u3_r8(6, Registers8::D),
            0xF3 => Instructions::SET_u3_r8(6, Registers8::E),
            0xF4 => Instructions::SET_u3_r8(6, Registers8::H),
            0xF5 => Instructions::SET_u3_r8(6, Registers8::L),
            0xF6 => Instructions::SET_u3_HL(6),
            0xF7 => Instructions::SET_u3_r8(6, Registers8::A),
            0xF8 => Instructions::SET_u3_r8(7, Registers8::B),
            0xF9 => Instructions::SET_u3_r8(7, Registers8::C),
            0xFA => Instructions::SET_u3_r8(7, Registers8::D),
            0xFB => Instructions::SET_u3_r8(7, Registers8::E),
            0xFC => Instructions::SET_u3_r8(7, Registers8::H),
            0xFD => Instructions::SET_u3_r8(7, Registers8::L),
            0xFE => Instructions::SET_u3_HL(7),
            0xFF => Instructions::SET_u3_r8(7, Registers8::A),
        }
    }
}

#[allow(non_camel_case_types)]
impl CPU {
    pub(super) fn NONE(&mut self) {}

    pub(super) fn PREFIX(&mut self, mmu: &mut MMU) {
        let opcode: Byte = mmu.read_memory(self.registers.get_program_counter() as Address);
        let instructions: Instructions = Instructions::get_instruction_prefixed(opcode);
        self.registers.increase_program_counter(1);
        self.execute(mmu, instructions);
    }

    pub(super) fn LD_r8_r8(&mut self, destination: Registers8, source: Registers8) {
        let value: Register8 = self.registers.get_register_8(source);
        self.registers.set_register_8(destination, value);
        self.registers.set_program_counter(1);
    }

    pub(super) fn LD_r8_n8(&mut self, mmu: &mut MMU, destination: Registers8) {
        let value: Register8 = mmu.read_memory(self.registers.get_program_counter() as Address);
        self.registers.set_register_8(destination, value);
        self.registers.increase_program_counter(1);
    }

    pub(super) fn LD_r16_r16(&mut self, destination: Registers16, source: Registers16) {
        let value: Register16 = self.registers.get_register_16(source);
        self.registers.set_register_16(destination, value);
        self.registers.set_program_counter(1);
    }

    pub(super) fn LD_r16_n16(&mut self, mmu: &mut MMU, destination: Registers16) {
        let value: Register16 =
            mmu.read_memory(self.registers.get_program_counter() as Address) as Register16;
        self.registers.set_register_16(destination, value);
        self.registers.increase_program_counter(2);
    }

    pub(super) fn LD_HL_r8(&mut self, mmu: &mut MMU, source: Registers8) {
        let value: Register8 = self.registers.get_register_8(source);
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        mmu.write_memory(address, value);
    }

    pub(super) fn LD_HL_n8(&mut self, mmu: &mut MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let value: Register8 = mmu.read_memory(address_pc);
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        mmu.write_memory(address, value);
        self.registers.increase_program_counter(1);
    }

    pub(super) fn LD_r8_HL(&mut self, mmu: &MMU, destination: Registers8) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        self.registers.set_register_8(destination, value);
    }

    pub(super) fn LD_r16_A(&mut self, mmu: &mut MMU, destination: Registers16) {
        let address: Address = self.registers.get_register_16(destination) as Address;
        let value: Register8 = self.registers.get_register_8(Registers8::A);
        mmu.write_memory(address, value);
    }

    pub(super) fn LD_n16_A(&mut self, mmu: &mut MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let address: Address = mmu.read_memory(address_pc) as Address;
        let value: Register8 = self.registers.get_register_8(Registers8::A);
        mmu.write_memory(address, value);
        self.registers.increase_program_counter(2);
    }

    pub(super) fn LDH_n16_A(&mut self, mmu: &mut MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let offset: Byte = mmu.read_memory(address_pc);
        let address: Address = 0xFF00 + offset as Address;
        let value: Register8 = self.registers.get_register_8(Registers8::A);
        mmu.write_memory(address, value);
        self.registers.increase_program_counter(1);
    }

    pub(super) fn LDH_C_A(&mut self, mmu: &mut MMU) {
        let offset: Byte = self.registers.get_register_8(Registers8::C);
        let address: Address = 0xFF00 + offset as Address;
        let value: Register8 = self.registers.get_register_8(Registers8::A);
        mmu.write_memory(address, value);
    }

    pub(super) fn LD_A_r16(&mut self, mmu: &MMU, source: Registers16) {
        let address: Address = self.registers.get_register_16(source) as Address;
        let value: Register8 = mmu.read_memory(address);
        self.registers.set_register_8(Registers8::A, value);
    }

    pub(super) fn LD_A_n16(&mut self, mmu: &MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let address: Address = mmu.read_memory(address_pc) as Address;
        let value: Register8 = mmu.read_memory(address);
        self.registers.set_register_8(Registers8::A, value);
        self.registers.increase_program_counter(2);
    }

    pub(super) fn LDH_A_n16(&mut self, mmu: &MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let offset: Byte = mmu.read_memory(address_pc);
        let address: Address = 0xFF00 + offset as Address;
        let value: Register8 = mmu.read_memory(address);
        self.registers.set_register_8(Registers8::A, value);
        self.registers.increase_program_counter(1);
    }

    pub(super) fn LDH_A_C(&mut self, mmu: &MMU) {
        let offset: Byte = self.registers.get_register_8(Registers8::C);
        let address: Address = 0xFF00 + offset as Address;
        let value: Register8 = mmu.read_memory(address);
        self.registers.set_register_8(Registers8::A, value);
    }

    pub(super) fn LD_HLI_A(&mut self, mmu: &mut MMU) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = self.registers.get_register_8(Registers8::A);
        mmu.write_memory(address, value);
        let hl: Register16 = self.registers.get_register_16(Registers16::HL);
        self.registers
            .set_register_16(Registers16::HL, hl.wrapping_add(1));
    }

    pub(super) fn LD_HLD_A(&mut self, mmu: &mut MMU) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = self.registers.get_register_8(Registers8::A);
        mmu.write_memory(address, value);
        let hl: Register16 = self.registers.get_register_16(Registers16::HL);
        self.registers
            .set_register_16(Registers16::HL, hl.wrapping_sub(1));
    }

    pub(super) fn LD_A_HLI(&mut self, mmu: &MMU) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        self.registers.set_register_8(Registers8::A, value);
        let hl: Register16 = self.registers.get_register_16(Registers16::HL);
        self.registers
            .set_register_16(Registers16::HL, hl.wrapping_add(1));
    }

    pub(super) fn LD_A_HLD(&mut self, mmu: &MMU) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        self.registers.set_register_8(Registers8::A, value);
        let hl: Register16 = self.registers.get_register_16(Registers16::HL);
        self.registers
            .set_register_16(Registers16::HL, hl.wrapping_sub(1));
    }

    pub(super) fn ADC_A_r8(&mut self, source: Registers8) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let value: Register8 = self.registers.get_register_8(source);
        let carry: Byte = if self.registers.get_flag(Flags::Carry) {
            1
        } else {
            0
        };
        let (result, did_overflow1) = a.overflowing_add(value);
        let (result, did_overflow2) = result.overflowing_add(carry);
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers
            .set_flag(Flags::HalfCarry, (a & 0x0F) + (value & 0x0F) + carry > 0x0F);
        self.registers
            .set_flag(Flags::Carry, did_overflow1 || did_overflow2);

        self.registers.set_program_counter(1);
    }

    pub(super) fn ADC_A_HL(&mut self, mmu: &MMU) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let carry: Byte = if self.registers.get_flag(Flags::Carry) {
            1
        } else {
            0
        };
        let (result, did_overflow1) = a.overflowing_add(value);
        let (result, did_overflow2) = result.overflowing_add(carry);
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers
            .set_flag(Flags::HalfCarry, (a & 0x0F) + (value & 0x0F) + carry > 0x0F);
        self.registers
            .set_flag(Flags::Carry, did_overflow1 || did_overflow2);

        self.registers.set_program_counter(1);
    }

    pub(super) fn ADC_A_n8(&mut self, mmu: &MMU) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let value: Register8 = mmu.read_memory(address_pc);
        let carry: Byte = if self.registers.get_flag(Flags::Carry) {
            1
        } else {
            0
        };
        let (result, did_overflow1) = a.overflowing_add(value);
        let (result, did_overflow2) = result.overflowing_add(carry);
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers
            .set_flag(Flags::HalfCarry, (a & 0x0F) + (value & 0x0F) + carry > 0x0F);
        self.registers
            .set_flag(Flags::Carry, did_overflow1 || did_overflow2);

        self.registers.increase_program_counter(1);
    }

    pub(super) fn ADD_A_r8(&mut self, source: Registers8) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let value: Register8 = self.registers.get_register_8(source);
        let (result, did_overflow) = a.overflowing_add(value);
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers
            .set_flag(Flags::HalfCarry, (a & 0x0F) + (value & 0x0F) > 0x0F);
        self.registers.set_flag(Flags::Carry, did_overflow);

        self.registers.set_program_counter(1);
    }

    pub(super) fn ADD_A_HL(&mut self, mmu: &MMU) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let (result, did_overflow) = a.overflowing_add(value);
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers
            .set_flag(Flags::HalfCarry, (a & 0x0F) + (value & 0x0F) > 0x0F);
        self.registers.set_flag(Flags::Carry, did_overflow);

        self.registers.set_program_counter(1);
    }

    pub(super) fn ADD_A_n8(&mut self, mmu: &MMU) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let value: Register8 = mmu.read_memory(address_pc);
        let (result, did_overflow) = a.overflowing_add(value);
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers
            .set_flag(Flags::HalfCarry, (a & 0x0F) + (value & 0x0F) > 0x0F);
        self.registers.set_flag(Flags::Carry, did_overflow);

        self.registers.increase_program_counter(1);
    }

    pub(super) fn CP_A_r8(&mut self, source: Registers8) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let value: Register8 = self.registers.get_register_8(source);
        let (result, did_overflow) = a.overflowing_sub(value);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, true);
        self.registers
            .set_flag(Flags::HalfCarry, (a & 0x0F) < (value & 0x0F));
        self.registers.set_flag(Flags::Carry, did_overflow);

        self.registers.set_program_counter(1);
    }

    pub(super) fn CP_A_HL(&mut self, mmu: &MMU) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let (result, did_overflow) = a.overflowing_sub(value);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, true);
        self.registers
            .set_flag(Flags::HalfCarry, (a & 0x0F) < (value & 0x0F));
        self.registers.set_flag(Flags::Carry, did_overflow);

        self.registers.set_program_counter(1);
    }

    pub(super) fn CP_A_n8(&mut self, mmu: &MMU) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let value: Register8 = mmu.read_memory(address_pc);
        let (result, did_overflow) = a.overflowing_sub(value);
        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, true);
        self.registers
            .set_flag(Flags::HalfCarry, (a & 0x0F) < (value & 0x0F));
        self.registers.set_flag(Flags::Carry, did_overflow);
        self.registers.increase_program_counter(1);
    }

    pub(super) fn DEC_r8(&mut self, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let (result, did_overflow) = value.overflowing_sub(1);
        self.registers.set_register_8(target, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, true);
        self.registers
            .set_flag(Flags::HalfCarry, (value & 0x0F) == 0x00);

        self.registers.set_program_counter(1);
    }

    pub(super) fn DEC_HL(&mut self, mmu: &mut MMU) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let (result, did_overflow) = value.overflowing_sub(1);
        mmu.write_memory(address, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, true);
        self.registers
            .set_flag(Flags::HalfCarry, (value & 0x0F) == 0x00);

        self.registers.set_program_counter(1);
    }

    pub(super) fn INC_r8(&mut self, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let (result, did_overflow) = value.overflowing_add(1);
        self.registers.set_register_8(target, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers
            .set_flag(Flags::HalfCarry, (value & 0x0F) == 0x0F);

        self.registers.set_program_counter(1);
    }

    pub(super) fn INC_HL(&mut self, mmu: &mut MMU) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let (result, did_overflow) = value.overflowing_add(1);
        mmu.write_memory(address, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers
            .set_flag(Flags::HalfCarry, (value & 0x0F) == 0x0F);

        self.registers.set_program_counter(1);
    }

    pub(super) fn SBC_A_r8(&mut self, source: Registers8) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let value: Register8 = self.registers.get_register_8(source);
        let carry: Byte = if self.registers.get_flag(Flags::Carry) {
            1
        } else {
            0
        };
        let (result, did_overflow1) = a.overflowing_sub(value);
        let (result, did_overflow2) = result.overflowing_sub(carry);
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, true);
        self.registers
            .set_flag(Flags::HalfCarry, (a & 0x0F) < (value & 0x0F) + carry);
        self.registers
            .set_flag(Flags::Carry, did_overflow1 || did_overflow2);

        self.registers.set_program_counter(1);
    }

    pub(super) fn SBC_A_HL(&mut self, mmu: &MMU) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let carry: Byte = if self.registers.get_flag(Flags::Carry) {
            1
        } else {
            0
        };
        let (result, did_overflow1) = a.overflowing_sub(value);
        let (result, did_overflow2) = result.overflowing_sub(carry);
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, true);
        self.registers
            .set_flag(Flags::HalfCarry, (a & 0x0F) < (value & 0x0F) + carry);
        self.registers
            .set_flag(Flags::Carry, did_overflow1 || did_overflow2);

        self.registers.set_program_counter(1);
    }

    pub(super) fn SBC_A_n8(&mut self, mmu: &MMU) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let value: Register8 = mmu.read_memory(address_pc);
        let carry: Byte = if self.registers.get_flag(Flags::Carry) {
            1
        } else {
            0
        };
        let (result, did_overflow1) = a.overflowing_sub(value);
        let (result, did_overflow2) = result.overflowing_sub(carry);
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, true);
        self.registers
            .set_flag(Flags::HalfCarry, (a & 0x0F) < (value & 0x0F) + carry);
        self.registers
            .set_flag(Flags::Carry, did_overflow1 || did_overflow2);

        self.registers.increase_program_counter(1);
    }

    pub(super) fn SUB_A_r8(&mut self, source: Registers8) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let value: Register8 = self.registers.get_register_8(source);
        let (result, did_overflow) = a.overflowing_sub(value);
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, true);
        self.registers
            .set_flag(Flags::HalfCarry, (a & 0x0F) < (value & 0x0F));
        self.registers.set_flag(Flags::Carry, did_overflow);

        self.registers.set_program_counter(1);
    }

    pub(super) fn SUB_A_HL(&mut self, mmu: &MMU) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let (result, did_overflow) = a.overflowing_sub(value);
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, true);
        self.registers
            .set_flag(Flags::HalfCarry, (a & 0x0F) < (value & 0x0F));
        self.registers.set_flag(Flags::Carry, did_overflow);

        self.registers.set_program_counter(1);
    }

    pub(super) fn SUB_A_n8(&mut self, mmu: &MMU) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let value: Register8 = mmu.read_memory(address_pc);
        let (result, did_overflow) = a.overflowing_sub(value);
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, true);
        self.registers
            .set_flag(Flags::HalfCarry, (a & 0x0F) < (value & 0x0F));
        self.registers.set_flag(Flags::Carry, did_overflow);

        self.registers.increase_program_counter(1);
    }

    pub(super) fn ADD_HL_r16(&mut self, source: Registers16) {
        let hl: Register16 = self.registers.get_register_16(Registers16::HL);
        let value: Register16 = self.registers.get_register_16(source);
        let (result, did_overflow) = hl.overflowing_add(value);
        self.registers.set_register_16(Registers16::HL, result);

        self.registers.set_flag(Flags::Subtraction, false);
        self.registers
            .set_flag(Flags::HalfCarry, (hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF);
        self.registers.set_flag(Flags::Carry, did_overflow);

        self.registers.set_program_counter(1);
    }

    pub(super) fn DEC_r16(&mut self, target: Registers16) {
        let value: Register16 = self.registers.get_register_16(target);
        let (result, did_overflow) = value.overflowing_sub(1);
        self.registers.set_register_16(target, result);
        self.registers.set_program_counter(1);
    }

    pub(super) fn INC_r16(&mut self, target: Registers16) {
        let value: Register16 = self.registers.get_register_16(target);
        let (result, did_overflow) = value.overflowing_add(1);
        self.registers.set_register_16(target, result);
        self.registers.set_program_counter(1);
    }

    pub(super) fn AND_A_r8(&mut self, source: Registers8) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let value: Register8 = self.registers.get_register_8(source);
        let result: Register8 = a & value;
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, true);
        self.registers.set_flag(Flags::Carry, false);

        self.registers.set_program_counter(1);
    }

    pub(super) fn AND_A_HL(&mut self, mmu: &MMU) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let result: Register8 = a & value;
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, true);
        self.registers.set_flag(Flags::Carry, false);

        self.registers.set_program_counter(1);
    }

    pub(super) fn AND_A_n8(&mut self, mmu: &MMU) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let value: Register8 = mmu.read_memory(address_pc);
        let result: Register8 = a & value;
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, true);
        self.registers.set_flag(Flags::Carry, false);

        self.registers.increase_program_counter(1);
    }

    pub(super) fn CPL(&mut self) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let result: Register8 = !a;
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Subtraction, true);
        self.registers.set_flag(Flags::HalfCarry, true);

        self.registers.set_program_counter(1);
    }

    pub(super) fn OR_A_r8(&mut self, source: Registers8) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let value: Register8 = self.registers.get_register_8(source);
        let result: Register8 = a | value;
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, false);

        self.registers.set_program_counter(1);
    }

    pub(super) fn OR_A_HL(&mut self, mmu: &MMU) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let result: Register8 = a | value;
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, false);

        self.registers.set_program_counter(1);
    }

    pub(super) fn OR_A_n8(&mut self, mmu: &MMU) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let value: Register8 = mmu.read_memory(address_pc);
        let result: Register8 = a | value;
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, false);

        self.registers.increase_program_counter(1);
    }

    pub(super) fn XOR_A_r8(&mut self, source: Registers8) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let value: Register8 = self.registers.get_register_8(source);
        let result: Register8 = a ^ value;
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, false);

        self.registers.set_program_counter(1);
    }

    pub(super) fn XOR_A_HL(&mut self, mmu: &MMU) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let result: Register8 = a ^ value;
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, false);

        self.registers.set_program_counter(1);
    }

    pub(super) fn XOR_A_n8(&mut self, mmu: &MMU) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let value: Register8 = mmu.read_memory(address_pc);
        let result: Register8 = a ^ value;
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, false);

        self.registers.increase_program_counter(1);
    }

    pub(super) fn BIT_u3_r8(&mut self, bit: u3, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let bit_value: bool = (value & (1 << bit)) != 0;

        self.registers.set_flag(Flags::Zero, !bit_value);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, true);

        self.registers.set_program_counter(1);
    }

    pub(super) fn BIT_u3_HL(&mut self, mmu: &MMU, bit: u3) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let bit_value: bool = (value & (1 << bit)) != 0;

        self.registers.set_flag(Flags::Zero, !bit_value);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, true);

        self.registers.set_program_counter(1);
    }

    pub(super) fn RES_u3_r8(&mut self, bit: u3, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let result: Register8 = value & !(1 << bit);
        self.registers.set_register_8(target, result);
        self.registers.set_program_counter(1);
    }

    pub(super) fn RES_u3_HL(&mut self, mmu: &mut MMU, bit: u3) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let result: Register8 = value & !(1 << bit);
        mmu.write_memory(address, result);
        self.registers.set_program_counter(1);
    }

    pub(super) fn SET_u3_r8(&mut self, bit: u3, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let result: Register8 = value | (1 << bit);
        self.registers.set_register_8(target, result);
        self.registers.set_program_counter(1);
    }

    pub(super) fn SET_u3_HL(&mut self, mmu: &mut MMU, bit: u3) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let result: Register8 = value | (1 << bit);
        mmu.write_memory(address, result);
        self.registers.set_program_counter(1);
    }

    pub(super) fn RL_r8(&mut self, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let carry: Byte = if self.registers.get_flag(Flags::Carry) {
            1
        } else {
            0
        };
        let result: Register8 = (value << 1) | carry;
        self.registers.set_register_8(target, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (value & 0x80) != 0);

        self.registers.set_program_counter(1);
    }

    pub(super) fn RL_HL(&mut self, mmu: &mut MMU) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let carry: Byte = if self.registers.get_flag(Flags::Carry) {
            1
        } else {
            0
        };
        let result: Register8 = (value << 1) | carry;
        mmu.write_memory(address, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (value & 0x80) != 0);

        self.registers.set_program_counter(1);
    }

    pub(super) fn RLA(&mut self) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let carry: Byte = if self.registers.get_flag(Flags::Carry) {
            1
        } else {
            0
        };
        let result: Register8 = (a << 1) | carry;
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, false);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (a & 0x80) != 0);

        self.registers.set_program_counter(1);
    }

    pub(super) fn RLC_r8(&mut self, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let result: Register8 = (value << 1) | (value >> 7);
        self.registers.set_register_8(target, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (value & 0x80) != 0);

        self.registers.set_program_counter(1);
    }

    pub(super) fn RLC_HL(&mut self, mmu: &mut MMU) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let result: Register8 = (value << 1) | (value >> 7);
        mmu.write_memory(address, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (value & 0x80) != 0);

        self.registers.set_program_counter(1);
    }

    pub(super) fn RLCA(&mut self) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let result: Register8 = (a << 1) | (a >> 7);
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, false);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (a & 0x80) != 0);

        self.registers.set_program_counter(1);
    }

    pub(super) fn RR_r8(&mut self, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let carry: Byte = if self.registers.get_flag(Flags::Carry) {
            1
        } else {
            0
        };
        let result: Register8 = (value >> 1) | (carry << 7);
        self.registers.set_register_8(target, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (value & 0x01) != 0);

        self.registers.set_program_counter(1);
    }

    pub(super) fn RR_HL(&mut self, mmu: &mut MMU) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let carry: Byte = if self.registers.get_flag(Flags::Carry) {
            1
        } else {
            0
        };
        let result: Register8 = (value >> 1) | (carry << 7);
        mmu.write_memory(address, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (value & 0x01) != 0);

        self.registers.set_program_counter(1);
    }

    pub(super) fn RRA(&mut self) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let carry: Byte = if self.registers.get_flag(Flags::Carry) {
            1
        } else {
            0
        };
        let result: Register8 = (a >> 1) | (carry << 7);
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, false);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (a & 0x01) != 0);

        self.registers.set_program_counter(1);
    }

    pub(super) fn RRC_r8(&mut self, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let result: Register8 = (value >> 1) | (value << 7);
        self.registers.set_register_8(target, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (value & 0x01) != 0);

        self.registers.set_program_counter(1);
    }

    pub(super) fn RRC_HL(&mut self, mmu: &mut MMU) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let result: Register8 = (value >> 1) | (value << 7);
        mmu.write_memory(address, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (value & 0x01) != 0);

        self.registers.set_program_counter(1);
    }

    pub(super) fn RRCA(&mut self) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let result: Register8 = (a >> 1) | (a << 7);
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, false);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (a & 0x01) != 0);

        self.registers.set_program_counter(1);
    }

    pub(super) fn SLA_r8(&mut self, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let result: Register8 = value << 1;
        self.registers.set_register_8(target, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (value & 0x80) != 0);

        self.registers.set_program_counter(1);
    }

    pub(super) fn SLA_HL(&mut self, mmu: &mut MMU) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let result: Register8 = value << 1;
        mmu.write_memory(address, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (value & 0x80) != 0);

        self.registers.set_program_counter(1);
    }

    pub(super) fn SRA_r8(&mut self, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let msb: Register8 = value & 0x80;
        let result: Register8 = (value >> 1) | msb;
        self.registers.set_register_8(target, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (value & 0x01) != 0);

        self.registers.set_program_counter(1);
    }

    pub(super) fn SRA_HL(&mut self, mmu: &mut MMU) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let msb: Register8 = value & 0x80;
        let result: Register8 = (value >> 1) | msb;
        mmu.write_memory(address, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (value & 0x01) != 0);

        self.registers.set_program_counter(1);
    }

    pub(super) fn SRL_r8(&mut self, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let result: Register8 = value >> 1;
        self.registers.set_register_8(target, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (value & 0x01) != 0);

        self.registers.set_program_counter(1);
    }

    pub(super) fn SRL_HL(&mut self, mmu: &mut MMU) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let result: Register8 = value >> 1;
        mmu.write_memory(address, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (value & 0x01) != 0);

        self.registers.set_program_counter(1);
    }

    pub(super) fn SWAP_r8(&mut self, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let result: Register8 = (value << 4) | (value >> 4);
        self.registers.set_register_8(target, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, false);

        self.registers.set_program_counter(1);
    }

    pub(super) fn SWAP_HL(&mut self, mmu: &mut MMU) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let result: Register8 = (value << 4) | (value >> 4);
        mmu.write_memory(address, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, false);

        self.registers.set_program_counter(1);
    }

    pub(super) fn CALL_n16(&mut self, mmu: &mut MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let low: Byte = mmu.read_memory(address_pc);
        let high: Byte = mmu.read_memory(address_pc + 1);
        let address: Address = (((high as u16) << 8) | (low as u16)) as Address;

        let sp: Address = self.registers.get_register_16(Registers16::SP) as Address;
        let pc: Register16 = self.registers.get_program_counter() + 2 as Register16;
        let sp = sp.wrapping_sub(2);
        mmu.write_memory(sp as Address, (pc & 0x00FF) as Byte);
        mmu.write_memory((sp + 1) as Address, (pc >> 8) as Byte);
        self.registers
            .set_register_16(Registers16::SP, sp as Register16);

        self.registers.set_program_counter(address as Register16);
    }

    pub(super) fn CALL_cc_n16(&mut self, mmu: &mut MMU) {
        let opcode: Byte = mmu.read_memory(self.registers.get_program_counter() as Address - 2);
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let low: Byte = mmu.read_memory(address_pc);
        let high: Byte = mmu.read_memory(address_pc + 1);
        let address: Address = (((high as u16) << 8) | (low as u16)) as Address;

        let condition: bool = match opcode {
            0xC4 => self.registers.is_flag_down(Flags::Zero),
            0xCC => self.registers.is_flag_up(Flags::Zero),
            0xD4 => self.registers.is_flag_down(Flags::Carry),
            0xDC => self.registers.is_flag_up(Flags::Carry),
            _ => panic!("Invalid opcode for CALL_cc_n16"),
        };

        if condition {
            let sp: Address = self.registers.get_register_16(Registers16::SP) as Address;
            let pc: Register16 = self.registers.get_program_counter() + 2 as Register16;
            let sp = sp.wrapping_sub(2);
            mmu.write_memory(sp as Address, (pc & 0x00FF) as Byte);
            mmu.write_memory((sp + 1) as Address, (pc >> 8) as Byte);
            self.registers
                .set_register_16(Registers16::SP, sp as Register16);

            self.registers.set_program_counter(address as Register16);
        } else {
            self.registers.increase_program_counter(2);
        }

        self.registers.set_program_counter(1);
    }

    pub(super) fn JP_HL(&mut self) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        self.registers.set_program_counter(address as Register16);
    }

    pub(super) fn JP_n16(&mut self, mmu: &MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let low: Byte = mmu.read_memory(address_pc);
        let high: Byte = mmu.read_memory(address_pc + 1);
        let address: Address = (((high as u16) << 8) | (low as u16)) as Address;
        self.registers.set_program_counter(address as Register16);
    }

    pub(super) fn JP_cc_n16(&mut self, mmu: &MMU) {
        let opcode: Byte = mmu.read_memory(self.registers.get_program_counter() as Address - 2);
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let low: Byte = mmu.read_memory(address_pc);
        let high: Byte = mmu.read_memory(address_pc + 1);
        let address: Address = (((high as u16) << 8) | (low as u16)) as Address;

        let condition: bool = match opcode {
            0xC2 => self.registers.is_flag_down(Flags::Zero),
            0xCA => self.registers.is_flag_up(Flags::Zero),
            0xD2 => self.registers.is_flag_down(Flags::Carry),
            0xDA => self.registers.is_flag_up(Flags::Carry),
            _ => panic!("Invalid opcode for JP_cc_n16"),
        };

        if condition {
            self.registers.set_program_counter(address as Register16);
        } else {
            self.registers.increase_program_counter(2);
        }
    }

    pub(super) fn JR_n16(&mut self, mmu: &MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let offset: i8 = mmu.read_memory(address_pc) as i8;
        let pc: Register16 = self.registers.get_program_counter();
        let address: Address = (pc as i32 + offset as i32 + 1) as Address;
        self.registers.set_program_counter(address as Register16);
    }

    pub(super) fn JR_cc_n16(&mut self, mmu: &MMU) {
        let opcode: Byte = mmu.read_memory(self.registers.get_program_counter() as Address - 2);
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let offset: i8 = mmu.read_memory(address_pc) as i8;
        let pc: Register16 = self.registers.get_program_counter();

        let condition: bool = match opcode {
            0x20 => self.registers.is_flag_down(Flags::Zero),
            0x28 => self.registers.is_flag_up(Flags::Zero),
            0x30 => self.registers.is_flag_down(Flags::Carry),
            0x38 => self.registers.is_flag_up(Flags::Carry),
            _ => panic!("Invalid opcode for JR_cc_n16"),
        };

        if condition {
            let address: Address = (pc as i32 + offset as i32 + 1) as Address;
            self.registers.set_program_counter(address as Register16);
        } else {
            self.registers.increase_program_counter(1);
        }
    }

    pub(super) fn RET_cc(&mut self, mmu: &mut MMU) {
        let opcode: Byte = mmu.read_memory(self.registers.get_program_counter() as Address - 2);
        let condition: bool = match opcode {
            0xC0 => self.registers.is_flag_down(Flags::Zero),
            0xC8 => self.registers.is_flag_up(Flags::Zero),
            0xD0 => self.registers.is_flag_down(Flags::Carry),
            0xD8 => self.registers.is_flag_up(Flags::Carry),
            _ => panic!("Invalid opcode for RET_cc"),
        };
        if condition {
            let sp: Address = self.registers.get_register_16(Registers16::SP) as Address;
            let low: Byte = mmu.read_memory(sp);
            let high: Byte = mmu.read_memory(sp + 1);
            let address: Address = (((high as u16) << 8) | (low as u16)) as Address;
            self.registers
                .set_register_16(Registers16::SP, sp.wrapping_add(2) as Register16);
            self.registers.set_program_counter(address as Register16);
        } else {
            self.registers.increase_program_counter(1);
        }
    }

    pub(super) fn RET(&mut self, mmu: &mut MMU) {
        let sp: Address = self.registers.get_register_16(Registers16::SP) as Address;
        let low: Byte = mmu.read_memory(sp);
        let high: Byte = mmu.read_memory(sp + 1);
        let address: Address = (((high as u16) << 8) | (low as u16)) as Address;
        self.registers
            .set_register_16(Registers16::SP, sp.wrapping_add(2) as Register16);
        self.registers.set_program_counter(address as Register16);
    }

    pub(super) fn RETI(&mut self, mmu: &mut MMU) {
        let sp: Address = self.registers.get_register_16(Registers16::SP) as Address;
        let low: Byte = mmu.read_memory(sp);
        let high: Byte = mmu.read_memory(sp + 1);
        let address: Address = (((high as u16) << 8) | (low as u16)) as Address;
        self.registers
            .set_register_16(Registers16::SP, sp.wrapping_add(2) as Register16);
        self.registers.set_program_counter(address as Register16);
        self.set_status(IME::Enabled);
    }

    pub(super) fn RST(&mut self, mmu: &mut MMU) {
        let opcode: Byte = mmu.read_memory(self.registers.get_program_counter() as Address - 1);
        let n: Byte = match opcode {
            0xC7 => 0x00,
            0xCF => 0x08,
            0xD7 => 0x10,
            0xDF => 0x18,
            0xE7 => 0x20,
            0xEF => 0x28,
            0xF7 => 0x30,
            0xFF => 0x38,
            _ => panic!("Invalid opcode for RST"),
        };

        let pc: Register16 = self.registers.get_program_counter();
        let sp: Address = self.registers.get_register_16(Registers16::SP) as Address;

        let sp: Address = sp.wrapping_sub(1);
        mmu.write_memory(sp, (pc >> 8) as Byte);
        let sp: Address = sp.wrapping_sub(1);
        mmu.write_memory(sp, (pc & 0x00FF) as Byte);
        self.registers
            .set_register_16(Registers16::SP, sp as Register16);

        self.registers.set_program_counter(n as Register16);
    }

    pub(super) fn CCF(&mut self) {
        let carry: bool = self.registers.get_flag(Flags::Carry);
        self.registers.set_flag(Flags::Carry, !carry);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_program_counter(1);
    }

    pub(super) fn SCF(&mut self) {
        self.registers.set_flag(Flags::Carry, true);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_program_counter(1);
    }

    pub(super) fn ADD_HL_SP(&mut self) {
        let hl: Register16 = self.registers.get_register_16(Registers16::HL);
        let sp: Register16 = self.registers.get_register_16(Registers16::SP);
        let (result, did_overflow) = hl.overflowing_add(sp);
        self.registers.set_register_16(Registers16::HL, result);

        self.registers.set_flag(Flags::Subtraction, false);
        self.registers
            .set_flag(Flags::HalfCarry, (hl & 0x0FFF) + (sp & 0x0FFF) > 0x0FFF);
        self.registers.set_flag(Flags::Carry, did_overflow);

        self.registers.set_program_counter(1);
    }

    pub(super) fn ADD_SP_e8(&mut self, mmu: &MMU) {
        let sp: Register16 = self.registers.get_register_16(Registers16::SP);
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let offset: i8 = mmu.read_memory(address_pc) as i8;
        let (result, did_overflow) = sp.overflowing_add(offset as Register16);
        self.registers.set_register_16(Registers16::SP, result);

        self.registers.set_flag(Flags::Zero, false);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(
            Flags::HalfCarry,
            (sp & 0x0F) + ((offset as u16) & 0x0F) > 0x0F,
        );
        self.registers
            .set_flag(Flags::Carry, (sp & 0xFF) + ((offset as u16) & 0xFF) > 0xFF);

        self.registers.increase_program_counter(1);
    }

    pub(super) fn DEC_SP(&mut self) {
        let sp: Register16 = self.registers.get_register_16(Registers16::SP);
        let (result, _did_overflow) = sp.overflowing_sub(1);
        self.registers.set_register_16(Registers16::SP, result);
        self.registers.set_program_counter(1);
    }

    pub(super) fn INC_SP(&mut self) {
        let sp: Register16 = self.registers.get_register_16(Registers16::SP);
        let (result, _did_overflow) = sp.overflowing_add(1);
        self.registers.set_register_16(Registers16::SP, result);
        self.registers.set_program_counter(1);
    }

    pub(super) fn LD_SP_n16(&mut self, mmu: &MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let low: Byte = mmu.read_memory(address_pc);
        let high: Byte = mmu.read_memory(address_pc + 1);
        let value: Register16 = (((high as u16) << 8) | (low as u16)) as Register16;
        self.registers.set_register_16(Registers16::SP, value);
        self.registers.increase_program_counter(2);
    }

    pub(super) fn LD_n16_SP(&mut self, mmu: &mut MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let low: Byte = mmu.read_memory(address_pc);
        let high: Byte = mmu.read_memory(address_pc + 1);
        let address: Address = (((high as u16) << 8) | (low as u16)) as Address;
        let sp: Register16 = self.registers.get_register_16(Registers16::SP);
        mmu.write_memory(address, (sp & 0x00FF) as Byte);
        mmu.write_memory(address + 1, (sp >> 8) as Byte);
        self.registers.increase_program_counter(2);
    }

    pub(super) fn LD_HL_SP_e8(&mut self, mmu: &MMU) {
        let sp: Register16 = self.registers.get_register_16(Registers16::SP);
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let offset: i8 = mmu.read_memory(address_pc) as i8;
        let (result, _did_overflow) = sp.overflowing_add(offset as Register16);
        self.registers.set_register_16(Registers16::HL, result);

        self.registers.set_flag(Flags::Zero, false);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(
            Flags::HalfCarry,
            (sp & 0x0F) + ((offset as u16) & 0x0F) > 0x0F,
        );
        self.registers
            .set_flag(Flags::Carry, (sp & 0xFF) + ((offset as u16) & 0xFF) > 0xFF);

        self.registers.increase_program_counter(1);
    }

    pub(super) fn LD_SP_HL(&mut self) {
        let hl: Register16 = self.registers.get_register_16(Registers16::HL);
        self.registers.set_register_16(Registers16::SP, hl);
        self.registers.set_program_counter(1);
    }

    pub(super) fn POP_AF(&mut self, mmu: &mut MMU) {
        let sp: Address = self.registers.get_register_16(Registers16::SP) as Address;
        let low: Byte = mmu.read_memory(sp);
        let high: Byte = mmu.read_memory(sp + 1);
        let value: Register16 = (((high as u16) << 8) | (low as u16)) as Register16;
        self.registers
            .set_register_16(Registers16::AF, value & 0xFFF0);
        self.registers
            .set_register_16(Registers16::SP, sp.wrapping_add(2) as Register16);
        self.registers.set_program_counter(1);
    }

    pub(super) fn POP_r16(&mut self, mmu: &mut MMU, target: Registers16) {
        let sp: Address = self.registers.get_register_16(Registers16::SP) as Address;
        let low: Byte = mmu.read_memory(sp);
        let high: Byte = mmu.read_memory(sp + 1);
        let value: Register16 = (((high as u16) << 8) | (low as u16)) as Register16;
        self.registers.set_register_16(target, value);
        self.registers
            .set_register_16(Registers16::SP, sp.wrapping_add(2) as Register16);
        self.registers.set_program_counter(1);
    }

    pub(super) fn PUSH_AF(&mut self, mmu: &mut MMU) {
        let sp: Address = self.registers.get_register_16(Registers16::SP) as Address;
        let af: Register16 = self.registers.get_register_16(Registers16::AF);
        let sp: Address = sp.wrapping_sub(2);
        mmu.write_memory(sp as Address, (af & 0x00FF) as Byte);
        mmu.write_memory((sp + 1) as Address, (af >> 8) as Byte);
        self.registers
            .set_register_16(Registers16::SP, sp as Register16);
        self.registers.set_program_counter(1);
    }

    pub(super) fn PUSH_r16(&mut self, mmu: &mut MMU, source: Registers16) {
        let sp: Address = self.registers.get_register_16(Registers16::SP) as Address;
        let value: Register16 = self.registers.get_register_16(source);
        let sp: Address = sp.wrapping_sub(2);
        mmu.write_memory(sp as Address, (value & 0x00FF) as Byte);
        mmu.write_memory((sp + 1) as Address, (value >> 8) as Byte);
        self.registers
            .set_register_16(Registers16::SP, sp as Register16);
        self.registers.set_program_counter(1);
    }

    pub(super) fn DI(&mut self) {
        self.set_status(IME::Disabled);
        self.registers.set_program_counter(1);
    }

    pub(super) fn EI(&mut self) {
        self.enable_ime_next_cycle = true;
        self.registers.set_program_counter(1);
    }

    pub(super) fn HALT(&mut self) {
        self.halted = true;
        self.registers.set_program_counter(1);
    }

    pub(super) fn DAA(&mut self) {
        let mut a: Register8 = self.registers.get_register_8(Registers8::A);
        let mut adjust: u8 = 0;
        let mut carry: bool = self.registers.get_flag(Flags::Carry);

        if self.registers.get_flag(Flags::HalfCarry)
            || (!self.registers.get_flag(Flags::Subtraction) && (a & 0x0F) > 9)
        {
            adjust |= 0x06;
        }
        if carry || (!self.registers.get_flag(Flags::Subtraction) && a > 0x99) {
            adjust |= 0x60;
            carry = true;
        }

        if self.registers.get_flag(Flags::Subtraction) {
            a = a.wrapping_sub(adjust);
        } else {
            a = a.wrapping_add(adjust);
        }

        self.registers.set_register_8(Registers8::A, a);
        self.registers.set_flag(Flags::Zero, a == 0);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, carry);

        self.registers.set_program_counter(1);
    }

    pub(super) fn NOP(&mut self) {
        self.registers.set_program_counter(1);
    }

    pub(super) fn STOP(&mut self) {
        self.halted = true;
        self.registers.set_program_counter(1);
    }
}
