use crate::{
    common::types::{Address, Byte, Register16},
    cpu::{
        cpu::{CPU, IME},
        registers::{Flags, Registers16},
    },
    mmu::mmu::MMU,
};

impl CPU {
    pub(crate) fn CALL_n16(&mut self, mmu: &mut MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let low: Byte = mmu.read_memory(address_pc);
        let high: Byte = mmu.read_memory(address_pc + 1);
        let address: Address = (((high as u16) << 8) | (low as u16)) as Address;

        let sp: Address = self.registers.get_register_16(Registers16::SP) as Address;
        let pc: Register16 = self.registers.get_program_counter() + 2 as Register16;
        let sp = sp.wrapping_sub(2);
        mmu.write_memory(sp as Address, (pc & 0x00FF) as Byte);
        mmu.write_memory((sp + 1) as Address, (pc >> 8) as Byte);
        self.registers
            .set_register_16(Registers16::SP, sp as Register16);

        self.registers.set_program_counter(address as Register16);
    }

    pub(crate) fn CALL_cc_n16(&mut self, mmu: &mut MMU) {
        let opcode: Byte = mmu.read_memory(self.registers.get_program_counter() as Address - 2);
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let low: Byte = mmu.read_memory(address_pc);
        let high: Byte = mmu.read_memory(address_pc + 1);
        let address: Address = (((high as u16) << 8) | (low as u16)) as Address;

        let condition: bool = match opcode {
            0xC4 => self.registers.is_flag_down(Flags::Zero),
            0xCC => self.registers.is_flag_up(Flags::Zero),
            0xD4 => self.registers.is_flag_down(Flags::Carry),
            0xDC => self.registers.is_flag_up(Flags::Carry),
            _ => panic!("Invalid opcode for CALL_cc_n16"),
        };

        if condition {
            let sp: Address = self.registers.get_register_16(Registers16::SP) as Address;
            let pc: Register16 = self.registers.get_program_counter() + 2 as Register16;
            let sp = sp.wrapping_sub(2);
            mmu.write_memory(sp as Address, (pc & 0x00FF) as Byte);
            mmu.write_memory((sp + 1) as Address, (pc >> 8) as Byte);
            self.registers
                .set_register_16(Registers16::SP, sp as Register16);

            self.registers.set_program_counter(address as Register16);
        } else {
            self.registers.increase_program_counter(2);
        }

        self.registers.set_program_counter(1);
    }

    pub(crate) fn JP_HL(&mut self) {
        let address: Address = self.registers.get_register_16(Registers16::HL) as Address;
        self.registers.set_program_counter(address as Register16);
    }

    pub(crate) fn JP_n16(&mut self, mmu: &MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let low: Byte = mmu.read_memory(address_pc);
        let high: Byte = mmu.read_memory(address_pc + 1);
        let address: Address = (((high as u16) << 8) | (low as u16)) as Address;
        self.registers.set_program_counter(address as Register16);
    }

    pub(crate) fn JP_cc_n16(&mut self, mmu: &MMU) {
        let opcode: Byte = mmu.read_memory(self.registers.get_program_counter() as Address - 2);
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let low: Byte = mmu.read_memory(address_pc);
        let high: Byte = mmu.read_memory(address_pc + 1);
        let address: Address = (((high as u16) << 8) | (low as u16)) as Address;

        let condition: bool = match opcode {
            0xC2 => self.registers.is_flag_down(Flags::Zero),
            0xCA => self.registers.is_flag_up(Flags::Zero),
            0xD2 => self.registers.is_flag_down(Flags::Carry),
            0xDA => self.registers.is_flag_up(Flags::Carry),
            _ => panic!("Invalid opcode for JP_cc_n16"),
        };

        if condition {
            self.registers.set_program_counter(address as Register16);
        } else {
            self.registers.increase_program_counter(2);
        }
    }

    pub(crate) fn JR_n16(&mut self, mmu: &MMU) {
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let offset: i8 = mmu.read_memory(address_pc) as i8;
        let pc: Register16 = self.registers.get_program_counter();
        let address: Address = (pc as i32 + offset as i32 + 1) as Address;
        self.registers.set_program_counter(address as Register16);
    }

