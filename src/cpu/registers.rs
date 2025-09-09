#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub f: u8,

    pub b: u8,
    pub c: u8,

    pub d: u8,
    pub e: u8,

    pub h: u8,
    pub l: u8,

    pub sp : u16,
    pub pc : u16,
}

// -- getters --
impl Registers {
    // pub const fn get_af(&self) -> u16 {
    //     (self.a as u16) << 8 | self.f as u16
    // }

    pub const fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub const fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub const fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }
}
// -- setters --
impl Registers {
    pub const fn set_bc(&mut self, word: u16) {
        self.b = ((word & 0xFF00) >> 8) as u8;
        self.c = (word & 0x00FF) as u8;
    }

    pub const fn set_de(&mut self, word: u16) {
        self.d = ((word & 0xFF00) >> 8) as u8;
        self.e = (word & 0x00FF) as u8;
    }

    pub const fn set_hl(&mut self, word: u16) {
        self.h = ((word & 0xFF00) >> 8) as u8;
        self.l = (word & 0x00FF) as u8;
    }
}
// -- F reg --
const ZERO_MASK: u8 = 0b1000_0000;
const SUBSTRACT_MASK: u8 = 0b0100_0000;
const HALF_CARRY_MASK: u8 = 0b0010_0000;
const CARRY_MASK: u8 = 0b0001_0000;

impl Registers {
    pub const fn set_zero(&mut self, val: bool) {
        if val {
            self.f |= ZERO_MASK
        } else {
            self.f &= !ZERO_MASK
        }
    }

    pub const fn set_substract(&mut self, val: bool) {
        if val {
            self.f |= SUBSTRACT_MASK
        } else {
            self.f &= !SUBSTRACT_MASK
        }
    }

    pub const fn set_half_carry(&mut self, val: bool) {
        if val {
            self.f |= HALF_CARRY_MASK
        } else {
            self.f &= !HALF_CARRY_MASK
        }
    }

    pub const fn set_carry(&mut self, val: bool) {
        if val {
            self.f |= CARRY_MASK
        } else {
            self.f &= !CARRY_MASK
        }
    }

    // pub const fn get_zero(&self)->bool{
    //     self.f & ZERO_MASK != 0
    // }

    // pub const fn get_substract(&self)->bool{
    //     self.f & SUBSTRACT_MASK != 0
    // }

    // pub const fn get_half_carry(&self)->bool{
    //     self.f & HALF_CARRY_MASK != 0
    // }

    pub const fn get_carry(&self)->bool{
        self.f & CARRY_MASK != 0
    }
}
