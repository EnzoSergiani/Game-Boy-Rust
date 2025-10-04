use crate::{
    common::types::{Address, Byte, Register8, Register16},
    cpu::{
        cpu::CPU,
        registers::{Registers8, Registers16},
    },
    mmu::mmu::MMU,
};

impl CPU {
    pub(crate) fn LD_r8_r8(&mut self, destination: Registers8, source: Registers8) {
        let value: Register8 = self.registers.get_register_8(source);
        self.registers.set_register_8(destination, value);
        self.registers.set_program_counter(1);
    }

    pub(crate) fn LD_r8_n8(&mut self, mmu: &mut MMU, destination: Registers8) {
        let value: Register8 = mmu.read_memory(self.registers.get_program_counter() as Address);
        self.registers.set_register_8(destination, value);
        self.registers.increase_program_counter(1);
    }

    pub(crate) fn LD_r16_r16(&mut self, destination: Registers16, source: Registers16) {
        let value: Register16 = self.registers.get_register_16(source);
        self.registers.set_register_16(destination, value);
        self.registers.set_program_counter(1);
    }

    pub(crate) fn LD_r16_n16(&mut self, mmu: &mut MMU, destination: Registers16) {
        let value: Register16 =
            mmu.read_memory(self.registers.get_program_counter() as Address) as Register16;
        self.registers.set_register_16(destination, value);
        self.registers.increase_program_counter(2);
    }

    pub(crate) fn LD_HL_r8(&mut self, mmu: &mut MMU, source: Registers8) {
        let value: Register8 = self.registers.get_register_8(source);
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        mmu.write_memory(address, value);
    }

    pub(crate) fn LD_HL_n8(&mut self, mmu: &mut MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let value: Register8 = mmu.read_memory(address_pc);
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        mmu.write_memory(address, value);
        self.registers.increase_program_counter(1);
    }

    pub(crate) fn LD_r8_HL(&mut self, mmu: &MMU, destination: Registers8) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        self.registers.set_register_8(destination, value);
    }

    pub(crate) fn LD_r16_A(&mut self, mmu: &mut MMU, destination: Registers16) {
        let address: Address = self.registers.get_register_16(destination) as Address;
        let value: Register8 = self.registers.get_register_8(Registers8::A);
        mmu.write_memory(address, value);
    }

    pub(crate) fn LD_n16_A(&mut self, mmu: &mut MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let address: Address = mmu.read_memory(address_pc) as Address;
        let value: Register8 = self.registers.get_register_8(Registers8::A);
        mmu.write_memory(address, value);
        self.registers.increase_program_counter(2);
    }

    pub(crate) fn LDH_n16_A(&mut self, mmu: &mut MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let offset: Byte = mmu.read_memory(address_pc);
        let address: Address = 0xFF00 + offset as Address;
        let value: Register8 = self.registers.get_register_8(Registers8::A);
        mmu.write_memory(address, value);
        self.registers.increase_program_counter(1);
    }

    pub(crate) fn LDH_C_A(&mut self, mmu: &mut MMU) {
        let offset: Byte = self.registers.get_register_8(Registers8::C);
        let address: Address = 0xFF00 + offset as Address;
        let value: Register8 = self.registers.get_register_8(Registers8::A);
        mmu.write_memory(address, value);
    }

    pub(crate) fn LD_A_r16(&mut self, mmu: &MMU, source: Registers16) {
        let address: Address = self.registers.get_register_16(source) as Address;
        let value: Register8 = mmu.read_memory(address);
        self.registers.set_register_8(Registers8::A, value);
    }

    pub(crate) fn LD_A_n16(&mut self, mmu: &MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let address: Address = mmu.read_memory(address_pc) as Address;
        let value: Register8 = mmu.read_memory(address);
        self.registers.set_register_8(Registers8::A, value);
        self.registers.increase_program_counter(2);
    }

    pub(crate) fn LDH_A_n16(&mut self, mmu: &MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let offset: Byte = mmu.read_memory(address_pc);
        let address: Address = 0xFF00 + offset as Address;
        let value: Register8 = mmu.read_memory(address);
        self.registers.set_register_8(Registers8::A, value);
        self.registers.increase_program_counter(1);
    }

    pub(crate) fn LDH_A_C(&mut self, mmu: &MMU) {
        let offset: Byte = self.registers.get_register_8(Registers8::C);
        let address: Address = 0xFF00 + offset as Address;
        let value: Register8 = mmu.read_memory(address);
        self.registers.set_register_8(Registers8::A, value);
    }

    pub(crate) fn LD_HLI_A(&mut self, mmu: &mut MMU) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = self.registers.get_register_8(Registers8::A);
        mmu.write_memory(address, value);
        let hl: Register16 = self.registers.get_register_16(Registers16::HL);
        self.registers
            .set_register_16(Registers16::HL, hl.wrapping_add(1));
    }

    pub(crate) fn LD_HLD_A(&mut self, mmu: &mut MMU) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = self.registers.get_register_8(Registers8::A);
        mmu.write_memory(address, value);
        let hl: Register16 = self.registers.get_register_16(Registers16::HL);
        self.registers
            .set_register_16(Registers16::HL, hl.wrapping_sub(1));
    }

    pub(crate) fn LD_A_HLI(&mut self, mmu: &MMU) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        self.registers.set_register_8(Registers8::A, value);
        let hl: Register16 = self.registers.get_register_16(Registers16::HL);
        self.registers
            .set_register_16(Registers16::HL, hl.wrapping_add(1));
    }

    pub(crate) fn LD_A_HLD(&mut self, mmu: &MMU) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        self.registers.set_register_8(Registers8::A, value);
        let hl: Register16 = self.registers.get_register_16(Registers16::HL);
        self.registers
            .set_register_16(Registers16::HL, hl.wrapping_sub(1));
    }
}
