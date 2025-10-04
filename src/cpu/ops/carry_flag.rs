use crate::cpu::{cpu::CPU, registers::Flags};

impl CPU {
    pub(crate) fn CCF(&mut self) {
        let carry: bool = self.registers.get_flag(Flags::Carry);
        self.registers.set_flag(Flags::Carry, !carry);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_program_counter(1);
    }

    pub(crate) fn SCF(&mut self) {
        self.registers.set_flag(Flags::Carry, true);
        self.registers.set_flag(Flags::Subtraction, false);
        self.registers.set_flag(Flags::HalfCarry, false);
        self.registers.set_program_counter(1);
    }
}
