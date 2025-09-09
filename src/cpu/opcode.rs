//constants :
include!(concat!(env!("OUT_DIR"), "/opcode_rs/enum.rs"));

#[derive(Debug, Clone, Copy)]
pub struct InvalideOpcode;

impl TryFrom<u8> for Opcode {
    type Error = InvalideOpcode;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match byte_to_opcode(value) {
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
                | Opcode::Illegal_fd => Err(InvalideOpcode),

                op => Ok(op),
        }
    }
}

impl From<u8> for PrefixedOpcode{
    fn from(value: u8) -> Self {
        byte_to_prefixed_opcode(value)
    }
}

include! {concat!(env!("OUT_DIR"), "/opcode_rs/byte_to_opcode.rs")}
