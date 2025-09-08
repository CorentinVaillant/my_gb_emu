use crate::{
    cpu::{
        instructions::{ArithmeticInstruction, ArithmeticTarget, Immediate, Instruction},
        opcode::Opcode,
    },
    mem_bus::MemBus,
    utils::Value,
};

impl Instruction {
    ///Read the instruction point by pc, and increment
    pub fn try_read(pc: &mut u16, mem_bus: &MemBus) -> Option<Instruction> {
        let byte = mem_bus.readb(*pc);
        *pc += 1;

        let value = Value::Byte(byte);

        let opcode = Opcode::try_from(value);

        if let Ok(opcode) = opcode {
            Some(match opcode {
                // Arithmetics
	            Opcode::ADDAA  => Instruction::Arithmetic(ArithmeticInstruction::ADD, None, Some(ArithmeticTarget::A)),
	            Opcode::ADDAB  => Instruction::Arithmetic(ArithmeticInstruction::ADD, None, Some(ArithmeticTarget::B)),
	            Opcode::ADDAC  => Instruction::Arithmetic(ArithmeticInstruction::ADD, None, Some(ArithmeticTarget::C)),
	            Opcode::ADDAD  => Instruction::Arithmetic(ArithmeticInstruction::ADD, None, Some(ArithmeticTarget::D)),
	            Opcode::ADDAE  => Instruction::Arithmetic(ArithmeticInstruction::ADD, None, Some(ArithmeticTarget::E)),
	            Opcode::ADDAH  => Instruction::Arithmetic(ArithmeticInstruction::ADD, None, Some(ArithmeticTarget::H)),
	            Opcode::ADDAL  => Instruction::Arithmetic(ArithmeticInstruction::ADD, None, Some(ArithmeticTarget::L)),
	            Opcode::ADDAHL => Instruction::Arithmetic(ArithmeticInstruction::ADD, None, Some(ArithmeticTarget::HL)),
                Opcode::ADDAn8 => Instruction::Arithmetic(ArithmeticInstruction::ADD, Some(Immediate::N8(read_next_byte(pc, mem_bus))), None),
                // ? ADD A [HL]





                _ => unimplemented!(),
            })
        } else {
            None
        }
    }

}

fn read_next_byte(pc: &mut u16, mem_bus: &MemBus) -> u8{
    let byte = mem_bus.readb(*pc);
    *pc += 1;
    byte
}