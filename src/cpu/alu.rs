use crate::{
    cpu::{
        Cpu,
        errors::IllegalInstructionErr,
        instructions::{
            ArithmeticInstruction, ArithmeticTarget, Immediate, Immediate3Bits, Instruction,
        },
    },
    utils::Value,
};

impl Cpu {
    pub(super) fn alu(
        &mut self,
        instruction: ArithmeticInstruction,
        opt_imm: Option<Immediate>,
        opt_target: Option<ArithmeticTarget>,
    ) -> Result<(), IllegalInstructionErr> {
        match (opt_imm, opt_target) {
            (Some(Immediate::E3(imm3b)), Some(target)) => {
                let value = self.get_arithmetic_target(target);
                match value {
                    Value::Byte(value) => match instruction {
                        ArithmeticInstruction::Bit => self.bit(imm3b, value),
                        ArithmeticInstruction::Res => self.res(imm3b, target),
                        ArithmeticInstruction::Set => self.set(imm3b, target),

                        _ => Err(Instruction::Arithmetic(instruction, opt_imm, opt_target))?,
                    },
                    _ => Err(Instruction::Arithmetic(instruction, opt_imm, opt_target))?,
                }
            }

            (Some(imm), None) => {
                let value: Value = imm.into();
                match value {
                    Value::Byte(value) => match instruction {
                        ArithmeticInstruction::Add => self.add(value),
                        ArithmeticInstruction::Adc => self.adc(value),
                        ArithmeticInstruction::Sub => self.sub(value),
                        ArithmeticInstruction::Sbc => self.sbc(value),
                        ArithmeticInstruction::And => self.and(value),
                        ArithmeticInstruction::Or => self.or(value),
                        ArithmeticInstruction::Xor => self.xor(value),
                        ArithmeticInstruction::Cp => self.cp(value),

                        _ => Err(Instruction::Arithmetic(instruction, opt_imm, opt_target))?,
                    },
                    Value::Word(value) => match instruction {
                        ArithmeticInstruction::AddHl => self.addhl(value),

                        _ => Err(Instruction::Arithmetic(instruction, opt_imm, opt_target))?,
                    },
                }
            }

            (_, Some(target)) => {
                let value = self.get_arithmetic_target(target);
                match value {
                    Value::Byte(value) => match instruction {
                        ArithmeticInstruction::Add => self.add(value),
                        ArithmeticInstruction::Adc => self.adc(value),
                        ArithmeticInstruction::Sub => self.sub(value),
                        ArithmeticInstruction::Sbc => self.sbc(value),
                        ArithmeticInstruction::And => self.and(value),
                        ArithmeticInstruction::Or => self.or(value),
                        ArithmeticInstruction::Xor => self.xor(value),
                        ArithmeticInstruction::Cp => self.cp(value),

                        ArithmeticInstruction::Inc => self.inc(target),
                        ArithmeticInstruction::Dec => self.dec(target),
                        ArithmeticInstruction::Srl => self.srl(target),
                        ArithmeticInstruction::Rr => self.rr(target),
                        ArithmeticInstruction::Rl => self.rl(target),
                        ArithmeticInstruction::Rrc => self.rrc(target),
                        ArithmeticInstruction::Rlc => self.rlc(target),
                        ArithmeticInstruction::Sra => self.sra(target),
                        ArithmeticInstruction::Sla => self.sla(target),
                        ArithmeticInstruction::Swap => self.swap(target),

                        _ => Err(Instruction::Arithmetic(instruction, opt_imm, opt_target))?,
                    },
                    Value::Word(value) => match instruction {
                        ArithmeticInstruction::AddHl => self.addhl(value),

                        ArithmeticInstruction::Inc => self.inc(target),
                        ArithmeticInstruction::Dec => self.dec(target),

                        _ => Err(Instruction::Arithmetic(instruction, opt_imm, opt_target))?,
                    },
                }
            }
            (None, None) => match instruction {
                ArithmeticInstruction::Ccf => self.ccf(),
                ArithmeticInstruction::Scf => self.scf(),
                ArithmeticInstruction::Rra => self.rra(),
                ArithmeticInstruction::Rla => self.rla(),
                ArithmeticInstruction::Rrca => self.rrca(),
                ArithmeticInstruction::Rlca => self.rlca(),
                ArithmeticInstruction::Cpl => self.cpl(),
                ArithmeticInstruction::Daa => self.daa(),

                _ => Err(Instruction::Arithmetic(instruction, opt_imm, opt_target))?,
            },
        }

        Ok(())
    }

