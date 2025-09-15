use crate::cpu::{instructions::MiscInstruction, Cpu};

impl Cpu {
    pub(super) fn misc(&mut self, instr : MiscInstruction) {
        match instr {
            MiscInstruction::Nop => (),
            MiscInstruction::Halt => self.halt(),
            MiscInstruction::Di => self.di(),
            MiscInstruction::Ei => self.ei(),
            MiscInstruction::Stop(_) => self.stop(),
        }
    }

    fn halt(&mut self) {
        self.halted = true;
    }

    fn di(&mut self) {
        self.ime = false;
    }

    fn ei(&mut self) {
        self.ime = true;
    }

    fn stop(&mut self) {
        self.low_pow = true;
    }
}