use crate::{mem::Mem, utils::panic_read_oob};

const CARTRIDGE_ROM_SIZE: usize = 0x4000;

#[derive(Debug)]
pub struct CartridgeRom {
    mem: [u8; CARTRIDGE_ROM_SIZE],
}

impl Mem for CartridgeRom {
    fn readb(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.mem[addr as usize],
            _ => panic_read_oob(),
        }
    }

    fn readw(&self, addr: u16) -> u16 {
        match addr {
            0x0000..0x3FFF => {
                self.mem[addr as usize] as u16 + ((self.mem[addr as usize + 1] as u16) << 8)
            }
            _ => panic_read_oob(),
        }
    }

    fn writeb(&mut self, _: u16, _: u8) {}

    fn writew(&mut self, _: u16, _: u16) {}
}
