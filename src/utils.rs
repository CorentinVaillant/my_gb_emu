use crate::cpu::instructions::Instruction;

#[inline(always)]
pub fn panic_illegal_instr(instruction: Instruction) -> ! {
    panic!("Illegal instruction : {instruction:?}")
}

#[inline(always)]
pub fn panic_read_oob() -> ! {
    panic!("Read out ouf bounds")
}

#[inline(always)]
pub fn panic_write_oob() -> ! {
    panic!("Write out ouf bounds")
}

#[inline(always)]
pub const fn bytes_to_word(b1: u8, b2: u8) -> u16 {
    b1 as u16 + ((b2 as u16) << 8)
}

#[inline(always)]
pub const fn word_to_bytes(word: u16) -> (u8, u8) {
    ((word & 0xFF) as u8, ((word & 0xFF00) >> 8) as u8)
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Word(u16),
    Byte(u8),
}

impl Value {
    pub fn overflowing_inc(&mut self) -> bool {
        match self {
            Value::Word(val) => {
                let (res, overflow) = val.overflowing_add(1);
                *val = res;
                overflow
            }
            Value::Byte(val) => {
                let (res, overflow) = val.overflowing_add(1);
                *val = res;
                overflow
            }
        }
    }


    pub fn overflowing_dec(&mut self) -> bool {
        match self {
            Value::Word(val) => {
                let (res, overflow) = val.overflowing_sub(1);
                *val = res;
                overflow
            }
            Value::Byte(val) => {
                let (res, overflow) = val.overflowing_sub(1);
                *val = res;
                overflow
            }
        }
    }

    pub fn is_zero(&self) -> bool{
        match self {
            Value::Word(val) => *val == 0,
            Value::Byte(val) => *val == 0,
        }
    }

    pub fn first_byte(&self) -> u8{
        match self{
            Value::Word(val) => (val & 0xFF) as u8,
            Value::Byte(val) => *val,
        }
    }
}

impl From<Value> for u16{
    fn from(value: Value) -> Self {
        match value{
            Value::Word(val) => val,
            Value::Byte(val) => val as u16,
        }
    }
}

//MARK: TEST

#[cfg(test)]
mod test {
    use crate::utils::{bytes_to_word, word_to_bytes};

    #[test]
    pub fn test_word_byte_convertion() {
        for word in 0x0000..=0xFFFF {
            let bytes = word_to_bytes(word);

            assert_eq!(word, bytes_to_word(bytes.0, bytes.1))
        }
    }
}
