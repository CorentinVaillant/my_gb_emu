pub mod joypad;
mod serial_transfer;

use crate::mem::{io::{joypad::Joypad, serial_transfer::SerialTransfer}, Mem};


pub const JOYPAD_ADDR:u16 = 0xFF00;
pub const SERIAL_TRANS_BEG: u16 = 0xFF01;
pub const SERIAL_TRANS_END: u16 = 0xFF02;

#[derive(Debug)]
pub struct IO{
    pub joypad : Joypad,
    serial_trans : SerialTransfer,
}


impl Mem for IO{
    fn readb(&self, addr : u16)->u8 {
        match addr {
            JOYPAD_ADDR => self.joypad.readb(addr),
            SERIAL_TRANS_BEG..=SERIAL_TRANS_END => self.serial_trans.readb(addr),
            0xFF00..=0xFFFF => unimplemented!(),
            _ => panic!("Memory out of bounds")
        }
    }

    fn readw(&self, addr: u16) -> u16 {
        match addr {
            JOYPAD_ADDR => self.joypad.readw(addr),
            SERIAL_TRANS_BEG..=SERIAL_TRANS_END => self.serial_trans.readw(addr),

            0xFF00..=0xFFFF => unimplemented!(),
            _ => panic!("memory read out of bounds"),
        }
    }

    fn writeb(&mut self, addr : u16,byte : u8 ) {
        match addr {
            JOYPAD_ADDR => self.joypad.writeb(addr,byte),
            0xFF00..=0xFFFF => unimplemented!(),
            _ => panic!("memory read out of bounds")
        }
    }

    fn writew(&mut self, addr: u16, word: u16) {
        match addr {
            JOYPAD_ADDR => self.joypad.writew(addr, word),
            SERIAL_TRANS_BEG..=SERIAL_TRANS_END => self.serial_trans.writew(addr, word),
            0xFF00..=0xFFFF => unimplemented!(),
            _ => panic!("memory read out of bounds")
        }
    }

}