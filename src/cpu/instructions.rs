use std::fmt::Display;

pub use arithmetic::*;
pub use jump::*;
pub use load::*;
pub use stack::*;
pub use misc::*;


#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Arithmetic(ArithmeticInstruction,Option<Immediate>,Option<ArithmeticTarget>),
    Jump(JumpInstruction,JumpTest,Option<JumpTarget>),
    Load(LoadDest, LoadSrc),
    Stack(StackInstruction, StackReg16),
    Misc(MiscInstruction)
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Instruction::Arithmetic(instr, imm, target) => {
                write!(f,"{instr}")?;
                if let Some(imm) = *imm {
                    write!(f, " {imm}")?;
                }
                if let Some(target) = target {
                    write!(f, " {target}")?;
                }
                Ok(())
            },
            Instruction::Jump(instr, test, target) => {
                write!(f, "{instr}{test} ")?;
                if let Some(target) = target {
                    write!(f, " {target}")?;
                }
                Ok(())
            },
            Instruction::Load(load_dest, load_src) => write!(f, "LD {load_dest} {load_src}"),
            Instruction::Stack(instr, reg16) => write!(f, "{instr} {reg16}"),
            Instruction::Misc(instr) => write!(f, "{instr}"),
        }
    }
}

mod arithmetic {
    use std::fmt::Display;

    use crate::utils::Value;

    #[derive(Debug, Clone, Copy)]
    pub enum ArithmeticInstruction {
        Add,
        AddHl,
        Adc,
        Sub,
        Sbc,
        And,
        Or,
        Xor,
        Cp,
        Inc,
        Dec,
        Ccf,
        Scf,
        Rra,
        Rla,
        Rrca,
        Rlca,
        Cpl,
        Bit,
        Res,
        Set,
        Srl,
        Rr,
        Rl,
        Rrc,
        Rlc,
        Sra,
        Sla,
        Swap,
        Daa,
    }

    impl Display for ArithmeticInstruction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ArithmeticInstruction::Add => write!(f,"Add"),
                ArithmeticInstruction::AddHl => write!(f,"AddHl"),
                ArithmeticInstruction::Adc => write!(f,"Adc"),
                ArithmeticInstruction::Sub => write!(f,"Sub"),
                ArithmeticInstruction::Sbc => write!(f,"Sbc"),
                ArithmeticInstruction::And => write!(f,"And"),
                ArithmeticInstruction::Or => write!(f,"Or"),
                ArithmeticInstruction::Xor => write!(f,"Xor"),
                ArithmeticInstruction::Cp => write!(f,"Cp"),
                ArithmeticInstruction::Inc => write!(f,"Inc"),
                ArithmeticInstruction::Dec => write!(f,"Dec"),
                ArithmeticInstruction::Ccf => write!(f,"Ccf"),
                ArithmeticInstruction::Scf => write!(f,"Scf"),
                ArithmeticInstruction::Rra => write!(f,"Rra"),
                ArithmeticInstruction::Rla => write!(f,"Rla"),
                ArithmeticInstruction::Rrca => write!(f,"Rrca"),
                ArithmeticInstruction::Rlca => write!(f,"Rlca"),
                ArithmeticInstruction::Cpl => write!(f,"Cpl"),
                ArithmeticInstruction::Bit => write!(f,"Bit"),
                ArithmeticInstruction::Res => write!(f,"Res"),
                ArithmeticInstruction::Set => write!(f,"Set"),
                ArithmeticInstruction::Srl => write!(f,"Srl"),
                ArithmeticInstruction::Rr => write!(f,"Rr"),
                ArithmeticInstruction::Rl => write!(f,"Rl"),
                ArithmeticInstruction::Rrc => write!(f,"Rrc"),
                ArithmeticInstruction::Rlc => write!(f,"Rlc"),
                ArithmeticInstruction::Sra => write!(f,"Sra"),
                ArithmeticInstruction::Sla => write!(f,"Sla"),
                ArithmeticInstruction::Swap => write!(f,"Swap"),
                ArithmeticInstruction::Daa => write!(f,"Daa"),
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum ArithmeticTarget {
        A,
        B,
        C,
        D,
        E,
        H,
        L,

        BC,
        DE,
        HL,

        SP,
        PC,

        HlAddr,
    }

    impl Display for ArithmeticTarget {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self{
                ArithmeticTarget::A => write!(f, "A"),
                ArithmeticTarget::C => write!(f, "C"),
                ArithmeticTarget::B => write!(f, "B"),
                ArithmeticTarget::E => write!(f, "E"),
                ArithmeticTarget::D => write!(f, "D"),
                ArithmeticTarget::L => write!(f, "L"),
                ArithmeticTarget::H => write!(f, "H"),
                ArithmeticTarget::DE => write!(f, "DE"),
                ArithmeticTarget::BC => write!(f, "BC"),
                ArithmeticTarget::SP => write!(f, "SP"),
                ArithmeticTarget::HL => write!(f, "HL"),
                ArithmeticTarget::HlAddr => write!(f, "[HL]"),
                ArithmeticTarget::PC => write!(f, "PC"),
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Immediate {
        E3(Immediate3Bits),
        N8(u8),
        N16(u16),
    }

