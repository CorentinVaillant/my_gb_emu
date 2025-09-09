use std::{collections::HashMap, fs, io::Write, path::PathBuf};

//Files consts
pub const OPCODE_RS_ENUM_PATH: &str = "opcode_rs/enum.rs";
pub const OPCODE_RS_BYTE_TO_OPCODE_PATH: &str = "opcode_rs/byte_to_opcode.rs";

//Enum gen const
const OPCODE_RS_ENUM_HEAD: &str =
    "//generated with build.rs \n//contain all the constants for opcode.rs\n";

const OPCODE_RS_ENUM_DECLARATION: &str = "\n#[allow(non_camel_case_types)]\npub enum Opcode{\n";
const OPCODE_RS_PREFIXED_ENUM_DECLARATION: &str =
    "\n#[allow(non_camel_case_types)]\npub enum PrefixedOpcode{\n";
//Convertion gen const
pub const OPCODE_RS_TRY_FROM_VALUE_DECLARATION: &str =
    "\nuse crate::utils::Value;\n impl TryFrom<Value> for Opcode{\n
\t
";

use serde::Deserialize;

pub fn main() {
    println!("cargo::rerun-if-changed=build_resources/opcodes.json");

    let json_str = include_str!("build_resources/opcodes.json");
    let data: JsonData = serde_json::from_str(json_str).expect("Could not read data");

    let out_dir = std::env::var("OUT_DIR").expect("Could not get OUT_DIR env var");
    generate_opcode_rs_enum(&data, &out_dir);
}

type JsonData = HashMap<String, HashMap<String, Instruction>>;

