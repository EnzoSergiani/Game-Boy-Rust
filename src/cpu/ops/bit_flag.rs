use crate::{
    common::types::{Address, Register8, u3},
    cpu::{
        cpu::CPU,
        registers::{Flags, Registers8, Registers16},
    },
    mmu::mmu::MMU,
};

impl CPU {
    pub(crate) fn BIT_u3_r8(&mut self, bit: u3, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let bit_value: bool = (value & (1 << bit)) != 0;

        self.registers.set_flag(Flags::Zero, !bit_value);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, true);

        self.registers.set_program_counter(1);
    }

    pub(crate) fn BIT_u3_HL(&mut self, mmu: &MMU, bit: u3) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let bit_value: bool = (value & (1 << bit)) != 0;

        self.registers.set_flag(Flags::Zero, !bit_value);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, true);

        self.registers.set_program_counter(1);
    }

    pub(crate) fn RES_u3_r8(&mut self, bit: u3, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let result: Register8 = value & !(1 << bit);
        self.registers.set_register_8(target, result);
        self.registers.set_program_counter(1);
    }

    pub(crate) fn RES_u3_HL(&mut self, mmu: &mut MMU, bit: u3) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let result: Register8 = value & !(1 << bit);
        mmu.write_memory(address, result);
        self.registers.set_program_counter(1);
    }

    pub(crate) fn SET_u3_r8(&mut self, bit: u3, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let result: Register8 = value | (1 << bit);
        self.registers.set_register_8(target, result);
        self.registers.set_program_counter(1);
    }

    pub(crate) fn SET_u3_HL(&mut self, mmu: &mut MMU, bit: u3) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        let value: Register8 = mmu.read_memory(address);
        let result: Register8 = value | (1 << bit);
        mmu.write_memory(address, result);
        self.registers.set_program_counter(1);
    }
}