    impl Display for Immediate{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Immediate::E3(e3) => write!(f,"0x{:1X}", e3.0),
                Immediate::N8(n8) => write!(f, "0x{:02X}", n8),
                Immediate::N16(n16) => write!(f, "0x{:04X}", n16),
            }
        }
    }

    impl From<Value> for Immediate {
        fn from(value: Value) -> Self {
            match value {
                Value::Word(val) => Immediate::N16(val),
                Value::Byte(val) => Immediate::N8(val),
            }
        }
    }

    impl From<Immediate> for Value {
        fn from(value: Immediate) -> Self {
            match value {
                Immediate::E3(val) => Value::Byte(val.into()),
                Immediate::N8(val) => Value::Byte(val),
                Immediate::N16(val) => Value::Word(val),
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Immediate3Bits(u8);

    #[derive(Debug, Clone, Copy)]
    pub struct CouldNotFitIn3Bits;

    impl std::fmt::Display for CouldNotFitIn3Bits {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "could not fit the value in 3 bits")
        }
    }

    impl TryFrom<u8> for Immediate3Bits {
        type Error = CouldNotFitIn3Bits;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            if value != value & 0b111 {
                Err(CouldNotFitIn3Bits)
            } else {
                Ok(Self(value))
            }
        }
    }

    impl From<Immediate3Bits> for u8 {
        fn from(value: Immediate3Bits) -> Self {
            value.0
        }
    }
}

mod jump{
    use std::fmt::Display;

    #[derive(Debug,Clone, Copy)]
    pub enum JumpInstruction {
        Jp,
        Jr,
        Call,
        Ret, 
        RetI,
        Rst,
    }

    impl Display for JumpInstruction{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                JumpInstruction::Jp => write!(f, "Jp"),
                JumpInstruction::Jr => write!(f, "Jr"),
                JumpInstruction::Call => write!(f, "Call"),
                JumpInstruction::Ret => write!(f, "Ret"),
                JumpInstruction::RetI => write!(f, "RetI"),
                JumpInstruction::Rst => write!(f, "Rst"),
            }
        }
    }

    #[derive(Debug,Clone, Copy)]
    pub enum JumpTest {
        NotZero, 
        Zero,
        NotCarry,
        Carry,
        Always
    }

    impl Display for JumpTest {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                JumpTest::NotZero => write!(f, "NZ"),
                JumpTest::Zero => write!(f, "Z"),
                JumpTest::NotCarry => write!(f, "NC"),
                JumpTest::Carry => write!(f, "C"),
                JumpTest::Always => write!(f, "AL"),
            }
        }
    }

    #[derive(Debug,Clone, Copy)]
    pub enum JumpTarget {
        Imm16(u16),
        ImmS8(i8),
        HL,
    }

    impl Display for JumpTarget{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                JumpTarget::Imm16(imm) => write!(f, "0x{:04X}",imm),
                JumpTarget::ImmS8(imm) => write!(f, "0x{:02X}",imm),
                JumpTarget::HL => write!(f, "HL"),
            }
        }
    }

    
}

mod load{
    use std::fmt::Display;

    #[derive(Debug,Clone, Copy)]
    pub enum ByteLoadDest{
        A, B, C, D, E, H, L, 

        AddrC,
        AddrSP,

        AddrBC, AddrDE, AddrHL, AddrHLadd, AddrHLsub,

        AddrImm(u16),
    }

