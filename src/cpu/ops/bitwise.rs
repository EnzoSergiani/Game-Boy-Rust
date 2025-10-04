use crate::{
    common::types::{Address, Register8},
    cpu::{
        cpu::CPU,
        registers::{Flags, Registers8, Registers16},
    },
    mmu::mmu::MMU,
};

impl CPU {
    pub(crate) fn AND_A_r8(&mut self, source: Registers8) {
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

    pub(crate) fn AND_A_HL(&mut self, mmu: &MMU) {
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

    pub(crate) fn AND_A_n8(&mut self, mmu: &MMU) {
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

    pub(crate) fn CPL(&mut self) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let result: Register8 = !a;
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Subtraction, true);
        self.registers.set_flag(Flags::HalfCarry, true);

        self.registers.set_program_counter(1);
    }

    pub(crate) fn OR_A_r8(&mut self, source: Registers8) {
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

    pub(crate) fn OR_A_HL(&mut self, mmu: &MMU) {
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

    pub(crate) fn OR_A_n8(&mut self, mmu: &MMU) {
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

    pub(crate) fn XOR_A_r8(&mut self, source: Registers8) {
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

    pub(crate) fn XOR_A_HL(&mut self, mmu: &MMU) {
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

    pub(crate) fn XOR_A_n8(&mut self, mmu: &MMU) {
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
}
