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
