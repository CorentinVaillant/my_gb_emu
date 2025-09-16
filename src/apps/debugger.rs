use std::{error::Error, io::Write};

use crate::{cpu::Cpu, utils::open_rom};

const MSG: &str = "[mem/reg/step/break <u16>/clear]: ";

pub fn debug(path : &str) -> Result<(), Box<dyn Error>> {
    let mem_bus = open_rom(path)?;
    let mut cpu = Cpu::new(mem_bus);
    let mut break_points: Vec<u16> = vec![];

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buff = String::new();
    while !cpu.halted {
        buff.clear();
        print!("[pc:0x{:04X}]{MSG}", cpu.reg.pc);
        stdout.flush()?;
        stdin.read_line(&mut buff)?;
        let mut split = buff.trim().split_ascii_whitespace();
        let arg1 = split.next();
        let arg2 = split.next();


        match (arg1, arg2) {
            (Some("m"), _) | (Some("mem"), _) => mem(&cpu),
            (Some("re"), _) | (Some("reg"), _) => reg(&cpu),
            (Some("s"), _) | (Some("step"), _) => step(&mut cpu),
            (Some("r"), _) | (Some("run"), _) => run(&mut cpu, &break_points),
            (Some("b"), Some(arg2)) | (Some("break"), Some(arg2)) => {
                add_break_point(arg2, &mut break_points)
            }
            (Some("clear"), _) => print!("\x1B[2J\x1B[1;1H"),
            (Some("exit"), _) => break,
            _ => println!("unknow command : \"{buff}\""),
        }
    }

    Ok(())
}

fn step(cpu: &mut Cpu) {
    cpu.step_verbose();
}

fn reg(cpu: &Cpu) {
    println!("Cpu Registers : {:#?}", cpu.reg)
}

fn mem(cpu: &Cpu) {
    println!("Cpu mem : {:#X?}", cpu.mem_bus)
}

fn add_break_point(arg2: &str, breaks: &mut Vec<u16>) {
    let addr = if arg2[0..2] == *"0x" {
        u16::from_str_radix(&arg2[2..], 16)
    } else {
        u16::from_str_radix(arg2, 10)
    };

    match addr {
        Ok(addr) => breaks.push(addr),
        Err(err) => println!("Could not parse {arg2} : {err}"),
    }
}

fn run(cpu: &mut Cpu, breaks: &Vec<u16>) {
    while !breaks.contains(&cpu.reg.pc) {
        cpu.step();
    }

    println!(" -- Break at 0x{:04X} -- ", cpu.reg.pc)
}