    fn get_arithmetic_target(&self, target: ArithmeticTarget) -> Value {
        match target {
            ArithmeticTarget::A => Value::Byte(self.reg.a),
            ArithmeticTarget::B => Value::Byte(self.reg.b),
            ArithmeticTarget::C => Value::Byte(self.reg.c),
            ArithmeticTarget::D => Value::Byte(self.reg.d),
            ArithmeticTarget::E => Value::Byte(self.reg.e),
            ArithmeticTarget::H => Value::Byte(self.reg.h),
            ArithmeticTarget::L => Value::Byte(self.reg.l),

            ArithmeticTarget::BC => Value::Word(self.reg.get_bc()),
            ArithmeticTarget::DE => Value::Word(self.reg.get_de()),
            ArithmeticTarget::HL => Value::Word(self.reg.get_hl()),

            ArithmeticTarget::SP => Value::Word(self.reg.sp),
            ArithmeticTarget::PC => Value::Word(self.reg.pc),

            ArithmeticTarget::HlAddr => Value::Byte(self.mem_bus.readb(self.reg.get_hl())),
        }
    }

    fn read_byte_from_arithmetic_target(&self, target: ArithmeticTarget) -> u8 {
        match target {
            ArithmeticTarget::A => self.reg.a,
            ArithmeticTarget::B => self.reg.b,
            ArithmeticTarget::C => self.reg.c,
            ArithmeticTarget::D => self.reg.d,
            ArithmeticTarget::E => self.reg.e,
            ArithmeticTarget::H => self.reg.h,
            ArithmeticTarget::L => self.reg.l,

            ArithmeticTarget::BC => self.reg.b,
            ArithmeticTarget::DE => self.reg.d,
            ArithmeticTarget::HL => self.reg.h,

            ArithmeticTarget::HlAddr => self.mem_bus.readb(self.reg.get_hl()),

            ArithmeticTarget::SP => (self.reg.sp & 0xFF) as u8, //should not happen
            ArithmeticTarget::PC => (self.reg.sp & 0xFF) as u8, //should not happen
        }
    }

    fn write_byte_to_arithmetic_target(&mut self, target: ArithmeticTarget, byte: u8) {
        match target {
            ArithmeticTarget::A => self.reg.a = byte,
            ArithmeticTarget::B => self.reg.b = byte,
            ArithmeticTarget::C => self.reg.c = byte,
            ArithmeticTarget::D => self.reg.d = byte,
            ArithmeticTarget::E => self.reg.e = byte,
            ArithmeticTarget::H => self.reg.h = byte,
            ArithmeticTarget::L => self.reg.l = byte,

            ArithmeticTarget::BC => self.reg.b = byte,
            ArithmeticTarget::DE => self.reg.d = byte,
            ArithmeticTarget::HL => self.reg.h = byte,
            ArithmeticTarget::SP => (), //should not happen
            ArithmeticTarget::PC => (), //should not happen

            ArithmeticTarget::HlAddr => self.mem_bus.writeb(self.reg.get_hl(), byte),
        }
    }

