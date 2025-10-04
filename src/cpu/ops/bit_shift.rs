use crate::{
    common::types::{Address, Byte, Register8, Register16},
    cpu::{
        cpu::CPU,
        registers::{Flags, Registers8, Registers16},
    },
    mmu::mmu::MMU,
};

impl CPU {
    pub(crate) fn RL_r8(&mut self, target: Registers8) {
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

    pub(crate) fn RL_HL(&mut self, mmu: &mut MMU) {
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

    pub(crate) fn RLA(&mut self) {
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

    pub(crate) fn RLC_r8(&mut self, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let result: Register8 = (value << 1) | (value >> 7);
        self.registers.set_register_8(target, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (value & 0x80) != 0);

        self.registers.set_program_counter(1);
    }

    pub(crate) fn RLC_HL(&mut self, mmu: &mut MMU) {
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

    pub(crate) fn RLCA(&mut self) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let result: Register8 = (a << 1) | (a >> 7);
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, false);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (a & 0x80) != 0);

        self.registers.set_program_counter(1);
    }

    pub(crate) fn RR_r8(&mut self, target: Registers8) {
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

    pub(crate) fn RR_HL(&mut self, mmu: &mut MMU) {
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

    pub(crate) fn RRA(&mut self) {
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

    pub(crate) fn RRC_r8(&mut self, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let result: Register8 = (value >> 1) | (value << 7);
        self.registers.set_register_8(target, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (value & 0x01) != 0);

        self.registers.set_program_counter(1);
    }

    pub(crate) fn RRC_HL(&mut self, mmu: &mut MMU) {
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

    pub(crate) fn RRCA(&mut self) {
        let a: Register8 = self.registers.get_register_8(Registers8::A);
        let result: Register8 = (a >> 1) | (a << 7);
        self.registers.set_register_8(Registers8::A, result);

        self.registers.set_flag(Flags::Zero, false);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (a & 0x01) != 0);

        self.registers.set_program_counter(1);
    }

    pub(crate) fn SLA_r8(&mut self, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let result: Register8 = value << 1;
        self.registers.set_register_8(target, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (value & 0x80) != 0);

        self.registers.set_program_counter(1);
    }

    pub(crate) fn SLA_HL(&mut self, mmu: &mut MMU) {
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

    pub(crate) fn SRA_r8(&mut self, target: Registers8) {
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

    pub(crate) fn SRA_HL(&mut self, mmu: &mut MMU) {
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

    pub(crate) fn SRL_r8(&mut self, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let result: Register8 = value >> 1;
        self.registers.set_register_8(target, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, (value & 0x01) != 0);

        self.registers.set_program_counter(1);
    }

    pub(crate) fn SRL_HL(&mut self, mmu: &mut MMU) {
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

    pub(crate) fn SWAP_r8(&mut self, target: Registers8) {
        let value: Register8 = self.registers.get_register_8(target);
        let result: Register8 = (value << 4) | (value >> 4);
        self.registers.set_register_8(target, result);

        self.registers.set_flag(Flags::Zero, result == 0);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_flag(Flags::Carry, false);

        self.registers.set_program_counter(1);
    }

    pub(crate) fn SWAP_HL(&mut self, mmu: &mut MMU) {
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
}
