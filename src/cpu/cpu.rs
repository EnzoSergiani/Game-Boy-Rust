use crate::{
    cartridge::cartridge::Cartridge,
    cpu::{instructions::Instructions, registers::Registers},
    mmu::mmu::{Address, Byte, MMU},
};

#[derive(Copy, Clone)]
pub enum IME {
    Enabled = 1,
    Disabled = 0,
}

pub struct CPU {
    pub registers: Registers,
    status: IME,
    pub enable_ime_next_cycle: bool,
    pub halted: bool,
}

impl CPU {
    pub fn new() -> Self {
        let _ = std::fs::remove_file("src/log/cpu.log");
        CPU {
            registers: Registers::new(),
            status: IME::Disabled,
            enable_ime_next_cycle: false,
            halted: false,
        }
    }

    pub fn start(&mut self, cartridge: &Cartridge) {
        self.set_status(IME::Enabled);
        let entry_point: Byte = cartridge.get_entry_point();
        self.registers.set_program_counter(entry_point as u16);
    }

    pub fn get_status(&self) -> IME {
        self.status
    }

    pub fn set_status(&mut self, status: IME) {
        match status {
            IME::Disabled => self.status = IME::Disabled,
            IME::Enabled => self.status = IME::Enabled,
        };
    }

    pub fn step(&mut self, mmu: &mut MMU) {
        let address: Address = self.registers.get_program_counter() as Address;
        let opcode: Byte = mmu.read_memory(address);
        let instruction: Instructions = Instructions::get_instruction(opcode);
        self.registers.increase_program_counter(1);
        self.execute(mmu, instruction);
    }

