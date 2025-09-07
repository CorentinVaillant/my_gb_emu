use crate::{
    mem::{cartridge_rom::CartridgeRom, mbc::Mbc},
    utils::bytes_to_word,
};

mod cartridge_rom;
mod io;
mod mbc;

#[derive(Debug)]
pub struct MemBus {
    cartridge_rom: CartridgeRom, //0x0000..=0x3FFF  =>   16 KiB ROM bank 00	From cartridge, usually a fixed bank
    mbc: Mbc, //0x4000..=0x7FFF  =>   16 KiB ROM Bank 01–NN	From cartridge, switchable bank via mapper (if any)
              //0x8000..=0x9FFF  =>   8 KiB Video RAM (VRAM)	In CGB mode, switchable bank 0/1
              //0xA000..=0xBFFF  =>   8 KiB External RAM	From cartridge, switchable bank if any
              //0xC000..=0xCFFF  =>   4 KiB Work RAM (WRAM)
              //0xD000..=0xDFFF  =>   4 KiB Work RAM (WRAM)	In CGB mode, switchable bank 1–7
              //0xE000..=0xFDFF  =>   Echo RAM (mirror of C000–DDFF)	Nintendo says use of this area is prohibited.
              //0xFE00..=0xFE9F  =>   Object attribute memory (OAM)
              //0xFEA0..=0xFEFF  =>   Not Usable	Nintendo says use of this area is prohibited.
              //0xFF00..=0xFF7F  =>   I/O Registers
              //0xFF80..=0xFFFE  =>   High RAM (HRAM)
              //0xFFFF..=0xFFFF  =>   Interrupt Enable register
}

pub trait Mem {
    fn readb(&self, addr: u16, mem_bus : &MemBus) -> u8;
    fn readw(&self, addr: u16, mem_bus : &MemBus) -> u16;

    fn writeb(&mut self, addr: u16, byte: u8 , mem_bus : &mut MemBus );
    fn writew(&mut self, addr: u16, word: u16, mem_bus : &mut MemBus );
}

impl MemBus {
    fn readb(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.cartridge_rom.readb(addr,&self),
            0x4000..=0x7FFF => self.mbc.readb(addr,&self), /*TODO*/
            0x8000..=0x9FFF => 0xFF,                 /*TODO*/
            0xA000..=0xBFFF => 0xFF,                 /*TODO*/
            0xC000..=0xCFFF => 0xFF,                 /*TODO*/
            0xD000..=0xDFFF => 0xFF,                 /*TODO*/
            0xE000..=0xFDFF => 0xFF,                 /*TODO*/
            0xFE00..=0xFE9F => 0xFF,                 /*TODO*/
            0xFEA0..=0xFEFF => 0xFF,                 /*TODO*/
            0xFF00..=0xFF7F => 0xFF,                 /*TODO*/
            0xFF80..=0xFFFE => 0xFF,                 /*TODO*/
            0xFFFF..=0xFFFF => 0xFF,                 /*TODO*/
        }
    }

    fn readw(&self, addr: u16) -> u16 {
        match addr {
            0x0000..0x3FFF => self.cartridge_rom.readw(addr,&self), /*TODO*/
            0x3FFF => bytes_to_word(self.cartridge_rom.readb(0x3FFF,&self), self.mbc.readb(0x4000,&self)),

            0x4000..0x7FFF => 0xFF, /*TODO*/
            0x7FFF => bytes_to_word(self.readb(0x7FFF), self.readb(0x7FFF + 1)),

            0x8000..0x9FFF => 0xFF, /*TODO*/
            0x9FFF => bytes_to_word(self.readb(0x9FFF), self.readb(0x9FFF + 1)),

            0xA000..0xBFFF => 0xFF, /*TODO*/
            0xBFFF => bytes_to_word(self.readb(0xBFFF), self.readb(0xBFFF + 1)),

            0xC000..0xCFFF => 0xFF, /*TODO*/
            0xCFFF => bytes_to_word(self.readb(0xCFFF), self.readb(0xCFFF + 1)),

            0xD000..0xDFFF => 0xFF, /*TODO*/
            0xDFFF => bytes_to_word(self.readb(0xDFFF), self.readb(0xDFFF + 1)),

            0xE000..0xFDFF => 0xFF, /*TODO*/
            0xFDFF => bytes_to_word(self.readb(0xFDFF), self.readb(0xFDFF + 1)),

            0xFE00..0xFE9F => 0xFF, /*TODO*/
            0xFE9F => bytes_to_word(self.readb(0xFE9F), self.readb(0xFE9F + 1)),

            0xFEA0..0xFEFF => 0xFF, /*TODO*/
            0xFEFF => bytes_to_word(self.readb(0xFEFF), self.readb(0xFEFF + 1)),

            0xFF00..0xFF7F => 0xFF, /*TODO*/
            0xFF7F => bytes_to_word(self.readb(0xFF7F), self.readb(0xFF7F + 1)),

            0xFF80..0xFFFE => 0xFF, /*TODO*/
            0xFFFE => bytes_to_word(self.readb(0xFFFE), self.readb(0xFFFE + 1)),

            0xFFFF => 0xFF, /*TODO*/
        }
    }

    fn writeb(&mut self, addr: u16, byte: u8) {
        match addr {
            0x0000..=0x3FFF => self.cartridge_rom.writeb(addr, byte,self),
            0x4000..=0x7FFF => self.mbc.writeb(addr, byte,self),
            0x8000..=0x9FFF => (), /*TODO*/
            0xA000..=0xBFFF => (), /*TODO*/
            0xC000..=0xCFFF => (), /*TODO*/
            0xD000..=0xDFFF => (), /*TODO*/
            0xE000..=0xFDFF => (), /*TODO*/
            0xFE00..=0xFE9F => (), /*TODO*/
            0xFEA0..=0xFEFF => (), /*TODO*/
            0xFF00..=0xFF7F => (), /*TODO*/
            0xFF80..=0xFFFE => (), /*TODO*/
            0xFFFF..=0xFFFF => (), /*TODO*/
        }
    }

    fn writew(&mut self, addr: u16, word: u16) {
        match addr {
            0x0000..=0x3FFF => self.cartridge_rom.writew(addr, word),
            0x4000..=0x7FFF => self.mbc.writew(addr, word),
            0x8000..=0x9FFF => (), /*TODO*/
            0xA000..=0xBFFF => (), /*TODO*/
            0xC000..=0xCFFF => (), /*TODO*/
            0xD000..=0xDFFF => (), /*TODO*/
            0xE000..=0xFDFF => (), /*TODO*/
            0xFE00..=0xFE9F => (), /*TODO*/
            0xFEA0..=0xFEFF => (), /*TODO*/
            0xFF00..=0xFF7F => (), /*TODO*/
            0xFF80..=0xFFFE => (), /*TODO*/
            0xFFFF..=0xFFFF => (), /*TODO*/
        }
    }
}