    fn write_value_to_arithmetic_target(&mut self, target: ArithmeticTarget, value: Value) {
        match target {
            ArithmeticTarget::A => self.reg.a = value.first_byte(),
            ArithmeticTarget::B => self.reg.b = value.first_byte(),
            ArithmeticTarget::C => self.reg.c = value.first_byte(),
            ArithmeticTarget::D => self.reg.d = value.first_byte(),
            ArithmeticTarget::E => self.reg.e = value.first_byte(),
            ArithmeticTarget::H => self.reg.h = value.first_byte(),
            ArithmeticTarget::L => self.reg.l = value.first_byte(),

            ArithmeticTarget::BC => self.reg.set_bc(value.into()),
            ArithmeticTarget::DE => self.reg.set_de(value.into()),
            ArithmeticTarget::HL => self.reg.set_hl(value.into()),
            ArithmeticTarget::SP => self.reg.sp = value.into(),
            ArithmeticTarget::PC => self.reg.sp = value.into(),

            ArithmeticTarget::HlAddr => self.mem_bus.writeb(self.reg.get_hl(), value.first_byte()),
        }
    }

    fn inc(&mut self, target: ArithmeticTarget) {
        let value = self.get_arithmetic_target(target);
        let mut result = value;
        let _ = result.overflowing_inc();
        self.write_value_to_arithmetic_target(target, result);

        if let Value::Byte(val) = value {
            self.reg.set_zero(value.is_zero());
            self.reg.set_substract(false);
            self.reg.set_half_carry(((val & 0xF) + 1) > 0xF);
        }
    }

    fn dec(&mut self, target: ArithmeticTarget) {
        let value = self.get_arithmetic_target(target);
        let mut result = value;
        let _ = result.overflowing_dec();
        self.write_value_to_arithmetic_target(target, result);

        if let Value::Byte(val) = value {
            self.reg.set_zero(value.is_zero());
            self.reg.set_substract(false);
            self.reg.set_half_carry((val & 0xF) == 0);
        }
    }

    // -- 08 bits source
    fn add(&mut self, value: u8) {
        let (result, overflow) = self.reg.a.overflowing_add(value);
        self.reg.set_zero(result == 0);
        self.reg.set_substract(false);
        self.reg.set_carry(overflow);
        self.reg
            .set_half_carry(((self.reg.a & 0xF) + (value & 0xF)) > 0xF);
        self.reg.a = result;
    }

    fn adc(&mut self, value: u8) {
        let carry = if self.reg.get_carry() { 1 } else { 0 };
        let (intermediate, overflow1) = self.reg.a.overflowing_add(value);
        let (result, overflow2) = intermediate.overflowing_add(carry);

        self.reg.set_zero(result == 0);
        self.reg.set_substract(false);
        self.reg.set_carry(overflow1 || overflow2);
        self.reg
            .set_half_carry(((self.reg.a & 0xF) + (value & 0xF) + carry) > 0xF);

        self.reg.a = result;
    }

    fn sub(&mut self, value: u8) {
        let (result, overflow) = self.reg.a.overflowing_sub(value);

        self.reg.set_zero(result == 0);
        self.reg.set_substract(true);
        self.reg.set_carry(overflow);
        self.reg.set_half_carry((self.reg.a & 0xF) < (value & 0xF));

        self.reg.a = result;
    }

    fn sbc(&mut self, value: u8) {
        let carry = if self.reg.get_carry() { 1 } else { 0 };
        let (intermediate, overflow1) = self.reg.a.overflowing_sub(value);
        let (result, overflow2) = intermediate.overflowing_sub(carry);

        self.reg.set_zero(result == 0);
        self.reg.set_substract(true);
        self.reg.set_carry(overflow1 || overflow2);
        self.reg
            .set_half_carry((self.reg.a & 0xF) < ((value & 0xF) + carry));

        self.reg.a = result;
    }

    fn daa(&mut self) {
        let mut adj:u8 = 0;
        if self.reg.get_substract() {
            if self.reg.get_half_carry() {
                adj += 0x6;
            }
            if self.reg.get_carry() {
                adj += 0x60;
            }
            self.reg.a = self.reg.a.wrapping_sub(adj)
        }else{
            if self.reg.get_half_carry() || (self.reg.a & 0xF) > 9{
                adj += 0x6;
            }
            if self.reg.get_carry() || self.reg.a > 0x99{
                adj += 0x60;
                self.reg.set_carry(true);
            }
            self.reg.a = self.reg.a.wrapping_add(adj)
        }
    }

