use crate::mem::{Mem};


#[derive(Debug)]
pub struct Joypad{
    select_buttons_read : bool,
    dpad_buttons_read : bool,

    select_buttons : u8, //Start Select B and A buttons
    dpad : u8,
}

impl Joypad{
    pub fn press_start(&mut self)   {self.select_buttons &= !0b1000 }
    pub fn press_select(&mut self)  {self.select_buttons &= !0b0100 }
    pub fn press_b(&mut self)       {self.select_buttons &= !0b0010 }
    pub fn press_a(&mut self)       {self.select_buttons &= !0b0001 }

    pub fn press_dpad_down(&mut self)   {self.dpad &= !0b1000 }
    pub fn press_dpad_up(&mut self)     {self.dpad &= !0b0100 }
    pub fn press_dpad_left(&mut self)   {self.dpad &= !0b0010 }
    pub fn press_dpad_right(&mut self)  {self.dpad &= !0b0001 }


    pub fn release_start(&mut self)   {self.select_buttons |= 0b1000 }
    pub fn release_select(&mut self)  {self.select_buttons |= 0b0100 }
    pub fn release_b(&mut self)       {self.select_buttons |= 0b0010 }
    pub fn release_a(&mut self)       {self.select_buttons |= 0b0001 }

    pub fn release_dpad_down(&mut self)     {self.dpad |= 0b1000 }
    pub fn release_dpad_up(&mut self)       {self.dpad |= 0b0100 }
    pub fn release_dpad_left(&mut self)     {self.dpad |= 0b0010 }
    pub fn release_dpad_right(&mut self)    {self.dpad |= 0b0001 }    
}

impl Default for Joypad {
    fn default() -> Self {
        Joypad {
            select_buttons_read: false,
            dpad_buttons_read: false,

            select_buttons: 0x0F,
            dpad: 0x0F,
        }
    }
}


impl Mem for Joypad{
    fn readb(&self, _addr: u16) -> u8 {
        let mut result = 0xFF; 

        if self.select_buttons_read {
            result &= 0xDF; // clear bit 5
            result &= 0xF0 | self.select_buttons;
        } else if self.dpad_buttons_read {
            result &= 0xEF; // clear bit 4
            result &= 0xF0 | self.dpad;
        }

        result
    }


    fn readw(&self, addr : u16)->u16 {

        self.readb(addr) as u16
    }

    fn writeb(&mut self, _addr : u16,byte : u8 ) {
        self.select_buttons_read = byte & 0b0010_0000 == 0;
        self.dpad_buttons_read   = byte & 0b0001_0000 == 0;

    }

    fn writew(&mut self, addr : u16,word : u16) {
        let write = (word & 0xFF) as u8;
        self.writeb(addr, write)
    }
}