    pub(crate) fn JR_cc_n16(&mut self, mmu: &MMU) {
        let opcode: Byte = mmu.read_memory(self.registers.get_program_counter() as Address - 2);
        let address_pc: Address = self.registers.get_program_counter() as Address;
        let offset: i8 = mmu.read_memory(address_pc) as i8;
        let pc: Register16 = self.registers.get_program_counter();

        let condition: bool = match opcode {
            0x20 => self.registers.is_flag_down(Flags::Zero),
            0x28 => self.registers.is_flag_up(Flags::Zero),
            0x30 => self.registers.is_flag_down(Flags::Carry),
            0x38 => self.registers.is_flag_up(Flags::Carry),
            _ => panic!("Invalid opcode for JR_cc_n16"),
        };

        if condition {
            let address: Address = (pc as i32 + offset as i32 + 1) as Address;
            self.registers.set_program_counter(address as Register16);
        } else {
            self.registers.increase_program_counter(1);
        }
    }

    pub(crate) fn RET_cc(&mut self, mmu: &mut MMU) {
        let opcode: Byte = mmu.read_memory(self.registers.get_program_counter() as Address - 2);
        let condition: bool = match opcode {
            0xC0 => self.registers.is_flag_down(Flags::Zero),
            0xC8 => self.registers.is_flag_up(Flags::Zero),
            0xD0 => self.registers.is_flag_down(Flags::Carry),
            0xD8 => self.registers.is_flag_up(Flags::Carry),
            _ => panic!("Invalid opcode for RET_cc"),
        };
        if condition {
            let sp: Address = self.registers.get_register_16(Registers16::SP) as Address;
            let low: Byte = mmu.read_memory(sp);
            let high: Byte = mmu.read_memory(sp + 1);
            let address: Address = (((high as u16) << 8) | (low as u16)) as Address;
            self.registers
                .set_register_16(Registers16::SP, sp.wrapping_add(2) as Register16);
            self.registers.set_program_counter(address as Register16);
        } else {
            self.registers.increase_program_counter(1);
        }
    }

    pub(crate) fn RET(&mut self, mmu: &mut MMU) {
        let sp: Address = self.registers.get_register_16(Registers16::SP) as Address;
        let low: Byte = mmu.read_memory(sp);
        let high: Byte = mmu.read_memory(sp + 1);
        let address: Address = (((high as u16) << 8) | (low as u16)) as Address;
        self.registers
            .set_register_16(Registers16::SP, sp.wrapping_add(2) as Register16);
        self.registers.set_program_counter(address as Register16);
    }

    pub(crate) fn RETI(&mut self, mmu: &mut MMU) {
        let sp: Address = self.registers.get_register_16(Registers16::SP) as Address;
        let low: Byte = mmu.read_memory(sp);
        let high: Byte = mmu.read_memory(sp + 1);
        let address: Address = (((high as u16) << 8) | (low as u16)) as Address;
        self.registers
            .set_register_16(Registers16::SP, sp.wrapping_add(2) as Register16);
        self.registers.set_program_counter(address as Register16);
        self.set_status(IME::Enabled);
    }

    pub(crate) fn RST(&mut self, mmu: &mut MMU) {
        let opcode: Byte = mmu.read_memory(self.registers.get_program_counter() as Address - 1);
        let n: Byte = match opcode {
            0xC7 => 0x00,
            0xCF => 0x08,
            0xD7 => 0x10,
            0xDF => 0x18,
            0xE7 => 0x20,
            0xEF => 0x28,
            0xF7 => 0x30,
            0xFF => 0x38,
            _ => panic!("Invalid opcode for RST"),
        };

        let pc: Register16 = self.registers.get_program_counter();
        let sp: Address = self.registers.get_register_16(Registers16::SP) as Address;

        let sp: Address = sp.wrapping_sub(1);
        mmu.write_memory(sp, (pc >> 8) as Byte);
        let sp: Address = sp.wrapping_sub(1);
        mmu.write_memory(sp, (pc & 0x00FF) as Byte);
        self.registers
            .set_register_16(Registers16::SP, sp as Register16);

        self.registers.set_program_counter(n as Register16);
    }
}
