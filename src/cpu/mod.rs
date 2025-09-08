use crate::{
    cpu::{instructions::Instruction, registers::Registers},
    mem_bus::MemBus,
};

mod alu;
pub mod instructions;
mod registers;
mod decoder;
pub mod opcode;

#[derive(Debug)]
pub struct Cpu {
    reg: Registers,
    pc : u16,
    mem_bus: MemBus,
}

#[allow(unused)]
impl Cpu {
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Arithmetic(instruction, imm, target) => {
                self.alu(instruction, imm, target)
            }

            _ => unimplemented!(),
        }
    }

    pub fn step(&mut self){
        let instr_byte = self.mem_bus.readb(self.pc);

        if let Some(instruction) = Instruction::try_read(&mut self.pc, &self.mem_bus){
            self.execute(instruction);
        }else{
            panic!("Cannot decode instruction :0x{:x}", instr_byte);
        };

    }
}
