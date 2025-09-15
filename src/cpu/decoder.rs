use crate::{
    cpu::{
        instructions::{ArithmeticInstruction, ArithmeticTarget, ByteLoadDest, Immediate, Instruction, JumpInstruction, JumpTarget, JumpTest, LoadDest, LoadSrc, MiscInstruction, StackInstruction, StackReg16, WordLoadDest}, opcode::{Mnemonic, Opcode}, registers::Registers
    },
    mem_bus::MemBus,
};

impl Instruction {
    ///Read the instruction point by pc, and increment
    pub fn try_read(reg : &mut Registers, mem_bus: &MemBus) -> Option<Instruction> {
        let byte = mem_bus.readb(reg.pc);
        reg.pc = reg.pc.wrapping_add(1);


        if let Ok(opcode) = Opcode::try_from(byte) {
            Some(match opcode.get_mnemonic() {
                // MARK: ALU INSTRUCTIONS
                //--> 8 bits Arithmetics
                //Add
	            Mnemonic::Add => {
                    //16 bits case
                    if byte & 0b1100_1111 == 0b0000_1001{
                        Instruction::Arithmetic(ArithmeticInstruction::Add, None, Some(byte_to_16_arithmetic_target(byte)))
                    }
                    //8bits case
                    else if opcode == Opcode::AddAN8{
                        Instruction::Arithmetic(ArithmeticInstruction::Add, Some(Immediate::N8(read_next_byte(&mut reg.pc, mem_bus))), None)
                    }else{
                        Instruction::Arithmetic(ArithmeticInstruction::Add, None, Some(byte_to_8_arithmetic_target(byte)))
                    }
                }
                //Adc
                Mnemonic::Adc => {
                    if opcode == Opcode::AdcAN8{
                        Instruction::Arithmetic(ArithmeticInstruction::Adc, Some(Immediate::N8(read_next_byte(&mut reg.pc, mem_bus))), None)
                    }else{
                        Instruction::Arithmetic(ArithmeticInstruction::Adc, None, Some(byte_to_8_arithmetic_target(byte)))
                    }
                }
                //Sub
                Mnemonic::Sub => {
                    if opcode == Opcode::SubAN8{
                        Instruction::Arithmetic(ArithmeticInstruction::Sub, Some(Immediate::N8(read_next_byte(&mut reg.pc, mem_bus))), None)
                    }else{
                        Instruction::Arithmetic(ArithmeticInstruction::Sub, None, Some(byte_to_8_arithmetic_target(byte)))
                    }
                }                
                //Sbc
                Mnemonic::Sbc => {
                    if opcode == Opcode::SbcAN8{
                        Instruction::Arithmetic(ArithmeticInstruction::Sbc, Some(Immediate::N8(read_next_byte(&mut reg.pc, mem_bus))), None)
                    }else{
                        Instruction::Arithmetic(ArithmeticInstruction::Sbc, None, Some(byte_to_8_arithmetic_target(byte)))
                    }
                }
                //Daa
                Mnemonic::Daa => {
                    Instruction::Arithmetic(ArithmeticInstruction::Daa, None, None)
                }
                //Cp
                Mnemonic::Cp => {
                    if opcode == Opcode::CpAN8{
                        Instruction::Arithmetic(ArithmeticInstruction::Cp, Some(Immediate::N8(read_next_byte(&mut reg.pc, mem_bus))), None)
                    }else{
                        Instruction::Arithmetic(ArithmeticInstruction::Cp, None, Some(byte_to_8_arithmetic_target(byte)))
                    }
                }
                //Inc
                Mnemonic::Inc => {
                    let operand = byte >> 3;
                    Instruction::Arithmetic(ArithmeticInstruction::Inc, None, Some(byte_to_8_arithmetic_target(operand)))
                }
                //Dec
                Mnemonic::Dec => {
                    let operand = byte >> 3;
                    Instruction::Arithmetic(ArithmeticInstruction::Dec, None, Some(byte_to_8_arithmetic_target(operand)))
                }
                // --> Bits
                //And
                Mnemonic::And => {
                    if opcode == Opcode::AndAN8{
                        Instruction::Arithmetic(ArithmeticInstruction::And, Some(Immediate::N8(read_next_byte(&mut reg.pc, mem_bus))), None)
                    }else{
                        Instruction::Arithmetic(ArithmeticInstruction::And, None, Some(byte_to_8_arithmetic_target(byte)))
                    }
                }
                //Or
                Mnemonic::Or => {
                    if opcode == Opcode::OrAN8{
                        Instruction::Arithmetic(ArithmeticInstruction::Or, Some(Immediate::N8(read_next_byte(&mut reg.pc, mem_bus))), None)
                    }else{
                        Instruction::Arithmetic(ArithmeticInstruction::Or, None, Some(byte_to_8_arithmetic_target(byte)))
                    }
                }              
                //Xor
                Mnemonic::Xor => {
                    if opcode == Opcode::XorAN8{
                        Instruction::Arithmetic(ArithmeticInstruction::Xor, Some(Immediate::N8(read_next_byte(&mut reg.pc, mem_bus))), None)
                    }else{
                        Instruction::Arithmetic(ArithmeticInstruction::Xor, None, Some(byte_to_8_arithmetic_target(byte)))
                    }
                }    
                //Cpl
                Mnemonic::Cpl => Instruction::Arithmetic(ArithmeticInstruction::Cpl, None, None),
                //Bit op
                Mnemonic::Rlca => Instruction::Arithmetic(ArithmeticInstruction::Rlca, None, None),
                Mnemonic::Rrca => Instruction::Arithmetic(ArithmeticInstruction::Rrca, None, None),
                Mnemonic::Rla => Instruction::Arithmetic(ArithmeticInstruction::Rla, None, None),
                Mnemonic::Rra => Instruction::Arithmetic(ArithmeticInstruction::Rra, None, None),
                Mnemonic::Prefix => Self::try_read_prefixed(reg, mem_bus)?,
                //Flags
                Mnemonic::Ccf => Instruction::Arithmetic(ArithmeticInstruction::Ccf, None,None),
                Mnemonic::Scf => Instruction::Arithmetic(ArithmeticInstruction::Scf, None,None),
                // MARK: JUMP INSTRUCTIONS
                //Call
                Mnemonic::Call => {
                    if opcode == Opcode::CallAddrN16 {
                        Instruction::Jump(JumpInstruction::Call, JumpTest::Always, Some(JumpTarget::Imm16(read_next_word(&mut reg.pc, mem_bus))))
                    }else{
                        let test = byte_to_jump_test(byte >> 3);
                        Instruction::Jump(JumpInstruction::Call, test, Some(JumpTarget::Imm16(read_next_word(&mut reg.pc, mem_bus))))
                    }
                }
                //Jp
                Mnemonic::Jp => {
                    if opcode == Opcode::JpAddrN16 {
                        Instruction::Jump(JumpInstruction::Jp, JumpTest::Always, Some(JumpTarget::Imm16(read_next_word(&mut reg.pc, mem_bus))))
                    }else{
                        let test = byte_to_jump_test(byte >> 3);
                        Instruction::Jump(JumpInstruction::Jp, test, Some(JumpTarget::Imm16(read_next_word(&mut reg.pc, mem_bus))))
                    }
                }
                //Jr
                Mnemonic::Jr => {
                    if opcode == Opcode::JrE8 {
                        Instruction::Jump(JumpInstruction::Jr, JumpTest::Always, Some(JumpTarget::ImmS8(read_next_byte_signed(&mut reg.pc, mem_bus))))
                    }else{
                        let test = byte_to_jump_test(byte >> 3);
                        Instruction::Jump(JumpInstruction::Jr, test, Some(JumpTarget::ImmS8(read_next_byte_signed(&mut reg.pc, mem_bus))))
                    }
                }
                //Ret
                Mnemonic::Ret => {
                    if opcode == Opcode::Ret{
                        Instruction::Jump(JumpInstruction::Ret, JumpTest::Always, None)
                    }else{
                        let test = byte_to_jump_test(byte >> 3);
                        Instruction::Jump(JumpInstruction::Ret, test, None)
                    }
                }

                Mnemonic::Reti => {
                    Instruction::Jump(JumpInstruction::RetI, JumpTest::Always, None)
                }
                //Rst
                Mnemonic::Rst => {
                    let target = byte & 0b0001_1000;
                    Instruction::Jump(JumpInstruction::Rst, JumpTest::Always, Some(JumpTarget::Imm16(target as u16)))
                }
                // MARK: LOAD INSTRUCTIONS
                Mnemonic::Ld => {
                    match opcode {
                        Opcode::LdAddrN16A => {
                            let imm = read_next_word(&mut reg.pc, mem_bus);
                            Instruction::Load(LoadDest::ByteDest(ByteLoadDest::AddrImm(imm)), LoadSrc::A)
                        }
                        Opcode::LdAAddrN16 => {
                            let imm = read_next_word(&mut reg.pc, mem_bus);
                            Instruction::Load(LoadDest::ByteDest(ByteLoadDest::A), LoadSrc::AddrImm(imm))
    
                        }
                        Opcode::LdHLSPiE8  => {
                            let imm = read_next_byte_signed(&mut reg.pc, mem_bus);
                            let addr = reg.sp.wrapping_add_signed(imm as i16);
                            Instruction::Load(LoadDest::WordDest(WordLoadDest::HL), LoadSrc::AddrImm(addr))
                        }
                        Opcode::LdSPHL => {
                            Instruction::Load(LoadDest::WordDest(WordLoadDest::SP), LoadSrc::HL)
                        }
                        Opcode::LdAddrN16SP => {
                            let imm = read_next_word(&mut reg.pc, mem_bus);
                            Instruction::Load(LoadDest::WordDest(WordLoadDest::AddrImm(imm)), LoadSrc::SP)
                        }
                        _ => {
                            if byte & 0b1100_0000 == 0b0100_0000{
                                let dest = byte_to_8_reg_load_dest((byte) >> 3);
                                let src = byte_to_8_reg_load_src(byte);
                                Instruction::Load(LoadDest::ByteDest(dest), src)

                            }else
                            // LD r8, imm8
                            if byte & 0b110 == 0b110 {
                                let dest = byte_to_8_reg_load_dest(byte >> 3);
                                let imm = read_next_byte(&mut reg.pc, mem_bus);
                                Instruction::Load(LoadDest::ByteDest(dest), LoadSrc::Imm8(imm))
                            }else{

                                let r16 = (byte & 0b0011_0000) >> 4;
                                match byte & 0xF {
                                    // LD r16 imm16
                                    0b0001 => {
                                        let dest = byte_to_16_load_dest(r16);
                                        let imm = read_next_word(&mut reg.pc, mem_bus);
                                        
                                        Instruction::Load(LoadDest::WordDest(dest), LoadSrc::Imm16(imm))
                                    },
                                // LD [r16] a
                                0b0010 => {
                                    let dest = byte_to_8_load_dest_addr(r16);
                                    
                                    Instruction::Load(LoadDest::ByteDest(dest), LoadSrc::A)
                                },
                                // LD a, [r16]
                                0b1010 => {
                                    let src = byte_to_8_load_src_addr(r16);
                                    
                                    Instruction::Load(LoadDest::ByteDest(ByteLoadDest::A), src)
                                },
                                
                                
                                
                                16..=u8::MAX => unreachable!(),
                                _ => return None,
                            }
                            }   
                            
                        }
                    }
                },
                Mnemonic::Ldh => {
                    match opcode {
                        Opcode::LdhCA => Instruction::Load(LoadDest::ByteDest(ByteLoadDest::C), LoadSrc::A),
                        Opcode::LdhAC => Instruction::Load(LoadDest::ByteDest(ByteLoadDest::A), LoadSrc::AddrC),
                        Opcode::LdhAddrN8A => {
                            let dest = read_next_byte(&mut reg.pc, mem_bus) as u16 + 0xFF;
                            Instruction::Load(LoadDest::ByteDest(ByteLoadDest::AddrImm(dest)), LoadSrc::A)
                        }
                        Opcode::LdhAAddrN8 => {
                            let src = read_next_byte(&mut reg.pc, mem_bus) as u16 + 0xFF;
                            Instruction::Load(LoadDest::ByteDest(ByteLoadDest::A), LoadSrc::AddrImm(src))
                        }
                        _ => return None,
                    }
                }

                //MARK: STACK INSTRUCTIONS
                Mnemonic::Push => {
                    let reg = byte_to_stack_r6(byte >> 4);
                    Instruction::Stack(StackInstruction::Push, reg)
                }
                Mnemonic::Pop => {
                    let reg = byte_to_stack_r6(byte >> 4);
                    Instruction::Stack(StackInstruction::Pop, reg)
                }

                //MARK: MISC INSTRUCTIONS
                Mnemonic::Nop =>
                    Instruction::Misc(MiscInstruction::Nop),
                Mnemonic::Halt =>
                    Instruction::Misc(MiscInstruction::Halt),
                Mnemonic::Ei =>
                    Instruction::Misc(MiscInstruction::Ei),
                Mnemonic::Di =>
                    Instruction::Misc(MiscInstruction::Di),

                Mnemonic::Stop=> {
                    let imm = read_next_byte(&mut reg.pc, mem_bus);
                    Instruction::Misc(MiscInstruction::Stop(imm))
                }
                
                //MARK: ILLEGAL INSTRUCTIONS    
                | Mnemonic::IllegalD3
                | Mnemonic::IllegalDb
                | Mnemonic::IllegalDd
                | Mnemonic::IllegalE3
                | Mnemonic::IllegalE4
                | Mnemonic::IllegalEb
                | Mnemonic::IllegalEc
                | Mnemonic::IllegalEd
                | Mnemonic::IllegalF4
                | Mnemonic::IllegalFc
                | Mnemonic::IllegalFd => return None,
                
                _ => unimplemented!("{opcode:?}"),
            })
        } else {
            None
        }
    }

