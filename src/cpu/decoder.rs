use crate::{
    cpu::{
        instructions::{ArithmeticInstruction, ArithmeticTarget, Immediate, Instruction, JumpInstruction, JumpTarget, JumpTest},
        opcode::Opcode, registers::Registers,
    },
    mem_bus::MemBus,
};

impl Instruction {
    ///Read the instruction point by pc, and increment
    pub fn try_read(reg : &mut Registers, mem_bus: &MemBus) -> Option<Instruction> {
        let byte = mem_bus.readb(reg.pc);
        reg.pc = reg.pc.wrapping_add(1);

        let opcode = Opcode::try_from(byte);

        if let Ok(opcode) = opcode {
            Some(match opcode {
                // MARK: ALU INSTRUCTIONS
                //--> 8 bits Arithmetics
                //Add
	            Opcode::AddAA  => Instruction::Arithmetic(ArithmeticInstruction::ADD, None, Some(ArithmeticTarget::A)),
	            Opcode::AddAB  => Instruction::Arithmetic(ArithmeticInstruction::ADD, None, Some(ArithmeticTarget::B)),
	            Opcode::AddAC  => Instruction::Arithmetic(ArithmeticInstruction::ADD, None, Some(ArithmeticTarget::C)),
	            Opcode::AddAD  => Instruction::Arithmetic(ArithmeticInstruction::ADD, None, Some(ArithmeticTarget::D)),
	            Opcode::AddAE  => Instruction::Arithmetic(ArithmeticInstruction::ADD, None, Some(ArithmeticTarget::E)),
	            Opcode::AddAH  => Instruction::Arithmetic(ArithmeticInstruction::ADD, None, Some(ArithmeticTarget::H)),
	            Opcode::AddAL  => Instruction::Arithmetic(ArithmeticInstruction::ADD, None, Some(ArithmeticTarget::L)),
                Opcode::AddAN8 => Instruction::Arithmetic(ArithmeticInstruction::ADD, Some(Immediate::N8(read_next_byte(&mut reg.pc, mem_bus))), None),
                Opcode::AddAAddrHL => Instruction::Arithmetic(ArithmeticInstruction::ADD, None, Some(ArithmeticTarget::HlAddr)),
                //Adc
                Opcode::AdcAB  => Instruction::Arithmetic(ArithmeticInstruction::ADC, None, Some(ArithmeticTarget::B)),
                Opcode::AdcAA  => Instruction::Arithmetic(ArithmeticInstruction::ADC, None, Some(ArithmeticTarget::A)),
                Opcode::AdcAC  => Instruction::Arithmetic(ArithmeticInstruction::ADC, None, Some(ArithmeticTarget::C)),
                Opcode::AdcAD  => Instruction::Arithmetic(ArithmeticInstruction::ADC, None, Some(ArithmeticTarget::D)),
                Opcode::AdcAE  => Instruction::Arithmetic(ArithmeticInstruction::ADC, None, Some(ArithmeticTarget::E)),
                Opcode::AdcAH  => Instruction::Arithmetic(ArithmeticInstruction::ADC, None, Some(ArithmeticTarget::H)),
                Opcode::AdcAL  => Instruction::Arithmetic(ArithmeticInstruction::ADC, None, Some(ArithmeticTarget::L)),
                Opcode::AdcAN8 => Instruction::Arithmetic(ArithmeticInstruction::ADC, Some(Immediate::N8(read_next_byte(&mut reg.pc, mem_bus))), None),
                Opcode::AdcAAddrHL => Instruction::Arithmetic(ArithmeticInstruction::ADC, None, Some(ArithmeticTarget::HlAddr)),
                //Sub
                Opcode::SubAB  => Instruction::Arithmetic(ArithmeticInstruction::SUB, None, Some(ArithmeticTarget::B)),
                Opcode::SubAA  => Instruction::Arithmetic(ArithmeticInstruction::SUB, None, Some(ArithmeticTarget::A)),
                Opcode::SubAC  => Instruction::Arithmetic(ArithmeticInstruction::SUB, None, Some(ArithmeticTarget::C)),
                Opcode::SubAD  => Instruction::Arithmetic(ArithmeticInstruction::SUB, None, Some(ArithmeticTarget::D)),
                Opcode::SubAE  => Instruction::Arithmetic(ArithmeticInstruction::SUB, None, Some(ArithmeticTarget::E)),
                Opcode::SubAH  => Instruction::Arithmetic(ArithmeticInstruction::SUB, None, Some(ArithmeticTarget::H)),
                Opcode::SubAL  => Instruction::Arithmetic(ArithmeticInstruction::SUB, None, Some(ArithmeticTarget::L)),
                Opcode::SubAN8 => Instruction::Arithmetic(ArithmeticInstruction::SUB, Some(Immediate::N8(read_next_byte(&mut reg.pc, mem_bus))), None),
                Opcode::SubAAddrHL => Instruction::Arithmetic(ArithmeticInstruction::SUB, None, Some(ArithmeticTarget::HlAddr)),
                //Sbc
                Opcode::SbcAB  => Instruction::Arithmetic(ArithmeticInstruction::SBC, None, Some(ArithmeticTarget::B)),
                Opcode::SbcAA  => Instruction::Arithmetic(ArithmeticInstruction::SBC, None, Some(ArithmeticTarget::A)),
                Opcode::SbcAC  => Instruction::Arithmetic(ArithmeticInstruction::SBC, None, Some(ArithmeticTarget::C)),
                Opcode::SbcAD  => Instruction::Arithmetic(ArithmeticInstruction::SBC, None, Some(ArithmeticTarget::D)),
                Opcode::SbcAE  => Instruction::Arithmetic(ArithmeticInstruction::SBC, None, Some(ArithmeticTarget::E)),
                Opcode::SbcAH  => Instruction::Arithmetic(ArithmeticInstruction::SBC, None, Some(ArithmeticTarget::H)),
                Opcode::SbcAL  => Instruction::Arithmetic(ArithmeticInstruction::SBC, None, Some(ArithmeticTarget::L)),
                Opcode::SbcAN8 => Instruction::Arithmetic(ArithmeticInstruction::SBC, Some(Immediate::N8(read_next_byte(&mut reg.pc, mem_bus))), None),
                Opcode::SbcAAddrHL => Instruction::Arithmetic(ArithmeticInstruction::SBC, None, Some(ArithmeticTarget::HlAddr)),
                //Cp
                Opcode::CpAB  => Instruction::Arithmetic(ArithmeticInstruction::CP, None, Some(ArithmeticTarget::B)),
                Opcode::CpAA  => Instruction::Arithmetic(ArithmeticInstruction::CP, None, Some(ArithmeticTarget::A)),
                Opcode::CpAC  => Instruction::Arithmetic(ArithmeticInstruction::CP, None, Some(ArithmeticTarget::C)),
                Opcode::CpAD  => Instruction::Arithmetic(ArithmeticInstruction::CP, None, Some(ArithmeticTarget::D)),
                Opcode::CpAE  => Instruction::Arithmetic(ArithmeticInstruction::CP, None, Some(ArithmeticTarget::E)),
                Opcode::CpAH  => Instruction::Arithmetic(ArithmeticInstruction::CP, None, Some(ArithmeticTarget::H)),
                Opcode::CpAL  => Instruction::Arithmetic(ArithmeticInstruction::CP, None, Some(ArithmeticTarget::L)),
                Opcode::CpAN8 => Instruction::Arithmetic(ArithmeticInstruction::CP, Some(Immediate::N8(read_next_byte(&mut reg.pc, mem_bus))), None),
                Opcode::CpAAddrHL => Instruction::Arithmetic(ArithmeticInstruction::CP, None, Some(ArithmeticTarget::HlAddr)),
                //Inc
                Opcode::IncA => Instruction::Arithmetic(ArithmeticInstruction::INC, None, Some(ArithmeticTarget::A)),
                Opcode::IncB => Instruction::Arithmetic(ArithmeticInstruction::INC, None, Some(ArithmeticTarget::B)),
                Opcode::IncC => Instruction::Arithmetic(ArithmeticInstruction::INC, None, Some(ArithmeticTarget::C)),
                Opcode::IncD => Instruction::Arithmetic(ArithmeticInstruction::INC, None, Some(ArithmeticTarget::D)),
                Opcode::IncE => Instruction::Arithmetic(ArithmeticInstruction::INC, None, Some(ArithmeticTarget::E)),
                Opcode::IncH => Instruction::Arithmetic(ArithmeticInstruction::INC, None, Some(ArithmeticTarget::H)),
                Opcode::IncL => Instruction::Arithmetic(ArithmeticInstruction::INC, None, Some(ArithmeticTarget::L)),
                Opcode::IncAddrHL => Instruction::Arithmetic(ArithmeticInstruction::INC, None, Some(ArithmeticTarget::HlAddr)),
                //Dec
                Opcode::DecA => Instruction::Arithmetic(ArithmeticInstruction::DEC, None, Some(ArithmeticTarget::A)),
                Opcode::DecB => Instruction::Arithmetic(ArithmeticInstruction::DEC, None, Some(ArithmeticTarget::B)),
                Opcode::DecC => Instruction::Arithmetic(ArithmeticInstruction::DEC, None, Some(ArithmeticTarget::C)),
                Opcode::DecD => Instruction::Arithmetic(ArithmeticInstruction::DEC, None, Some(ArithmeticTarget::D)),
                Opcode::DecE => Instruction::Arithmetic(ArithmeticInstruction::DEC, None, Some(ArithmeticTarget::E)),
                Opcode::DecH => Instruction::Arithmetic(ArithmeticInstruction::DEC, None, Some(ArithmeticTarget::H)),
                Opcode::DecL => Instruction::Arithmetic(ArithmeticInstruction::DEC, None, Some(ArithmeticTarget::L)),
                Opcode::DecAddrHL => Instruction::Arithmetic(ArithmeticInstruction::DEC, None, Some(ArithmeticTarget::HlAddr)),
                //--> 16 bits Arithmetics
                Opcode::AddHLBC => Instruction::Arithmetic(ArithmeticInstruction::ADDHL, None, Some(ArithmeticTarget::BC)),
                Opcode::AddHLDE => Instruction::Arithmetic(ArithmeticInstruction::ADDHL, None, Some(ArithmeticTarget::BC)),
                Opcode::AddHLHL => Instruction::Arithmetic(ArithmeticInstruction::ADDHL, None, Some(ArithmeticTarget::BC)),
                // --> Bits
                //And
                Opcode::AndAA => Instruction::Arithmetic(ArithmeticInstruction::AND, None, Some(ArithmeticTarget::A)),
                Opcode::AndAB => Instruction::Arithmetic(ArithmeticInstruction::AND, None, Some(ArithmeticTarget::B)),
                Opcode::AndAC => Instruction::Arithmetic(ArithmeticInstruction::AND, None, Some(ArithmeticTarget::C)),
                Opcode::AndAD => Instruction::Arithmetic(ArithmeticInstruction::AND, None, Some(ArithmeticTarget::D)),
                Opcode::AndAE => Instruction::Arithmetic(ArithmeticInstruction::AND, None, Some(ArithmeticTarget::E)),
                Opcode::AndAH => Instruction::Arithmetic(ArithmeticInstruction::AND, None, Some(ArithmeticTarget::H)),
                Opcode::AndAL => Instruction::Arithmetic(ArithmeticInstruction::AND, None, Some(ArithmeticTarget::L)),
                Opcode::AndAN8 => Instruction::Arithmetic(ArithmeticInstruction::AND, Some(Immediate::N8(read_next_byte(&mut reg.pc, mem_bus))), None),
                Opcode::AndAAddrHL => Instruction::Arithmetic(ArithmeticInstruction::AND, None, Some(ArithmeticTarget::HlAddr)),
                //Or
                Opcode::OrAA => Instruction::Arithmetic(ArithmeticInstruction::OR, None, Some(ArithmeticTarget::A)),
                Opcode::OrAB => Instruction::Arithmetic(ArithmeticInstruction::OR, None, Some(ArithmeticTarget::B)),
                Opcode::OrAC => Instruction::Arithmetic(ArithmeticInstruction::OR, None, Some(ArithmeticTarget::C)),
                Opcode::OrAD => Instruction::Arithmetic(ArithmeticInstruction::OR, None, Some(ArithmeticTarget::D)),
                Opcode::OrAE => Instruction::Arithmetic(ArithmeticInstruction::OR, None, Some(ArithmeticTarget::E)),
                Opcode::OrAH => Instruction::Arithmetic(ArithmeticInstruction::OR, None, Some(ArithmeticTarget::H)),
                Opcode::OrAL => Instruction::Arithmetic(ArithmeticInstruction::OR, None, Some(ArithmeticTarget::L)),
                Opcode::OrAN8 => Instruction::Arithmetic(ArithmeticInstruction::OR, Some(Immediate::N8(read_next_byte(&mut reg.pc, mem_bus))), None),
                Opcode::OrAAddrHL => Instruction::Arithmetic(ArithmeticInstruction::OR, None, Some(ArithmeticTarget::HlAddr)),                
                //Xor
                Opcode::XorAA => Instruction::Arithmetic(ArithmeticInstruction::XOR, None, Some(ArithmeticTarget::A)),
                Opcode::XorAB => Instruction::Arithmetic(ArithmeticInstruction::XOR, None, Some(ArithmeticTarget::B)),
                Opcode::XorAC => Instruction::Arithmetic(ArithmeticInstruction::XOR, None, Some(ArithmeticTarget::C)),
                Opcode::XorAD => Instruction::Arithmetic(ArithmeticInstruction::XOR, None, Some(ArithmeticTarget::D)),
                Opcode::XorAE => Instruction::Arithmetic(ArithmeticInstruction::XOR, None, Some(ArithmeticTarget::E)),
                Opcode::XorAH => Instruction::Arithmetic(ArithmeticInstruction::XOR, None, Some(ArithmeticTarget::H)),
                Opcode::XorAL => Instruction::Arithmetic(ArithmeticInstruction::XOR, None, Some(ArithmeticTarget::L)),
                Opcode::XorAN8 => Instruction::Arithmetic(ArithmeticInstruction::XOR, Some(Immediate::N8(read_next_byte(&mut reg.pc, mem_bus))), None),
                Opcode::XorAAddrHL => Instruction::Arithmetic(ArithmeticInstruction::XOR, None, Some(ArithmeticTarget::HlAddr)),
                //Cpl
                Opcode::Cpl => Instruction::Arithmetic(ArithmeticInstruction::CPL, None, None),
                //Bit op
                Opcode::Rlca => Instruction::Arithmetic(ArithmeticInstruction::RLCA, None, None),
                Opcode::Rrca => Instruction::Arithmetic(ArithmeticInstruction::RRCA, None, None),
                Opcode::Rla => Instruction::Arithmetic(ArithmeticInstruction::RLA, None, None),
                Opcode::Rra => Instruction::Arithmetic(ArithmeticInstruction::RRA, None, None),
                Opcode::Prefix => Self::try_read_prefixed(reg, mem_bus)?,
                //Flags
                Opcode::Ccf => Instruction::Arithmetic(ArithmeticInstruction::CCF, None,None),
                Opcode::Scf => Instruction::Arithmetic(ArithmeticInstruction::SCF, None,None),
                // MARK: JUMP INSTRUCTIONS
                //Call
                Opcode::CallAddrN16 => Instruction::Jump(JumpInstruction::Call, JumpTest::Always, Some(JumpTarget::Imm16(read_next_word(&mut reg.pc, mem_bus)))),
                Opcode::CallNCAddrN16 => Instruction::Jump(JumpInstruction::Call, JumpTest::NotCarry, Some(JumpTarget::Imm16(read_next_word(&mut reg.pc, mem_bus)))),
                Opcode::CallCAddrN16 => Instruction::Jump(JumpInstruction::Call, JumpTest::Carry, Some(JumpTarget::Imm16(read_next_word(&mut reg.pc, mem_bus)))),
                Opcode::CallNZAddrN16 => Instruction::Jump(JumpInstruction::Call, JumpTest::NotZero, Some(JumpTarget::Imm16(read_next_word(&mut reg.pc, mem_bus)))),
                Opcode::CallZAddrN16 => Instruction::Jump(JumpInstruction::Call, JumpTest::Zero, Some(JumpTarget::Imm16(read_next_word(&mut reg.pc, mem_bus)))),
                //Jp
                Opcode::JpAddrN16 => Instruction::Jump(JumpInstruction::Jp, JumpTest::Always, Some(JumpTarget::Imm16(read_next_word(&mut reg.pc, mem_bus)))),
                Opcode::JpNCAddrN16 => Instruction::Jump(JumpInstruction::Jp, JumpTest::NotCarry, Some(JumpTarget::Imm16(read_next_word(&mut reg.pc, mem_bus)))),
                Opcode::JpCAddrN16 => Instruction::Jump(JumpInstruction::Jp, JumpTest::Carry, Some(JumpTarget::Imm16(read_next_word(&mut reg.pc, mem_bus)))),
                Opcode::JpNZAddrN16 => Instruction::Jump(JumpInstruction::Jp, JumpTest::NotZero, Some(JumpTarget::Imm16(read_next_word(&mut reg.pc, mem_bus)))),
                Opcode::JpZAddrN16 => Instruction::Jump(JumpInstruction::Jp, JumpTest::Zero, Some(JumpTarget::Imm16(read_next_word(&mut reg.pc, mem_bus)))),
                Opcode::JpHL => Instruction::Jump(JumpInstruction::Jp, JumpTest::Always, Some(JumpTarget::HL)),
                //Jr
                Opcode::JrE8 => Instruction::Jump(JumpInstruction::Jr, JumpTest::Always, Some(JumpTarget::ImmS8(read_next_byte_signed(&mut reg.pc, mem_bus)))),
                Opcode::JrNCE8 => Instruction::Jump(JumpInstruction::Jr, JumpTest::NotCarry, Some(JumpTarget::ImmS8(read_next_byte_signed(&mut reg.pc, mem_bus)))),
                Opcode::JrCE8 => Instruction::Jump(JumpInstruction::Jr, JumpTest::Carry, Some(JumpTarget::ImmS8(read_next_byte_signed(&mut reg.pc, mem_bus)))),
                Opcode::JrNZE8 => Instruction::Jump(JumpInstruction::Jr, JumpTest::NotZero, Some(JumpTarget::ImmS8(read_next_byte_signed(&mut reg.pc, mem_bus)))),
                Opcode::JrZE8 => Instruction::Jump(JumpInstruction::Jr, JumpTest::Zero, Some(JumpTarget::ImmS8(read_next_byte_signed(&mut reg.pc, mem_bus)))),
                //Ret
                Opcode::Ret => Instruction::Jump(JumpInstruction::Ret, JumpTest::Always, None),
                Opcode::RetNC => Instruction::Jump(JumpInstruction::Ret, JumpTest::NotCarry, None),
                Opcode::RetC => Instruction::Jump(JumpInstruction::Ret, JumpTest::Carry, None),
                Opcode::RetNZ => Instruction::Jump(JumpInstruction::Ret, JumpTest::NotZero, None),
                Opcode::RetZ => Instruction::Jump(JumpInstruction::Ret, JumpTest::Zero, None),
                Opcode::Reti => Instruction::Jump(JumpInstruction::RetI, JumpTest::Always, None),
                // MARK: LOAD INSTRUCTIONS
                //TODO


                // ===X===
                //--> Illegals instructions

                | Opcode::Illegal_d3
                | Opcode::Illegal_db
                | Opcode::Illegal_dd
                | Opcode::Illegal_e3
                | Opcode::Illegal_e4
                | Opcode::Illegal_eb
                | Opcode::Illegal_ec
                | Opcode::Illegal_ed
                | Opcode::Illegal_f4
                | Opcode::Illegal_fc
                | Opcode::Illegal_fd => panic!("Illegal opcode : {byte:02X}"),
                
                _ => unimplemented!(),
            })
        } else {
            None
        }
    }

