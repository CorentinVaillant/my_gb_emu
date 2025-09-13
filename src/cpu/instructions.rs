pub use arithmetic::*;
pub use jump::*;
pub use load::*;


#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Arithmetic(ArithmeticInstruction,Option<Immediate>,Option<ArithmeticTarget>),
    Jump(JumpInstruction,JumpTest,Option<JumpTarget>),
    Load(LoadDest, LoadSrc)
}

mod arithmetic {
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

    #[derive(Debug, Clone, Copy)]
    pub enum Immediate {
        E3(Immediate3Bits),
        N8(u8),
        N16(u16),
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
    #[derive(Debug,Clone, Copy)]
    pub enum JumpInstruction {
        Jp,
        Jr,
        Call,
        Ret, 
        RetI,
        Rst,
    }

    #[derive(Debug,Clone, Copy)]
    pub enum JumpTest {
        NotZero, 
        Zero,
        NotCarry,
        Carry,
        Always
    }

    #[derive(Debug,Clone, Copy)]
    pub enum JumpTarget {
        Imm16(u16),
        ImmS8(i8),
        HL,
        Vec, // ?
    }

    
}


mod load{
    #[derive(Debug,Clone, Copy)]
    pub enum ByteLoadDest{
        A, B, C, D, E, H, L, 

        AddrC,
        AddrSP,

        AddrBC, AddrDE, AddrHL, AddrHLadd, AddrHLsub,

        AddrImm(u16),
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

    #[derive(Debug,Clone, Copy)]
    pub enum LoadDest {
        ByteDest(ByteLoadDest),
        WordDest(WordLoadDest),
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
}
