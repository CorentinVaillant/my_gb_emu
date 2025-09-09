//constants :
include!(concat!(env!("OUT_DIR"), "/opcode_rs/enum.rs"));

use crate::utils::Value;
#[derive(Debug, Clone, Copy)]
pub struct InvalideOpcode;

impl TryFrom<Value> for Opcode {
    type Error = InvalideOpcode;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Byte(byte) => match byte_to_opcode(byte) {
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
            },
            Value::Word(word) => {
                if word & 0xCB00 != 0xCB{
                    Err(InvalideOpcode)
                }else{
                    let prefixed_byte = (word & 0x00FF) as u8;
                    Ok(Opcode::CBPrefixed(byte_to_prefixed_opcode(prefixed_byte)))
                }
            },
        }
    }
}

include! {concat!(env!("OUT_DIR"), "/opcode_rs/byte_to_opcode.rs")}
