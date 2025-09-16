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
    pub reg: Registers,
    pub halted: bool,
    pub ime:bool,
    pub low_pow : bool,
    pub mem_bus: MemBus,
}

#[allow(unused)]
impl Cpu {
    pub fn new(mem:MemBus)->Self {
        let reg = Registers::zeroed();
        Self { reg, halted: false, ime: false, low_pow: false, mem_bus: mem }
    }

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


    pub fn step_verbose(&mut self) -> u16{
        if self.halted {return self.reg.pc;}
        if self.ime{
            self.halted = true;
        }
        let instr_byte = self.mem_bus.readb(self.reg.pc);

        if let Some(instruction) = Instruction::try_read(&mut self.reg, &self.mem_bus){
            println!("read : {instr_byte} => {instruction}");
            self.execute(instruction);
        }else{
            panic!("Cannot decode instruction :0x{:x}", instr_byte);
        };

        return self.reg.pc;

    }

    
}
