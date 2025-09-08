use crate::cpu::instructions::Instructions;
use crate::cpu::registers::{Register8, Register16, Registers, Registers8, Registers16};

pub type Opcode = u8;

#[derive(Copy, Clone)]
pub enum Status {
    Running,
    Halted,
    Stopped,
}

pub struct CPU {
    pub registers: Registers,
    status: Status,
}

impl CPU {
    pub fn new() -> Self {
        let _ = std::fs::remove_file("src/log/cpu.log");
        CPU {
            registers: Registers::new(),
            status: Status::Halted,
        }
    }

    pub fn start(&mut self) {
        self.set_status(Status::Running);
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    pub fn set_status(&mut self, status: Status) {
        match status {
            Status::Running => self.status = Status::Running,
            Status::Halted => self.status = Status::Halted,
            Status::Stopped => self.status = Status::Stopped,
        };
    }

    pub fn step(&mut self) {
        let opcode: Opcode = 0; // TODO: Fetch from memory
        let instruction: Instructions = Instructions::get_instruction(opcode);
        self.registers.increase_program_counter(1);
        self.execute(instruction);
    }

    pub fn execute(&mut self, instruction: Instructions) {
        match instruction {
            Instructions::NONE => {}
            Instructions::PREFIX => self.PREFIX(),
            _ => {} // TODO: To remove
        }
    }

    fn PREFIX(&mut self) {
        let opcode: Opcode = 0; // TODO: Fetch from memory
        let instructions: Instructions = Instructions::get_instruction_prefixed(opcode);
        self.registers.increase_program_counter(1);
        self.execute(instructions);
    }

