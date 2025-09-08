pub type Register8 = u8;
pub type Register16 = u16;

pub struct Registers {
    a: Register8,
    b: Register8,
    c: Register8,
    d: Register8,
    e: Register8,
    f: Register8,
    h: Register8,
    l: Register8,
    sp: Register16,
    pc: Register16,
}

#[derive(Debug)]
pub enum Registers8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

pub enum Registers16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

pub enum Flags {
    Zero,
    Subtraction,
    HalfCarry,
    Carry,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
        }
    }

    pub fn set_register_8(&mut self, register: Registers8, value: Register8) {
        match register {
            Registers8::A => self.a = value,
            Registers8::B => self.b = value,
            Registers8::C => self.c = value,
            Registers8::D => self.d = value,
            Registers8::E => self.e = value,
            Registers8::F => self.f = value,
            Registers8::H => self.h = value,
            Registers8::L => self.l = value,
        }
    }

    pub fn get_register_8(&self, register: Registers8) -> Register8 {
        match register {
            Registers8::A => self.a,
            Registers8::B => self.b,
            Registers8::C => self.c,
            Registers8::D => self.d,
            Registers8::E => self.e,
            Registers8::F => self.f,
            Registers8::H => self.h,
            Registers8::L => self.l,
        }
    }

    pub fn set_register_16(&mut self, register: Registers16, value: Register16) {
        match register {
            Registers16::AF => {
                self.a = ((value >> 8) & 0xFF) as Register8;
                self.f = (value & 0xF0) as Register8;
            }
            Registers16::BC => {
                self.b = ((value >> 8) & 0xFF) as Register8;
                self.c = (value & 0xFF) as Register8;
            }
            Registers16::DE => {
                self.d = ((value >> 8) & 0xFF) as Register8;
                self.e = (value & 0xFF) as Register8;
            }
            Registers16::HL => {
                self.h = ((value >> 8) & 0xFF) as Register8;
                self.l = (value & 0xFF) as Register8;
            }
            Registers16::SP => self.sp = value,
            Registers16::PC => self.pc = value,
        }
    }

    pub fn get_register_16(&self, register: Registers16) -> Register16 {
        match register {
            Registers16::AF => (self.a as Register16) << 8 | self.f as Register16,
            Registers16::BC => (self.b as Register16) << 8 | self.c as Register16,
            Registers16::DE => (self.d as Register16) << 8 | self.e as Register16,
            Registers16::HL => (self.h as Register16) << 8 | self.l as Register16,
            Registers16::SP => self.sp,
            Registers16::PC => self.pc,
        }
    }

    pub fn set_stack_pointer(&mut self, value: Register16) {
        self.sp = value;
    }

    pub fn get_stack_pointer(&self) -> Register16 {
        self.sp
    }

    pub fn set_program_counter(&mut self, value: Register16) {
        self.sp = value;
    }

    pub fn get_program_counter(&self) -> Register16 {
        self.pc
    }

    pub fn set_flag(&mut self, flag: Flags) {
        match flag {
            Flags::Zero => self.set_register_8(Registers8::F, self.f | (1 << 7)),
            Flags::Subtraction => self.set_register_8(Registers8::F, self.f | (1 << 6)),
            Flags::HalfCarry => self.set_register_8(Registers8::F, self.f | (1 << 5)),
            Flags::Carry => self.set_register_8(Registers8::F, self.f | (1 << 4)),
        }
    }

    pub fn unset_flag(&mut self, flag: Flags) {
        match flag {
            Flags::Zero => self.set_register_8(Registers8::F, self.f & (1 << 7)),
            Flags::Subtraction => self.set_register_8(Registers8::F, self.f & (1 << 6)),
            Flags::HalfCarry => self.set_register_8(Registers8::F, self.f & (1 << 5)),
            Flags::Carry => self.set_register_8(Registers8::F, self.f & (1 << 4)),
        }
    }

    pub fn get_flag(&self, flag: Flags) -> bool {
        match flag {
            Flags::Zero => (self.get_register_8(Registers8::F) & (1 << 7)) != 0,
            Flags::Subtraction => (self.get_register_8(Registers8::F) & (1 << 6)) != 0,
            Flags::HalfCarry => (self.get_register_8(Registers8::F) & (1 << 5)) != 0,
            Flags::Carry => (self.get_register_8(Registers8::F) & (1 << 4)) != 0,
        }
    }

    pub fn increase_program_counter(&mut self, value: Register16) {
        self.pc = self.pc.wrapping_add(value);
    }
}