    impl Display for ByteLoadDest {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ByteLoadDest::A => write!(f, "A"),
                ByteLoadDest::B => write!(f, "B"),
                ByteLoadDest::C => write!(f, "C"),
                ByteLoadDest::D => write!(f, "D"),
                ByteLoadDest::E => write!(f, "E"),
                ByteLoadDest::H => write!(f, "H"),
                ByteLoadDest::L => write!(f, "L"),
                ByteLoadDest::AddrC => write!(f, "[C]"),
                ByteLoadDest::AddrSP => write!(f, "[SP]"),
                ByteLoadDest::AddrBC => write!(f, "[BC]"),
                ByteLoadDest::AddrDE => write!(f, "[DE]"),
                ByteLoadDest::AddrHL => write!(f, "[HL]"),
                ByteLoadDest::AddrHLadd => write!(f, "[HL+]"),
                ByteLoadDest::AddrHLsub => write!(f, "[HL-]"),
                ByteLoadDest::AddrImm(imm) => write!(f, "0x{imm:04X}"),
            }
        }
    }

    #[derive(Debug,Clone, Copy)]
    pub enum WordLoadDest{

        BC, DE, HL,

        SP,

        AddrC,
        AddrSP,

        AddrBC, AddrDE, AddrHL, AddrHLadd, AddrHLsub,

        AddrImm(u16),
    }

    impl Display for WordLoadDest{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                WordLoadDest::BC => write!(f, "BC"),
                WordLoadDest::DE => write!(f, "DE"),
                WordLoadDest::HL => write!(f, "HL"),
                WordLoadDest::SP => write!(f, "SP"),
                WordLoadDest::AddrC => write!(f, "[C]"),
                WordLoadDest::AddrSP => write!(f, "[SP]"),
                WordLoadDest::AddrBC => write!(f, "[BC]"),
                WordLoadDest::AddrDE => write!(f, "[DE]"),
                WordLoadDest::AddrHL => write!(f, "[HL]"),
                WordLoadDest::AddrHLadd => write!(f, "[HL+]"),
                WordLoadDest::AddrHLsub => write!(f, "[HL-]"),
                WordLoadDest::AddrImm(imm) => write!(f, " [{imm:04X}]"),
            }
        }
    }

    #[derive(Debug,Clone, Copy)]
    pub enum LoadDest {
        ByteDest(ByteLoadDest),
        WordDest(WordLoadDest),
    }

    impl Display for LoadDest {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                LoadDest::ByteDest(dest) => write!(f, "{dest}"),
                LoadDest::WordDest(dest) => write!(f, "{dest}"),
            }
        }
    }

    #[derive(Debug,Clone, Copy)]
    pub enum LoadSrc {
        A, B, C, D, E, H, L, 

        HL,
        SP,
        
        Imm8(u8),
        Imm16(u16),

        AddrSP,
        AddrBC, AddrDE, AddrHL, AddrHLadd, AddrHLsub, 
        AddrC,

        AddrImm(u16),
    }

    impl Display for LoadSrc {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self{
                LoadSrc::A => write!(f, "A"),
                LoadSrc::B => write!(f, "B"),
                LoadSrc::C => write!(f, "C"),
                LoadSrc::D => write!(f, "D"),
                LoadSrc::E => write!(f, "E"),
                LoadSrc::H => write!(f, "H"),
                LoadSrc::L => write!(f, "L"),
                LoadSrc::HL => write!(f, "HL"),
                LoadSrc::SP => write!(f, "SP"),
                LoadSrc::Imm8(imm) => write!(f,"0x{imm:02X}"),
                LoadSrc::Imm16(imm) => write!(f,"0x{imm:04X}"),
                LoadSrc::AddrSP => write!(f, "[SP]"),
                LoadSrc::AddrBC => write!(f, "[BC]"),
                LoadSrc::AddrDE => write!(f, "[DE]"),
                LoadSrc::AddrHL => write!(f, "[HL]"),
                LoadSrc::AddrHLadd => write!(f, "[HL+]"),
                LoadSrc::AddrHLsub => write!(f, "[HL-]"),
                LoadSrc::AddrC => write!(f, "[C]"),
                LoadSrc::AddrImm(imm) => write!(f, "[0x{imm:04X}]"),
            }
        }
    }
}

mod stack{
    use std::fmt::Display;

    #[derive(Debug,Clone, Copy)]
    pub enum StackReg16{
        BC,
        DE,
        HL, 
        AF
    }

    impl Display for StackReg16{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                StackReg16::BC => write!(f, "BC"),
                StackReg16::DE => write!(f, "DE"),
                StackReg16::HL => write!(f, "HL"),
                StackReg16::AF => write!(f, "AF"),
            }
        }
    }

    #[derive(Debug,Clone, Copy)]
    pub enum StackInstruction{
        Push,Pop
    }

    impl Display for StackInstruction{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self{
                StackInstruction::Push => write!(f, "Push"),
                StackInstruction::Pop => write!(f, "Pop"),
            } 
        }
    }
}

mod misc{
    use std::fmt::Display;

    #[derive(Debug,Clone, Copy)]
    pub enum MiscInstruction{
        Nop, 
        Halt,
        Di, 
        Ei,
        Stop(u8),
    }

    impl Display for MiscInstruction{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MiscInstruction::Nop => write!(f, "Nop"),
                MiscInstruction::Halt => write!(f, "Halt"),
                MiscInstruction::Di => write!(f, "Di"),
                MiscInstruction::Ei => write!(f, "Ei"),
                MiscInstruction::Stop(n8) => write!(f, "Stop 0x{n8:02X}")
            }
        }
    }
}