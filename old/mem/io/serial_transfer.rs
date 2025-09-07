use crate::mem::Mem;

//TODO impl

#[derive(Debug)]
pub struct SerialTransfer;

impl Mem for SerialTransfer {
    fn readb(&self, _: u16) -> u8 {
        0xF
    }

    fn readw(&self, _: u16) -> u16 {
        0xF
    }

    fn writeb(&mut self, _: u16, _: u8) {}

    fn writew(&mut self, _: u16, _: u16) {}
}