    fn try_read_prefixed(reg : &mut Registers, mem_bus: &MemBus) -> Option<Instruction>{
        let byte = mem_bus.readb(reg.pc);
        reg.pc = reg.pc.wrapping_add(1);
        // let opcode = PrefixedOpcode::from(byte);

            let operand = byte & 0b111;
            let target = match operand{
                0b000 => ArithmeticTarget::B,
                0b001 => ArithmeticTarget::C,
                0b010 => ArithmeticTarget::D,
                0b011 => ArithmeticTarget::E,
                0b100 => ArithmeticTarget::H,
                0b101 => ArithmeticTarget::L,
                0b110 => ArithmeticTarget::HlAddr,
                0b111 => ArithmeticTarget::A,
                _=>unreachable!()
            };

            let instr = if byte >> 6 == 0 {
                let op = byte >> 3;
                match op{
                    0b000 => Instruction::Arithmetic(ArithmeticInstruction::RLC, None, Some(target)),
                    0b001 => Instruction::Arithmetic(ArithmeticInstruction::RRC, None, Some(target)),
                    0b010 => Instruction::Arithmetic(ArithmeticInstruction::RL, None, Some(target)),
                    0b011 => Instruction::Arithmetic(ArithmeticInstruction::RR, None, Some(target)),
                    0b100 => Instruction::Arithmetic(ArithmeticInstruction::SLA, None, Some(target)),
                    0b101 => Instruction::Arithmetic(ArithmeticInstruction::SRA, None, Some(target)),
                    0b110 => Instruction::Arithmetic(ArithmeticInstruction::SWAP, None, Some(target)),
                    0b111 => Instruction::Arithmetic(ArithmeticInstruction::SRL, None, Some(target)),
                    _=>unreachable!()
                }
            }else {
                let bit_index = Immediate::E3((((byte & 0b0011_1000) >> 4) as u8).try_into().ok()?);
                if byte >> 6 == 0b01 {
                    //Bit instruction
                    Instruction::Arithmetic(ArithmeticInstruction::BIT, Some(bit_index), Some(target))
                }else if byte >> 6 == 0b10 {
                    Instruction::Arithmetic(ArithmeticInstruction::RES, Some(bit_index), Some(target))
                }else {
                    Instruction::Arithmetic(ArithmeticInstruction::SET, Some(bit_index), Some(target))
                }
            };
            Some(instr)
            

    }

}

#[inline]
fn read_next_byte_signed(pc: &mut u16, mem_bus: &MemBus) -> i8{
    u8::cast_signed(read_next_byte(pc, mem_bus))
}

fn read_next_byte(pc: &mut u16, mem_bus: &MemBus) -> u8{
    let byte = mem_bus.readb(*pc);
    *pc = pc.wrapping_add(1);
    byte
}

fn read_next_word(pc: &mut u16, mem_bus: &MemBus) -> u16{
    let word = mem_bus.readw(*pc);
    *pc = pc.wrapping_add(2);
    word
}