#[derive(Debug)]
pub struct MemBus {}

#[allow(unused)] //removeme
impl MemBus {
    pub fn readb(&self, addr: u16) -> u8 {
        //todo
        0xFF
    }

    pub fn readw(&self, addr: u16) -> u16 {
        //todo
        0xFFFF
    }

    pub fn writeb(&self, addr: u16, byte: u8) {
        //todo
    }

    pub fn writew(&self, addr: u16, word: u16) {
        //todo
    }
}
