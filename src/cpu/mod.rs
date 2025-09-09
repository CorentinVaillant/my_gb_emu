use crate::{
    cpu::{instructions::Instruction, registers::Registers},
    mem_bus::MemBus,
};

mod alu;
pub mod instructions;
mod registers;
mod decoder;
pub mod opcode;
mod jumps;

#[derive(Debug)]
pub struct Cpu {
    reg: Registers,
    mem_bus: MemBus,
}

#[allow(unused)]
impl Cpu {
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Arithmetic(instruction, imm, target) => {
                self.alu(instruction, imm, target)
            }
            Instruction::Jump(instruction,test ,target ) =>{
                self.jump(instruction, test, target) 
            }

            _ => unimplemented!(),
        }
    }

    pub fn step(&mut self){
        let instr_byte = self.mem_bus.readb(self.reg.pc);

        if let Some(instruction) = Instruction::try_read(&mut self.reg, &self.mem_bus){
            self.execute(instruction);
        }else{
            panic!("Cannot decode instruction :0x{:x}", instr_byte);
        };

    }
}
