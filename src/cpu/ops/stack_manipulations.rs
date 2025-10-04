use crate::{
    common::types::{Address, Byte, Register16},
    cpu::{
        cpu::CPU,
        registers::{Flags, Registers16},
    },
    mmu::mmu::MMU,
};

impl CPU {
    pub(crate) fn ADD_HL_SP(&mut self) {
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

    pub(crate) fn ADD_SP_e8(&mut self, mmu: &MMU) {
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

    pub(crate) fn DEC_SP(&mut self) {
        let sp: Register16 = self.registers.get_register_16(Registers16::SP);
        let (result, _did_overflow) = sp.overflowing_sub(1);
        self.registers.set_register_16(Registers16::SP, result);
        self.registers.set_program_counter(1);
    }

    pub(crate) fn INC_SP(&mut self) {
        let sp: Register16 = self.registers.get_register_16(Registers16::SP);
        let (result, _did_overflow) = sp.overflowing_add(1);
        self.registers.set_register_16(Registers16::SP, result);
        self.registers.set_program_counter(1);
    }

    pub(crate) fn LD_SP_n16(&mut self, mmu: &MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let low: Byte = mmu.read_memory(address_pc);
        let high: Byte = mmu.read_memory(address_pc + 1);
        let value: Register16 = (((high as u16) << 8) | (low as u16)) as Register16;
        self.registers.set_register_16(Registers16::SP, value);
        self.registers.increase_program_counter(2);
    }

    pub(crate) fn LD_n16_SP(&mut self, mmu: &mut MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let low: Byte = mmu.read_memory(address_pc);
        let high: Byte = mmu.read_memory(address_pc + 1);
        let address: Address = (((high as u16) << 8) | (low as u16)) as Address;
        let sp: Register16 = self.registers.get_register_16(Registers16::SP);
        mmu.write_memory(address, (sp & 0x00FF) as Byte);
        mmu.write_memory(address + 1, (sp >> 8) as Byte);
        self.registers.increase_program_counter(2);
    }

    pub(crate) fn LD_HL_SP_e8(&mut self, mmu: &MMU) {
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

    pub(crate) fn LD_SP_HL(&mut self) {
        let hl: Register16 = self.registers.get_register_16(Registers16::HL);
        self.registers.set_register_16(Registers16::SP, hl);
        self.registers.set_program_counter(1);
    }

    pub(crate) fn POP_AF(&mut self, mmu: &mut MMU) {
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

    pub(crate) fn POP_r16(&mut self, mmu: &mut MMU, target: Registers16) {
        let sp: Address = self.registers.get_register_16(Registers16::SP) as Address;
        let low: Byte = mmu.read_memory(sp);
        let high: Byte = mmu.read_memory(sp + 1);
        let value: Register16 = (((high as u16) << 8) | (low as u16)) as Register16;
        self.registers.set_register_16(target, value);
        self.registers
            .set_register_16(Registers16::SP, sp.wrapping_add(2) as Register16);
        self.registers.set_program_counter(1);
    }

    pub(crate) fn PUSH_AF(&mut self, mmu: &mut MMU) {
        let sp: Address = self.registers.get_register_16(Registers16::SP) as Address;
        let af: Register16 = self.registers.get_register_16(Registers16::AF);
        let sp: Address = sp.wrapping_sub(2);
        mmu.write_memory(sp as Address, (af & 0x00FF) as Byte);
        mmu.write_memory((sp + 1) as Address, (af >> 8) as Byte);
        self.registers
            .set_register_16(Registers16::SP, sp as Register16);
        self.registers.set_program_counter(1);
    }

    pub(crate) fn PUSH_r16(&mut self, mmu: &mut MMU, source: Registers16) {
        let sp: Address = self.registers.get_register_16(Registers16::SP) as Address;
        let value: Register16 = self.registers.get_register_16(source);
        let sp: Address = sp.wrapping_sub(2);
        mmu.write_memory(sp as Address, (value & 0x00FF) as Byte);
        mmu.write_memory((sp + 1) as Address, (value >> 8) as Byte);
        self.registers
            .set_register_16(Registers16::SP, sp as Register16);
        self.registers.set_program_counter(1);
    }
}
