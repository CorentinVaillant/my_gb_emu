use crate::{
    cpu::{instructions::Instruction, registers::Registers},
    mem_bus::MemBus,
};

mod alu;
pub mod instructions;
pub mod registers;
mod decoder;
pub mod opcode;
mod jumps;
mod load;
mod errors;
mod stack;
mod misc;

#[derive(Debug)]
pub struct Cpu {
    reg: Registers,
    halted: bool,
    ime:bool,
    low_pow : bool,
    mem_bus: MemBus,
}

#[allow(unused)]
impl Cpu {
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Arithmetic(instruction, imm, target) => {
                self.alu(instruction, imm, target)
                    .expect("Could not execute arithmetic instruction")
            }
            Instruction::Jump(instruction,test ,target ) =>{
                self.jump(instruction, test, target)
                    .expect("Could not execute jump instruction")
            }
            Instruction::Load(target, src) => {
                self.load(target, src)
                    .expect("Could not execute load instruction")
            }
            Instruction::Stack(instr, reg) =>
                self.stack(instr, reg),

            Instruction::Misc(instr) => 
                self.misc(instr),
        }
    }

    pub fn step(&mut self){
        if self.halted {return;}
        if self.ime{
            self.halted = true;
        }
        let instr_byte = self.mem_bus.readb(self.reg.pc);

        if let Some(instruction) = Instruction::try_read(&mut self.reg, &self.mem_bus){
            self.execute(instruction);
        }else{
            panic!("Cannot decode instruction :0x{:x}", instr_byte);
        };

    }
}