    fn and(&mut self, value: u8) {
        self.reg.a &= value;

        self.reg.set_zero(self.reg.a == 0);
        self.reg.set_substract(false);
        self.reg.set_half_carry(true);
        self.reg.set_carry(false);
    }

    fn or(&mut self, value: u8) {
        self.reg.a |= value;

        self.reg.set_zero(self.reg.a == 0);
        self.reg.set_substract(false);
        self.reg.set_half_carry(false);
        self.reg.set_carry(false);
    }

    fn xor(&mut self, value: u8) {
        self.reg.a ^= value;

        self.reg.set_zero(self.reg.a == 0);
        self.reg.set_substract(false);
        self.reg.set_half_carry(false);
        self.reg.set_carry(false);
    }

    fn cp(&mut self, value: u8) {
        let (result, overflow) = self.reg.a.overflowing_sub(value);

        self.reg.set_zero(result == 0);
        self.reg.set_substract(true);
        self.reg.set_carry(overflow);
        self.reg.set_half_carry((self.reg.a & 0xF) < (value & 0xF));
    }

    fn bit(&mut self, imm3b: Immediate3Bits, value: u8) {
        let bit: u8 = imm3b.into();
        let mask = 0b1 << bit;
        let zeroed = value & mask == 0;

        self.reg.set_zero(zeroed);
        self.reg.set_substract(false);
        self.reg.set_half_carry(true);
    }

    // -- 16 bits source
    fn addhl(&mut self, value: u16) {
        let hl = self.reg.get_hl();
        let (result, overflow) = hl.overflowing_add(value);

        self.reg.set_substract(false);
        self.reg.set_carry(overflow);
        self.reg
            .set_half_carry(((hl & 0xFFF) + (value & 0xFFF)) > 0xFFF);

        self.reg.set_hl(result);
    }

    // -- No source
    fn rra(&mut self) {
        let old_carry = self.reg.get_carry();
        let new_carry = (self.reg.a & 0x01) != 0;
        self.reg.a >>= 1;
        if old_carry {
            self.reg.a |= 0x80;
        }

        self.reg.set_carry(new_carry);
        self.reg.set_zero(false);
        self.reg.set_substract(false);
        self.reg.set_half_carry(false);
    }

    fn rla(&mut self) {
        let old_carry = self.reg.get_carry();
        let new_carry = (self.reg.a & 0x80) != 0;
        self.reg.a <<= 1;
        if old_carry {
            self.reg.a |= 0x01;
        }

        self.reg.set_carry(new_carry);
        self.reg.set_zero(false);
        self.reg.set_substract(false);
        self.reg.set_half_carry(false);
    }

    fn rrca(&mut self) {
        let new_carry = (self.reg.a & 0x01) != 0;
        self.reg.a = self.reg.a.rotate_right(1);

        self.reg.set_carry(new_carry);
        self.reg.set_zero(false);
        self.reg.set_substract(false);
        self.reg.set_half_carry(false);
    }

    fn rlca(&mut self) {
        let new_carry = (self.reg.a & 0x80) != 0;
        self.reg.a = self.reg.a.rotate_left(1);

        self.reg.set_carry(new_carry);
        self.reg.set_zero(false);
        self.reg.set_substract(false);
        self.reg.set_half_carry(false);
    }

    fn cpl(&mut self) {
        self.reg.a = !self.reg.a;

        self.reg.set_substract(true);
        self.reg.set_half_carry(true);
    }

    // -- 8bits target
    fn res(&mut self, imm3b: Immediate3Bits, target: ArithmeticTarget) {
        let bit: u8 = imm3b.into();
        let mask = !(0b1 << bit);
        let value = self.read_byte_from_arithmetic_target(target);
        let result = value & mask;
        self.write_byte_to_arithmetic_target(target, result);
    }

    fn set(&mut self, imm3b: Immediate3Bits, target: ArithmeticTarget) {
        let bit: u8 = imm3b.into();
        let mask = 0b1 << bit;
        let value = self.read_byte_from_arithmetic_target(target);
        let result = value | mask;
        self.write_byte_to_arithmetic_target(target, result);
    }

