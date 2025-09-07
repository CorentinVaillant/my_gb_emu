use crate::mem::MemBus;

#[derive(Debug)]
pub struct Cpu{
    //registers
    pub reg_af: u16,     // A, - => Accumulator & Flags
    pub reg_bc: u16,     // B, C => BC
    pub reg_de: u16,     // D, E => DE
    pub reg_hl: u16,     // H, L => HL
    pub reg_sp: u16,     // -, - => Stack Pointer
    pub reg_pc: u16,     // -, - => Program Counter/Pointer

    mem_bus : MemBus
}

impl Cpu{

    fn step(&mut self){
        // let byte = self.mem_bus.
    }
}