use crate::{cpu::{instructions::{JumpInstruction, JumpTarget, JumpTest}, Cpu}, utils::panic_illegal_instr};

impl Cpu {
    pub fn jump(&mut self, instruction :JumpInstruction,test: JumpTest,opt_target: Option<JumpTarget>){
        if !self.jump_test(test){return;}
        if let Some(target) = opt_target{
            match (instruction, target){
                //Call
                (JumpInstruction::Call , JumpTarget::Imm16(target)) =>
                    self.call(target),
                //Jp
                (JumpInstruction::Jp , JumpTarget::Imm16(target)) =>
                    self.jp(target),
                (JumpInstruction::Jp , JumpTarget::HL) =>
                    self.jp(self.reg.get_hl()),
                //Jr
                (JumpInstruction::Jr , JumpTarget::ImmS8(offset)) =>
                    self.jr(offset),

                _=>panic_illegal_instr(super::instructions::Instruction::Jump(instruction, test, opt_target))
            }
        }else{
            match instruction{

                //Ret
                JumpInstruction::Ret => 
                    self.ret(),
                JumpInstruction::RetI => 
                    self.reti(),
                _=>panic_illegal_instr(super::instructions::Instruction::Jump(instruction, test, opt_target))
            }

        }
    }

    fn jump_test(&self, test: JumpTest) -> bool{
        match test {
            JumpTest::NotZero => !self.reg.get_zero(),
            JumpTest::Zero => self.reg.get_zero(),
            JumpTest::NotCarry => !self.reg.get_carry(),
            JumpTest::Carry => self.reg.get_carry(),
            JumpTest::Always => true,
        }
    }

    fn call(&mut self, addr: u16){
        self.mem_bus.writew(self.reg.sp, self.reg.pc);
        self.reg.sp = self.reg.sp.wrapping_add(2);
        self.reg.pc = addr;
    }

    fn jp(&mut self, addr: u16){
        self.reg.pc = addr;
    }

    fn jr(&mut self, offset: i8){
        self.reg.pc = self.reg.pc.wrapping_add_signed(offset as i16);
    }

    fn ret(&mut self){
        self.reg.pc = self.mem_bus.readw(self.reg.sp);
        self.reg.pc = self.reg.pc.wrapping_add(2);
    }

    fn reti(&mut self){
        todo!()
    }
}