    fn try_read_prefixed(reg : &mut Registers, mem_bus: &MemBus) -> Option<Instruction>{
        let byte = mem_bus.readb(reg.pc);
        reg.pc = reg.pc.wrapping_add(1);
        // let opcode = PrefixedOpcode::from(byte);

            let target = byte_to_8_arithmetic_target(byte);

            let instr = if byte >> 6 == 0 {
                let op = byte >> 3;
                match op{
                    0b000 => Instruction::Arithmetic(ArithmeticInstruction::Rlc, None, Some(target)),
                    0b001 => Instruction::Arithmetic(ArithmeticInstruction::Rrc, None, Some(target)),
                    0b010 => Instruction::Arithmetic(ArithmeticInstruction::Rl, None, Some(target)),
                    0b011 => Instruction::Arithmetic(ArithmeticInstruction::Rr, None, Some(target)),
                    0b100 => Instruction::Arithmetic(ArithmeticInstruction::Sla, None, Some(target)),
                    0b101 => Instruction::Arithmetic(ArithmeticInstruction::Sra, None, Some(target)),
                    0b110 => Instruction::Arithmetic(ArithmeticInstruction::Swap, None, Some(target)),
                    0b111 => Instruction::Arithmetic(ArithmeticInstruction::Srl, None, Some(target)),
                    _=>unreachable!()
                }
            }else {
                let bit_index = Immediate::E3(((byte & 0b0011_1000) >> 4).try_into().ok()?);
                if byte >> 6 == 0b01 {
                    //Bit instruction
                    Instruction::Arithmetic(ArithmeticInstruction::Bit, Some(bit_index), Some(target))
                }else if byte >> 6 == 0b10 {
                    Instruction::Arithmetic(ArithmeticInstruction::Res, Some(bit_index), Some(target))
                }else {
                    Instruction::Arithmetic(ArithmeticInstruction::Set, Some(bit_index), Some(target))
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



///convert the 3 last bits into a 8bits ArithmeticTarget enum
const fn byte_to_8_arithmetic_target(byte:u8) -> ArithmeticTarget{
    match byte & 0b111 {
        0b000 => ArithmeticTarget::B,
        0b001 => ArithmeticTarget::C,
        0b010 => ArithmeticTarget::D,
        0b011 => ArithmeticTarget::E,
        0b100 => ArithmeticTarget::H,
        0b101 => ArithmeticTarget::L,
        0b110 => ArithmeticTarget::HlAddr,
        0b111 => ArithmeticTarget::A,

        8_u8..=u8::MAX => unreachable!()
    }
}

///convert the 2 last bits into a 16bits ArithmeticTarget enum
const fn byte_to_16_arithmetic_target(byte:u8) -> ArithmeticTarget{
    match byte & 0b11 {
        0b00=> ArithmeticTarget::BC,
        0b01=> ArithmeticTarget::DE,
        0b10=> ArithmeticTarget::HL,
        0b11=> ArithmeticTarget::SP,

        4_u8..=u8::MAX => unreachable!()
    }
}

///convert the 2 last bits into a JumpTest, this does not take in account Always
const fn byte_to_jump_test(byte:u8) -> JumpTest{
    match byte & 0b11 {
        0b00 => JumpTest::NotZero,
        0b01 => JumpTest::Zero,
        0b10 => JumpTest::NotCarry,
        0b11 => JumpTest::Carry,
        
        4_u8..=u8::MAX => unreachable!()
    }
}

///convert the 2 last bits into a 16bits LoadDest enum
const fn byte_to_16_load_dest(byte:u8) -> WordLoadDest{
    match byte & 0b11 {
        0b00=> WordLoadDest::BC,
        0b01=> WordLoadDest::DE,
        0b10=> WordLoadDest::HL,
        0b11=> WordLoadDest::SP,

        4_u8..=u8::MAX => unreachable!()
    }
}

///convert the 2 last bits into a ByteLoadDest addr enum
const fn byte_to_8_load_dest_addr(byte:u8) -> ByteLoadDest{
    match byte & 0b11 {
        0b00=> ByteLoadDest::AddrBC,
        0b01=> ByteLoadDest::AddrDE,
        0b10=> ByteLoadDest::AddrHLadd,
        0b11=> ByteLoadDest::AddrHLsub,

        4_u8..=u8::MAX => unreachable!()
    }
}

///convert the last 2 bits into a LoadSrc addr enum
const fn byte_to_8_load_src_addr(byte:u8) -> LoadSrc{
    match byte & 0b11 {
        0b00=> LoadSrc::AddrBC,
        0b01=> LoadSrc::AddrDE,
        0b10=> LoadSrc::AddrHL,
        0b11=> LoadSrc::AddrSP,

        4_u8..=u8::MAX => unreachable!()
    }
}

///convert the 3 last bits into a 8bits reg LoadDest enum
const fn byte_to_8_reg_load_dest(byte:u8) -> ByteLoadDest{
    match byte & 0b111 {
        0b000 => ByteLoadDest::B,
        0b001 => ByteLoadDest::C,
        0b010 => ByteLoadDest::D,
        0b011 => ByteLoadDest::E,
        0b100 => ByteLoadDest::H,
        0b101 => ByteLoadDest::L,
        0b110 => ByteLoadDest::AddrHL,
        0b111 => ByteLoadDest::A,

        8_u8..=u8::MAX => unreachable!()
    }
}

///convert the 3 last bits into a 8bits reg LoadSrc enum
const fn byte_to_8_reg_load_src(byte:u8) -> LoadSrc{
    match byte & 0b111 {
        0b000 => LoadSrc::B,
        0b001 => LoadSrc::C,
        0b010 => LoadSrc::D,
        0b011 => LoadSrc::E,
        0b100 => LoadSrc::H,
        0b101 => LoadSrc::L,
        0b110 => LoadSrc::AddrHL,
        0b111 => LoadSrc::A,

        8_u8..=u8::MAX => unreachable!()
    }
}

///Convert the last 2 bits into a StackReg16 enum
const fn byte_to_stack_r6(byte:u8) -> StackReg16{
    match byte & 0b11 {
        0b00 => StackReg16::BC, 
        0b01 => StackReg16::DE, 
        0b10 => StackReg16::HL, 
        0b11 => StackReg16::AF, 

        4_u8..=u8::MAX => unreachable!()
    }
}