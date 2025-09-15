#[derive(Debug)]
pub struct MemBus {
    rom : [u8; 0x4000], // 0x0000 -> 0x3FFF
    //[...]
    if_flag: u8, // 0xFF0F
    ie_flag: u8, // 0xFFFF
}

impl MemBus {
    pub fn from_bytes(rom: &[u8])->Self{
        let rom = core::array::from_fn(|i|*rom.get(i).unwrap_or(&0));

        Self { rom, if_flag: 0, ie_flag: 0, }
    }
}

impl MemBus {
    pub fn readb(&self, addr: u16) -> u8 {
        match addr{
            0x0000..0x4000 => self.rom[addr as usize],

            0xFF0F => self.if_flag,
            0xFFFF => self.ie_flag,

            _ => 0xFF,
        }
    }

    pub fn readw(&self, addr: u16) -> u16 {
        match addr{
            0x0000..0x3FFF => (self.rom[addr as usize] as u16) | ((self.rom[addr as usize + 1] as u16)<< 8),
            0x3FFF => self.rom[addr as usize] as u16,

            0xFF0F => self.if_flag as u16,
            0xFFFF => self.ie_flag as u16,

            _ => 0xFFFF,
        }
    }

    pub fn writeb(&self, _addr: u16, _byte: u8) {
        //todo
    }

    pub fn writew(&self, _addr: u16, _word: u16) {
        //todo
    }
}