    pub fn execute(&mut self, mmu: &mut MMU, instruction: Instructions) {
        match instruction {
            Instructions::NONE => {}
            Instructions::PREFIX => self.PREFIX(mmu),
            Instructions::LD_r8_r8(destination, source) => self.LD_r8_r8(destination, source),
            Instructions::LD_r8_n8(destination) => self.LD_r8_n8(mmu, destination),
            Instructions::LD_r16_r16(destination, source) => self.LD_r16_r16(destination, source),
            Instructions::LD_r16_n16(destination) => self.LD_r16_n16(mmu, destination),
            Instructions::LD_HL_r8(source) => self.LD_HL_r8(mmu, source),
            Instructions::LD_HL_n8 => self.LD_HL_n8(mmu),
            Instructions::LD_r8_HL(destination) => self.LD_r8_HL(mmu, destination),
            Instructions::LD_r16_A(destination) => self.LD_r16_A(mmu, destination),
            Instructions::LD_n16_A => self.LD_n16_A(mmu),
            Instructions::LDH_n16_A => self.LDH_n16_A(mmu),
            Instructions::LDH_C_A => self.LDH_C_A(mmu),
            Instructions::LD_A_r16(source) => self.LD_A_r16(mmu, source),
            Instructions::LD_A_n16 => self.LD_A_n16(mmu),
            Instructions::LDH_A_n16 => self.LDH_A_n16(mmu),
            Instructions::LDH_A_C => self.LDH_A_C(mmu),
            Instructions::LD_HLI_A => self.LD_HLI_A(mmu),
            Instructions::LD_HLD_A => self.LD_HLD_A(mmu),
            Instructions::LD_A_HLI => self.LD_A_HLI(mmu),
            Instructions::LD_A_HLD => self.LD_A_HLD(mmu),
            Instructions::ADC_A_r8(source) => self.ADC_A_r8(source),
            Instructions::ADC_A_HL => self.ADC_A_HL(mmu),
            Instructions::ADC_A_n8 => self.ADC_A_n8(mmu),
            Instructions::ADD_A_r8(source) => self.ADD_A_r8(source),
            Instructions::ADD_A_HL => self.ADD_A_HL(mmu),
            Instructions::ADD_A_n8 => self.ADD_A_n8(mmu),
            Instructions::CP_A_r8(source) => self.CP_A_r8(source),
            Instructions::CP_A_HL => self.CP_A_HL(mmu),
            Instructions::CP_A_n8 => self.CP_A_n8(mmu),
            Instructions::DEC_r8(register) => self.DEC_r8(register),
            Instructions::DEC_HL => self.DEC_HL(mmu),
            Instructions::INC_r8(register) => self.INC_r8(register),
            Instructions::INC_HL => self.INC_HL(mmu),
            Instructions::SBC_A_r8(source) => self.SBC_A_r8(source),
            Instructions::SBC_A_HL => self.SBC_A_HL(mmu),
            Instructions::SBC_A_n8 => self.SBC_A_n8(mmu),
            Instructions::SUB_A_r8(source) => self.SUB_A_r8(source),
            Instructions::SUB_A_HL => self.SUB_A_HL(mmu),
            Instructions::SUB_A_n8 => self.SUB_A_n8(mmu),
            Instructions::ADD_HL_r16(source) => self.ADD_HL_r16(source),
            Instructions::DEC_r16(register) => self.DEC_r16(register),
            Instructions::INC_r16(register) => self.INC_r16(register),
            Instructions::AND_A_r8(source) => self.AND_A_r8(source),
            Instructions::AND_A_HL => self.AND_A_HL(mmu),
            Instructions::AND_A_n8 => self.AND_A_n8(mmu),
            Instructions::CPL => self.CPL(),
            Instructions::OR_A_r8(source) => self.OR_A_r8(source),
            Instructions::OR_A_HL => self.OR_A_HL(mmu),
            Instructions::OR_A_n8 => self.OR_A_n8(mmu),
            Instructions::XOR_A_r8(source) => self.XOR_A_r8(source),
            Instructions::XOR_A_HL => self.XOR_A_HL(mmu),
            Instructions::XOR_A_n8 => self.XOR_A_n8(mmu),
            Instructions::BIT_u3_r8(bit, register) => self.BIT_u3_r8(bit, register),
            Instructions::BIT_u3_HL(bit) => self.BIT_u3_HL(mmu, bit),
            Instructions::RES_u3_r8(bit, register) => self.RES_u3_r8(bit, register),
            Instructions::RES_u3_HL(bit) => self.RES_u3_HL(mmu, bit),
            Instructions::SET_u3_r8(bit, register) => self.SET_u3_r8(bit, register),
            Instructions::SET_u3_HL(bit) => self.SET_u3_HL(mmu, bit),
            Instructions::RL_r8(register) => self.RL_r8(register),
            Instructions::RL_HL => self.RL_HL(mmu),
            Instructions::RLA => self.RLA(),
            Instructions::RLC_r8(register) => self.RLC_r8(register),
            Instructions::RLC_HL => self.RLC_HL(mmu),
            Instructions::RLCA => self.RLCA(),
            Instructions::RR_r8(register) => self.RR_r8(register),
            Instructions::RR_HL => self.RR_HL(mmu),
            Instructions::RRA => self.RRA(),
            Instructions::RRC_r8(register) => self.RRC_r8(register),
            Instructions::RRC_HL => self.RRC_HL(mmu),
            Instructions::RRCA => self.RRCA(),
            Instructions::SLA_r8(register) => self.SLA_r8(register),
            Instructions::SLA_HL => self.SLA_HL(mmu),
            Instructions::SRA_r8(register) => self.SRA_r8(register),
            Instructions::SRA_HL => self.SRA_HL(mmu),
            Instructions::SRL_r8(register) => self.SRL_r8(register),
            Instructions::SRL_HL => self.SRL_HL(mmu),
            Instructions::SWAP_r8(register) => self.SWAP_r8(register),
            Instructions::SWAP_HL => self.SWAP_HL(mmu),
            Instructions::CALL_n16 => self.CALL_n16(mmu),
            Instructions::CALL_cc_n16 => self.CALL_cc_n16(mmu),
            Instructions::JP_HL => self.JP_HL(),
            Instructions::JP_n16 => self.JP_n16(mmu),
            Instructions::JP_cc_n16 => self.JP_cc_n16(mmu),
            Instructions::JR_n16 => self.JR_n16(mmu),
            Instructions::JR_cc_n16 => self.JR_cc_n16(mmu),
            Instructions::RET_cc => self.RET_cc(mmu),
            Instructions::RET => self.RET(mmu),
            Instructions::RETI => self.RETI(mmu),
            Instructions::RST => self.RST(mmu),
            Instructions::CCF => self.CCF(),
            Instructions::SCF => self.SCF(),
            Instructions::ADD_HL_SP => self.ADD_HL_SP(),
            Instructions::ADD_SP_e8 => self.ADD_SP_e8(mmu),
            Instructions::DEC_SP => self.DEC_SP(),
            Instructions::INC_SP => self.INC_SP(),
            Instructions::LD_SP_n16 => self.LD_SP_n16(mmu),
            Instructions::LD_n16_SP => self.LD_n16_SP(mmu),
            Instructions::LD_HL_SP_e8 => self.LD_HL_SP_e8(mmu),
            Instructions::LD_SP_HL => self.LD_SP_HL(),
            Instructions::POP_AF => self.POP_AF(mmu),
            Instructions::POP_r16(destination) => self.POP_r16(mmu, destination),
            Instructions::PUSH_AF => self.PUSH_AF(mmu),
            Instructions::PUSH_r16(source) => self.PUSH_r16(mmu, source),
            Instructions::DI => self.DI(),
            Instructions::EI => self.EI(),
            Instructions::HALT => self.HALT(),
            Instructions::DAA => self.DAA(),
            Instructions::NOP => self.NOP(),
            Instructions::STOP => self.STOP(),
        }
    }
}