    fn srl(&mut self, target: ArithmeticTarget) {
        let value = self.read_byte_from_arithmetic_target(target);
        let (result, overflow) = value.overflowing_shr(1);
        self.write_byte_to_arithmetic_target(target, result);

        self.reg.set_zero(result == 0);
        self.reg.set_substract(false);
        self.reg.set_half_carry(false);
        self.reg.set_carry(overflow);
    }

    fn rr(&mut self, target: ArithmeticTarget) {
        let value = self.read_byte_from_arithmetic_target(target);
        let old_carry = self.reg.get_carry();
        let new_carry = (value & 0x01) != 0;
        let result = if old_carry {
            (value >> 1) | 0x80
        } else {
            value >> 1
        };

        self.write_byte_to_arithmetic_target(target, result);

        self.reg.set_zero(result == 0);
        self.reg.set_substract(false);
        self.reg.set_half_carry(false);
        self.reg.set_carry(new_carry);
    }

    fn rl(&mut self, target: ArithmeticTarget) {
        let value = self.read_byte_from_arithmetic_target(target);
        let old_carry = self.reg.get_carry();
        let new_carry = (value & 0x80) != 0;
        let result = if old_carry {
            (value << 1) | 0x01
        } else {
            value << 1
        };

        self.write_byte_to_arithmetic_target(target, result);

        self.reg.set_zero(result == 0);
        self.reg.set_substract(false);
        self.reg.set_half_carry(false);
        self.reg.set_carry(new_carry);
    }

    fn rrc(&mut self, target: ArithmeticTarget) {
        let value = self.read_byte_from_arithmetic_target(target);
        let result = value.rotate_right(1);
        self.write_byte_to_arithmetic_target(target, result);

        self.reg.set_zero(result == 0);
        self.reg.set_substract(false);
        self.reg.set_half_carry(false);
        self.reg.set_carry(value & 0b1 == 1);
    }

    fn rlc(&mut self, target: ArithmeticTarget) {
        let value = self.read_byte_from_arithmetic_target(target);
        let result = value.rotate_left(1);
        self.write_byte_to_arithmetic_target(target, result);

        self.reg.set_zero(result == 0);
        self.reg.set_substract(false);
        self.reg.set_half_carry(false);
        self.reg.set_carry(result & 0x1 == 1);
    }

    fn sra(&mut self, target: ArithmeticTarget) {
        let value = self.read_byte_from_arithmetic_target(target);
        let (val_shr, overflow) = value.overflowing_shr(1);
        let result = val_shr | (value & 0x80);
        self.write_byte_to_arithmetic_target(target, result);

        self.reg.set_zero(result == 0);
        self.reg.set_substract(false);
        self.reg.set_half_carry(false);
        self.reg.set_carry(overflow);
    }

    fn sla(&mut self, target: ArithmeticTarget) {
        let value = self.read_byte_from_arithmetic_target(target);
        let (result, overflow) = value.overflowing_shl(1);
        self.write_byte_to_arithmetic_target(target, result);

        self.reg.set_zero(result == 0);
        self.reg.set_substract(false);
        self.reg.set_half_carry(false);
        self.reg.set_carry(overflow);
    }

    fn swap(&mut self, target: ArithmeticTarget) {
        let value = self.read_byte_from_arithmetic_target(target);
        let (n1, n2) = ((value & 0xF0) >> 4, value & 0x0F);
        let result = n1 | (n2 << 4);
        self.write_byte_to_arithmetic_target(target, result);

        self.reg.set_zero(result == 0);
        self.reg.set_substract(false);
        self.reg.set_half_carry(false);
        self.reg.set_carry(false);
    }

    // -- flags
    fn ccf(&mut self) {
        self.reg.set_carry(!self.reg.get_carry());

        self.reg.set_substract(false);
        self.reg.set_half_carry(false);
    }

    fn scf(&mut self) {
        self.reg.set_substract(false);
        self.reg.set_half_carry(false);
        self.reg.set_carry(true);
    }
}
