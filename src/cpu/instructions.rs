

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Arithmetic(ArithmeticInstruction, Option<Immediate3Bits>, Option<ArithmeticTarget>)
}

#[derive(Debug, Clone, Copy)]
pub enum ArithmeticInstruction {
    ADD,
    ADDHL,
    ADC,
    SUB,
    SBC,
    AND,
    OR,
    XOR,
    CP,
    INC,
    DEC,
    CCF,
    SCF,
    RRA,
    RLA,
    RRCA,
    RLCA,
    CPL,
    BIT,
    RES,
    SET,
    SRL,
    RR,
    RL,
    RRC,
    RLC,
    SRA,
    SLA,
    SWAP,
}

#[derive(Debug, Clone, Copy)]
pub enum ArithmeticTarget {
    A, B, C, D, E, H, L,

    BC,DE,HL,

    Addr(u16),
}

#[derive(Debug,Clone, Copy)]
pub struct Immediate3Bits(u8);

#[derive(Debug,Clone, Copy)]
pub struct CouldNotFitIn3Bits;

impl std::fmt::Display for CouldNotFitIn3Bits{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not fit the value in 3 bits")
    }
}

impl TryFrom<u8> for Immediate3Bits{
    type Error = CouldNotFitIn3Bits;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value != value & 0b111{
            Err(CouldNotFitIn3Bits)
        }else {
            Ok(Self(value))
        }
    }
}

impl From<Immediate3Bits> for u8{
    fn from(value: Immediate3Bits) -> Self {
        value.0
    }
}