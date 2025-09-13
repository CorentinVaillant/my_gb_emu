//constants :
include!(concat!(env!("OUT_DIR"), "/opcode_rs/enum.rs"));

#[derive(Debug, Clone, Copy)]
pub struct InvalideOpcode;

impl TryFrom<u8> for Opcode {
    type Error = InvalideOpcode;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match byte_to_opcode(value) {
                | Opcode::IllegalD3
                | Opcode::IllegalDb
                | Opcode::IllegalDd
                | Opcode::IllegalE3
                | Opcode::IllegalE4
                | Opcode::IllegalEb
                | Opcode::IllegalEc
                | Opcode::IllegalEd
                | Opcode::IllegalF4
                | Opcode::IllegalFc
                | Opcode::IllegalFd => Err(InvalideOpcode),

                op => Ok(op),
        }
    }
}

#[cfg(prefixed_opcode)]
impl From<u8> for PrefixedOpcode{
    fn from(value: u8) -> Self {
        byte_to_prefixed_opcode(value)
    }
}

include! {concat!(env!("OUT_DIR"), "/opcode_rs/byte_to_opcode.rs")}

include! {concat!(env!("OUT_DIR"), "/opcode_rs/mnemonic_enum.rs")}

include! {concat!(env!("OUT_DIR"), "/opcode_rs/opcode_to_mnemonics.rs")}
