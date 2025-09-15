use crate::{cpu::{instructions::Instruction, opcode::Opcode, registers::Registers}, mem_bus::MemBus};

pub mod utils;
mod cpu;
mod mem_bus;
#[cfg(test)]
mod test;

const TETRIS: &[u8] = include_bytes!("./test/hello.gb");

fn main() {
    println!("tetris lenth :0x{:0X}", TETRIS.len());
    let mem_bus = MemBus::from_bytes(TETRIS);
    let mut regs = Registers::zeroed();
    regs.pc = 0x0;

    #[derive(Debug,Clone)]
    enum InstrOpcode {
        Instruction(Instruction),
        Opcode(Vec<(u8, Result<Opcode, cpu::opcode::InvalideOpcode>)>),
    }

    #[derive(Debug,Clone)]
    struct Ligne{
        inst_op : InstrOpcode, 
        nb : u16,
    }

    let mut instructions = Vec::with_capacity(100);
    while regs.pc < 0x4000 {
        let pc_before = regs.pc;
        let instr_opt = Instruction::try_read(&mut regs, &mem_bus);
        if let Some(instr) = instr_opt {
            instructions.push(Ligne{inst_op : InstrOpcode::Instruction(instr), nb : pc_before});
        }else {
            let mut opcodes = vec![];
            for i in pc_before..regs.pc{
                let byte = mem_bus.readb(i);
                opcodes.push((byte, Opcode::try_from(byte)));
            }

            instructions.push(Ligne{inst_op: InstrOpcode::Opcode(opcodes), nb: pc_before});
        }
    }

    for ligne in instructions.iter(){
        match &ligne.inst_op{
            InstrOpcode::Instruction(instruction) => println!("0x{:04X}|\t\t{instruction}", ligne.nb),
            InstrOpcode::Opcode(items) => println!("0x{:04X}|;;{:02X?}", ligne.nb, items),
        }
    }
}
