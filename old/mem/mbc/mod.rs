use crate::mem::Mem;


#[derive(Debug)]
pub enum Mbc {
    //todo
}


impl Mem for Mbc {
    fn readb(&self, addr: u16) -> u8 {
        todo!()
    }

    fn readw(&self, addr: u16) -> u16 {
        todo!()
    }

    fn writeb(&mut self, addr: u16, byte: u8) {
        todo!()
    }

    fn writew(&mut self, addr: u16, word: u16) {
        todo!()
    }
}