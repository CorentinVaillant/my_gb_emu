use std::error::Error;


mod cpu;
mod mem_bus;
pub mod utils;
mod apps;
pub mod graphics;


const HELP_MSG :&str = "
Usage :
\tgb_emu dbg <rom_path> : launch a tiny debugger onto a rom
\tgb_emu dasm <rom_path> : print the de-assemble rom 
";

fn main() -> Result<(),Box<dyn Error>> {
    let mut args = std::env::args();

    let _arg0 = args.next();
    let arg1 = args.next().map(|s|s.to_ascii_lowercase());
    let arg2 = args.next();

    match (arg1.as_deref(),arg2.as_deref()) {
        (Some("help"),_) => println!("{HELP_MSG}"),
        (Some("dbg"),Some(path)) => apps::debugger::debug(path)?,

        (Some("deass"),Some(path)) |
        (Some("deassemble"),Some(path)) |
        (Some("deasm"),Some(path)) |
        (Some("dasm"),Some(path))  => apps::deasm::desasm(path)?,

        (Some(x1),Some(x2)) => Err(format!("Unsuported args : {x1},{x2}"))?,
        (Some(x),None) => Err(format!("Unsuported args : {x}"))?,
        (None,_) => Err(String::from("Please give some arguments"))?,
    }
    
    Ok(())
}