fn generate_opcode_rs_enum(data: &JsonData, out_dir: &String) {
    // -- File creation
    let dest = PathBuf::from(out_dir).join(OPCODE_RS_ENUM_PATH);

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).expect("Could not create directories for output");
    }

    println!("Creating file : {:?}", dest);
    let mut output = fs::File::create(dest).expect("Could not create the output file");

    output
        .write_all(OPCODE_RS_ENUM_HEAD.as_bytes())
        .expect("Could not write into the output file");

    // -- Non Prefixed
    output
        .write_all(OPCODE_RS_ENUM_DECLARATION.as_bytes())
        .expect("Could not write into the output file");

    let unprefixed_data: Vec<_> = get_opcode_data_sorted(&data["unprefixed"]);
    for (opcode, instruction) in unprefixed_data.iter() {
        let to_write = format!(
            "\t/*{opcode:02X}*/ {}, \t//{}\n",
            instruction.to_string(""),
            instruction.description()
        );
        output
            .write_all(to_write.as_bytes())
            .expect("Could not write into the output file");
    }

    // output
    //     .write_all("\tCBPrefixed(PrefixedOpcode)\n".as_bytes())
    //     .expect("Could not write into the output file");

    output
        .write_all("\n}".as_bytes())
        .expect("Could not write into the output file");

    // -- Non Prefixed
    output
        .write_all(OPCODE_RS_PREFIXED_ENUM_DECLARATION.as_bytes())
        .expect("Could not write into the output file");

    let prefixed_data: Vec<_> = get_opcode_data_sorted(&data["cbprefixed"]);
    for (opcode, instruction) in prefixed_data.iter() {
        let to_write = format!("\t/*{opcode:02X}*/ {},\n", instruction.to_string(""),);
        output
            .write_all(to_write.as_bytes())
            .expect("Could not write into the output file");
    }

    output
        .write_all("}\n".as_bytes())
        .expect("Could not write into the output file");

    // -- TryFrom Value

    // -- File creation
    let dest = PathBuf::from(out_dir).join(OPCODE_RS_BYTE_TO_OPCODE_PATH);

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).expect("Could not create directories for output");
    }

    println!("Creating file : {:?}", dest);
    let mut output = fs::File::create(dest).expect("Could not create the output file");

    // -- Byte to opcode
    output
        .write_all(
            "\nconst fn byte_to_opcode(byte:u8)->Opcode{\n
    match byte{ \n"
                .as_bytes(),
        )
        .expect("Could not write into the output file");
    for (opcode, instruction) in unprefixed_data.iter() {
        output
            .write_all(
                format!(
                    "\t\t0x{opcode:02X} => Opcode::{},\n",
                    instruction.to_string("")
                )
                .as_bytes(),
            )
            .expect("Could not write into the output file");
    }
    output
        .write_all("\t}\n}".as_bytes())
        .expect("Could not write into the output file");

    // -- Byte to prefixed opcode
    output
        .write_all(
            "\nconst fn byte_to_prefixed_opcode(byte:u8)->PrefixedOpcode{\n
    match byte{ \n"
                .as_bytes(),
        )
        .expect("Could not write into the output file");
    for (opcode, instruction) in prefixed_data.iter() {
        output
            .write_all(
                format!(
                    "\t\t0x{opcode:02X} => PrefixedOpcode::{},\n",
                    instruction.to_string("")
                )
                .as_bytes(),
            )
            .expect("Could not write into the output file");
    }
    output
        .write_all("\t}\n}".as_bytes())
        .expect("Could not write into the output file");
}

fn get_opcode_data_sorted(data: &HashMap<String, Instruction>) -> Vec<(u8, Instruction)> {
    let mut data: Vec<_> = data
        .iter()
        .map(|(opcode, instruction)| {
            let trim = opcode.trim_start_matches("0x");
            (
                u8::from_str_radix(trim, 16).expect("could not parse opcode"),
                instruction.clone(),
            )
        })
        .collect();

    data.sort_by(|(op1, _), (op2, _)| op1.cmp(op2));

    data
}

//structs
#[derive(Debug, Clone, Deserialize)]
struct Operand {
    name: String,
    decrement: Option<bool>,
    increment: Option<bool>,
    // bytes: Option<u8>,
    immediate: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct Instruction {
    mnemonic: String,
    // bytes: u8,
    // cycles: Vec<u8>,
    operands: Vec<Operand>,
    // immediate: bool,
}

fn to_camelcase(string: &String) -> String {
    string
        .split_ascii_whitespace()
        .map(|word| {
            let mut word = word.to_string();
            word.as_mut_str()
                .get_mut(0..1)
                .map(|c| c.make_ascii_uppercase());
            word
        })
        .collect()
}

impl Instruction {
    fn to_string(&self, separator: &str) -> String {
        let mut buff = to_camelcase(&self.mnemonic.to_ascii_lowercase());
        for operand in &self.operands {
            let ope_name = if operand.name.starts_with('$') {
                operand.name[1..].to_string()
            } else if operand.name.starts_with('a') {
                format!("AddrN{}",&operand.name[1..])
            } else {
                let mut name = operand.name.clone();

                if let Some(decrement) = operand.decrement
                    && decrement
                {
                    name = format!("{}d", name);
                } else if let Some(increment) = operand.increment
                    && increment
                {
                    name = format!("{}i", name);
                }
                if !operand.immediate && operand.name == "HL" {
                    name = format!("Addr{}", name)
                }
                name
            };

            buff = format!("{}{}{}", buff, separator, to_camelcase(&ope_name));
        }

        buff
    }

    fn description(&self) -> String {
        let mut buff = self.mnemonic.clone();

        for operand in &self.operands {
            let ope_name = if operand.name.starts_with('$') {
                operand.name[1..].to_string()
            }else if operand.name.starts_with('a') {
                format!("[n{}]",&operand.name[1..])
            } else {
                let mut name = operand.name.clone();

                if let Some(decrement) = operand.decrement
                    && decrement
                {
                    name = format!("{}-", name);
                } else if let Some(increment) = operand.increment
                    && increment
                {
                    name = format!("{}+", name);
                }
                if !operand.immediate && operand.name == "HL" {
                    name = format!("[{}]", name)
                }
                name
            };

            buff = format!("{} {}", buff, ope_name);
        }

        buff
    }
}
