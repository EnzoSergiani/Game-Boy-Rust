use crate::{
    common::types::{Address, Byte, Register8},
    cpu::{
        cpu::{CPU, Instructions},
        registers::{Flags, Registers8},
    },
    mmu::mmu::MMU,
};

impl CPU {
    pub(crate) fn NONE(&mut self) {}

    pub(crate) fn DAA(&mut self) {
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

    pub(crate) fn NOP(&mut self) {
        self.registers.set_program_counter(1);
    }

    pub(crate) fn STOP(&mut self) {
        self.halted = true;
        self.registers.set_program_counter(1);
    }

    pub(crate) fn PREFIX(&mut self, mmu: &mut MMU) {
        let opcode: Byte = mmu.read_memory(self.registers.get_program_counter() as Address);
        let instructions: Instructions = CPU::get_instruction_prefixed(opcode);
        self.registers.increase_program_counter(1);
        self.execute(mmu, instructions);
    }
}
