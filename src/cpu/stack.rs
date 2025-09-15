use crate::cpu::{instructions::{StackInstruction, StackReg16}, Cpu};


impl Cpu{
    pub fn stack(&mut self, instruction: StackInstruction, reg: StackReg16) {
        match instruction {
            StackInstruction::Push => self.push(reg),
            StackInstruction::Pop => self.pop(reg),
        }
    }

    fn push(&mut self, reg: StackReg16){
        self.reg.sp = self.reg.sp.wrapping_sub(2);
        self.mem_bus.writew(self.reg.sp, self.get_reg_value(reg));
    }

    fn pop(&mut self, reg: StackReg16){
        let value = self.mem_bus.readw(self.reg.sp);
        self.set_reg(reg, value);
        self.reg.sp = self.reg.sp.wrapping_add(2);
    }

    fn get_reg_value(&self, reg: StackReg16) -> u16{
        match reg {
            StackReg16::BC => self.reg.get_bc(),
            StackReg16::DE => self.reg.get_de(),
            StackReg16::HL => self.reg.get_hl(),
            StackReg16::AF => self.reg.get_af(),
        }
    }

    fn set_reg(&mut self, reg:StackReg16, word: u16){
        match reg {
            StackReg16::BC => self.reg.set_bc(word),
            StackReg16::DE => self.reg.set_de(word),
            StackReg16::HL => self.reg.set_hl(word),
            StackReg16::AF => self.reg.set_af(word),
        }
    }
}