use crate::{
    common::types::Byte,
    cpu::{
        cpu::CPU,
        registers::{Flags, Registers8, Registers16},
    },
};

use chrono::{Local, Timelike};
use std::{fs::OpenOptions, io::Write};

impl CPU {
    pub fn write_log(&mut self, opcode: Byte) {
        let now: chrono::DateTime<Local> = Local::now();
        let log_time: String = format!(
            "Time: {:02}:{:02}:{:02}:{:03}\n",
            now.hour(),
            now.minute(),
            now.second(),
            now.timestamp_subsec_millis()
        );
        let log_opcode: String = format!("Next opcode: 0x{:02X}", opcode);
        let log_message_registers_8: String = format!(
            "A: {:02X}, B: {:02X}, C: {:02X}, D: {:02X}, E: {:02X}, F: {:02X}, H: {:02X}, L: {:02X}",
            self.registers.get_register_8(Registers8::A),
            self.registers.get_register_8(Registers8::B),
            self.registers.get_register_8(Registers8::C),
            self.registers.get_register_8(Registers8::D),
            self.registers.get_register_8(Registers8::E),
            self.registers.get_register_8(Registers8::F),
            self.registers.get_register_8(Registers8::H),
            self.registers.get_register_8(Registers8::L)
        );
        let log_message_registers_16: String = format!(
            "AF: {:04X}, BC: {:04X}, DE: {:04X}, HL: {:04X}, SP: {:04X}, PC: {:04X}",
            self.registers.get_register_16(Registers16::AF),
            self.registers.get_register_16(Registers16::BC),
            self.registers.get_register_16(Registers16::DE),
            self.registers.get_register_16(Registers16::HL),
            self.registers.get_stack_pointer(),
            self.registers.get_program_counter()
        );
        let log_message_flags: String = format!(
            "z: {}, n: {}, h: {}, c: {}",
            self.registers.get_flag(Flags::Zero) as u8,
            self.registers.get_flag(Flags::Subtraction) as u8,
            self.registers.get_flag(Flags::HalfCarry) as u8,
            self.registers.get_flag(Flags::Carry) as u8
        );

        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("src/log/cpu.log")
        {
            let _ = file.write_all(b"*--------------------------------------------------------*\n");
            let _ = file.write_all(log_time.as_bytes());
            let _ = file.write_all(b"\n");
            let _ = file.write_all(log_opcode.as_bytes());
            let _ = file.write_all(b"\n");
            let _ = file.write_all(log_message_registers_8.as_bytes());
            let _ = file.write_all(b"\n");
            let _ = file.write_all(log_message_registers_16.as_bytes());
            let _ = file.write_all(b"\n");
            let _ = file.write_all(log_message_flags.as_bytes());
            let _ = file.write_all(b"\n");
        }
    }
}
