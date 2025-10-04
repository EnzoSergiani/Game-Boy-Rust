use crate::cpu::cpu::{CPU, IME};

impl CPU {
    pub(crate) fn DI(&mut self) {
        self.set_status(IME::Disabled);
        self.registers.set_program_counter(1);
    }

    pub(crate) fn EI(&mut self) {
        self.enable_ime_next_cycle = true;
        self.registers.set_program_counter(1);
    }

    pub(crate) fn HALT(&mut self) {
        self.halted = true;
        self.registers.set_program_counter(1);
    }
}
