use crate::{
    common::types::{Address, Byte, Register8, Register16},
    cpu::{
        cpu::CPU,
        registers::{Flags, Registers8, Registers16},
    },
    mmu::mmu::MMU,
};

impl CPU {
    pub(crate) fn ADC_A_r8(&mut self, source: Registers8) {
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

    pub(crate) fn ADC_A_HL(&mut self, mmu: &MMU) {
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

    pub(crate) fn ADC_A_n8(&mut self, mmu: &MMU) {
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

    pub(crate) fn ADD_A_r8(&mut self, source: Registers8) {
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

    pub(crate) fn ADD_A_HL(&mut self, mmu: &MMU) {
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

    pub(crate) fn ADD_A_n8(&mut self, mmu: &MMU) {
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

    pub(crate) fn CP_A_r8(&mut self, source: Registers8) {
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

    pub(crate) fn CP_A_HL(&mut self, mmu: &MMU) {
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

    pub(crate) fn CP_A_n8(&mut self, mmu: &MMU) {
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

    pub(crate) fn DEC_r8(&mut self, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let (result, did_overflow) = value.overflowing_sub(1);
        self.registers.set_register_8(target, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, true);
        self.registers
            .set_flag(Flags::HalfCarry, (value & 0x0F) == 0x00);

        self.registers.set_program_counter(1);
    }

    pub(crate) fn DEC_HL(&mut self, mmu: &mut MMU) {
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

    pub(crate) fn INC_r8(&mut self, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let (result, did_overflow) = value.overflowing_add(1);
        self.registers.set_register_8(target, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers
            .set_flag(Flags::HalfCarry, (value & 0x0F) == 0x0F);

        self.registers.set_program_counter(1);
    }

    pub(crate) fn INC_HL(&mut self, mmu: &mut MMU) {
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

    pub(crate) fn SBC_A_r8(&mut self, source: Registers8) {
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

    pub(crate) fn SBC_A_HL(&mut self, mmu: &MMU) {
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

    pub(crate) fn SBC_A_n8(&mut self, mmu: &MMU) {
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

    pub(crate) fn SUB_A_r8(&mut self, source: Registers8) {
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

    pub(crate) fn SUB_A_HL(&mut self, mmu: &MMU) {
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

    pub(crate) fn SUB_A_n8(&mut self, mmu: &MMU) {
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
    pub(crate) fn ADD_HL_r16(&mut self, source: Registers16) {
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

    pub(crate) fn DEC_r16(&mut self, target: Registers16) {
        let value: Register16 = self.registers.get_register_16(target);
        let (result, did_overflow) = value.overflowing_sub(1);
        self.registers.set_register_16(target, result);
        self.registers.set_program_counter(1);
    }

    pub(crate) fn INC_r16(&mut self, target: Registers16) {
        let value: Register16 = self.registers.get_register_16(target);
        let (result, did_overflow) = value.overflowing_add(1);
        self.registers.set_register_16(target, result);
        self.registers.set_program_counter(1);
    }
}
