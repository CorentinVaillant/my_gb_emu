use crate::{cpu::{errors::IllegalInstructionErr, instructions::{ByteLoadDest, Instruction, LoadSrc, LoadDest, WordLoadDest}, Cpu}, utils::Value};


impl Cpu{
    pub(super) fn load(&mut self, target: LoadDest, src: LoadSrc)->Result<(), IllegalInstructionErr>{
        match target{
            LoadDest::ByteDest(b_target) => {
                let value = self.get_src_value(src, false);
                match value{
                    Value::Byte(byte) => self.write_byte_target(b_target, byte),
                    Value::Word(_) => Err(Instruction::Load(target, src))?,
                }
            },
            LoadDest::WordDest(w_target) => {
                let value = self.get_src_value(src, true);
                match value{
                    Value::Word(word) => self.write_word_target(w_target, word),
                    Value::Byte(_) => Err(Instruction::Load(target, src))?,
                }
            },
        }

        Ok(())
    }

    fn write_byte_target(&mut self, target: ByteLoadDest, byte:u8){
        match target {
            ByteLoadDest::A => self.reg.a = byte,
            ByteLoadDest::B => self.reg.b = byte,
            ByteLoadDest::C => self.reg.c = byte,
            ByteLoadDest::D => self.reg.d = byte,
            ByteLoadDest::E => self.reg.e = byte,
            ByteLoadDest::H => self.reg.h = byte,
            ByteLoadDest::L => self.reg.l = byte,

            ByteLoadDest::AddrC => self.mem_bus.writeb(0xFF00 + (self.reg.c as u16), byte),
            ByteLoadDest::AddrBC => self.mem_bus.writeb(self.reg.get_bc(), byte),
            ByteLoadDest::AddrDE => self.mem_bus.writeb(self.reg.get_de(), byte),
            ByteLoadDest::AddrHL => self.mem_bus.writeb(self.reg.get_hl(), byte),
            ByteLoadDest::AddrSP => self.mem_bus.writeb(self.reg.sp, byte),
            ByteLoadDest::AddrHLadd => {self.mem_bus.writeb(self.reg.get_hl(), byte); self.reg.set_hl(self.reg.get_hl().wrapping_add(1));},
            ByteLoadDest::AddrHLsub => {self.mem_bus.writeb(self.reg.get_hl(), byte); self.reg.set_hl(self.reg.get_hl().wrapping_sub(1));}
            ByteLoadDest::AddrImm(addr) => self.mem_bus.writeb(addr, byte),
        }
    }

    fn write_word_target(&mut self, target: WordLoadDest, word:u16){
        match target {
            WordLoadDest::BC => self.reg.set_bc(word),
            WordLoadDest::DE => self.reg.set_de(word),
            WordLoadDest::HL => self.reg.set_hl(word),

            WordLoadDest::SP => self.reg.sp = word,
            
            WordLoadDest::AddrC => self.mem_bus.writew(0xFF00 + (self.reg.c as u16), word),
            WordLoadDest::AddrBC => self.mem_bus.writew(self.reg.get_bc(), word),
            WordLoadDest::AddrDE => self.mem_bus.writew(self.reg.get_de(), word),
            WordLoadDest::AddrHL => self.mem_bus.writew(self.reg.get_hl(), word),
            WordLoadDest::AddrSP => self.mem_bus.writew(self.reg.sp, word),
            WordLoadDest::AddrHLadd => {self.mem_bus.writew(self.reg.get_hl(), word); self.reg.set_hl(self.reg.get_hl().wrapping_add(1));},
            WordLoadDest::AddrHLsub => {self.mem_bus.writew(self.reg.get_hl(), word); self.reg.set_hl(self.reg.get_hl().wrapping_sub(1));}
            WordLoadDest::AddrImm(addr) => self.mem_bus.writew(addr, word),
            
        }
    }

    fn get_src_value(&mut self, src: LoadSrc, load_word_from_addr: bool) -> Value {
        match src {
            LoadSrc::A => Value::Byte(self.reg.a),
            LoadSrc::B => Value::Byte(self.reg.b),
            LoadSrc::C => Value::Byte(self.reg.c),
            LoadSrc::D => Value::Byte(self.reg.d),
            LoadSrc::E => Value::Byte(self.reg.e),
            LoadSrc::H => Value::Byte(self.reg.h),
            LoadSrc::L => Value::Byte(self.reg.l),

            LoadSrc::HL => Value::Word(self.reg.get_hl()),
            LoadSrc::SP => Value::Word(self.reg.sp),

            LoadSrc::Imm8(val) => Value::Byte(val),
            LoadSrc::Imm16(val) => Value::Word(val),

            LoadSrc::AddrC => {
                let addr = self.reg.c as u16 + 0xFF00;
                if load_word_from_addr{
                    Value::Word(self.mem_bus.readw(addr))
                }else{
                    Value::Byte(self.mem_bus.readb(addr))
                }
            }
            LoadSrc::AddrBC => {
                if load_word_from_addr{
                    Value::Word(self.mem_bus.readw(self.reg.get_bc()))
                }else{
                    Value::Byte(self.mem_bus.readb(self.reg.get_bc()))
                }
            },
            LoadSrc::AddrDE => {
                if load_word_from_addr{
                    Value::Word(self.mem_bus.readw(self.reg.get_de()))
                }else{
                    Value::Byte(self.mem_bus.readb(self.reg.get_de()))
                }
            },
            LoadSrc::AddrHL => {
                if load_word_from_addr{
                    Value::Word(self.mem_bus.readw(self.reg.get_hl()))
                }else{
                    Value::Byte(self.mem_bus.readb(self.reg.get_hl()))
                }
            },

            LoadSrc::AddrSP => {
                if load_word_from_addr{
                    Value::Word(self.mem_bus.readw(self.reg.sp))
                }else{
                    Value::Byte(self.mem_bus.readb(self.reg.sp))
                }
            },
            LoadSrc::AddrHLadd => {
                let read = if load_word_from_addr{
                    Value::Word(self.mem_bus.readw(self.reg.get_hl()))
                }else{
                    Value::Byte(self.mem_bus.readb(self.reg.get_hl()))
                };
                self.reg.set_hl(self.reg.get_hl().wrapping_add(1));
                read
            },
            LoadSrc::AddrHLsub => {
                let read = if load_word_from_addr{
                    Value::Word(self.mem_bus.readw(self.reg.get_hl()))
                }else{
                    Value::Byte(self.mem_bus.readb(self.reg.get_hl()))
                };
                self.reg.set_hl(self.reg.get_hl().wrapping_sub(1));
                read
            },
            LoadSrc::AddrImm(val) => {
                if load_word_from_addr{
                    Value::Word(self.mem_bus.readw(val))
                }else{
                    Value::Byte(self.mem_bus.readb(val))
                }
            },
        }
    }
}


