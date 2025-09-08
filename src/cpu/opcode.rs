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
                | Opcode::ILLEGAL_D3
                | Opcode::ILLEGAL_DB
                | Opcode::ILLEGAL_DD
                | Opcode::ILLEGAL_E3
                | Opcode::ILLEGAL_E4
                | Opcode::ILLEGAL_EB
                | Opcode::ILLEGAL_EC
                | Opcode::ILLEGAL_ED
                | Opcode::ILLEGAL_F4
                | Opcode::ILLEGAL_FC
                | Opcode::ILLEGAL_FD => Err(InvalideOpcode),

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
