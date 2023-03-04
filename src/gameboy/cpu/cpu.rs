use anyhow::{bail, Result};

use super::super::bus::bus::{Bus, BusIterator};
use super::instruction::{Instruction, Operand};
use super::regs::{Flag, Register, RegisterFile};

/// Return type of CPU::op_* functions
type CPUOpResult = Result<OpOk>;

/// Function signature of CPU::op_* functions
pub type CPUOpFn = fn(&mut CPU, &Instruction) -> CPUOpResult;

/// Result of a successful CPU::op_* function.
pub struct OpOk {
    /// New program counter
    pc: u16,

    /// Cycles taken
    cycles: usize,
}

impl OpOk {
    /// Normal successful op, moves PC to next
    /// instruction and always fixed cycle count.
    #[inline(always)]
    fn ok(cpu: &CPU, instr: &Instruction) -> Self {
        Self {
            pc: cpu.regs.pc + instr.len as u16,
            cycles: instr.def.cycles[0].into(),
        }
    }
}

/// Gameboy CPU
pub struct CPU {
    pub bus: Box<dyn Bus>,
    pub regs: RegisterFile,

    /// Total amount of cycles
    cycles: usize,
}

impl CPU {
    pub fn new(bus: Box<dyn Bus>) -> Self {
        Self {
            bus,
            regs: RegisterFile::new(),
            cycles: 0,
        }
    }

    pub fn peek_next_instr(&self) -> Result<Instruction> {
        let mut busiter = BusIterator::new_from(&self.bus, self.regs.pc);
        Instruction::decode(&mut busiter)
    }

    pub fn step(&mut self) -> Result<()> {
        let instr = self.peek_next_instr()?;
        let result = (instr.def.func)(self, &instr)?;
        self.regs.pc = result.pc;
        self.cycles += result.cycles;
        Ok(())
    }

    pub fn get_cycles(&self) -> usize {
        self.cycles
    }

    pub fn op_set(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_res(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_srl(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_swap(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_sla(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_sra(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_bit(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_rl(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_rla(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_rlc(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_rlca(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_rr(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_rra(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_rrc(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_rrca(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_ei(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_di(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_rst(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_nop(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_stop(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_halt(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    /// LD - Load Register
    pub fn op_ld(&mut self, instr: &Instruction) -> CPUOpResult {
        // Source operand
        let val: u16 = match &instr.def.operands[1] {
            // LD _, imm16
            Operand::Immediate16 => instr.imm16(0)?,
            // LD _, reg
            Operand::Register(reg) => self.regs.read(*reg),
            _ => todo!(),
        };

        // Destination operand
        match instr.def.operands[0] {
            // LD reg, _
            Operand::Register(dest) => self.regs.write(dest, val)?,
            // LD (reg-), _
            Operand::RegisterIndirectDec(dest) => {
                self.bus.write(self.regs.read_dec(dest)?, val.try_into()?)
            }
            _ => bail!("Invalid first operand: {:?}", instr.def.operands[0]),
        }

        Ok(OpOk::ok(self, instr))
    }

    pub fn op_ldh(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_scf(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_ccf(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_cp(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_cpl(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_or(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    /// XOR - Bitwise XOR
    pub fn op_xor(&mut self, instr: &Instruction) -> CPUOpResult {
        let a = self.regs.read8(Register::A)?;
        let val = match instr.def.operands[0] {
            // XOR reg
            Operand::Register(r) => self.regs.read8(r)?,
            _ => todo!(),
        };
        let result = a ^ val;
        self.regs.write(Register::A, result.into())?;
        self.regs.write_flags(&[(Flag::Z, result == 0)]);

        Ok(OpOk::ok(self, instr))
    }

    pub fn op_and(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_push(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_pop(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_adc(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_daa(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_add(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_sub(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_dec(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_inc(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_jr(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_jr_nc(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_jr_nz(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_jr_z(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_jp(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_jp_nc(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_jp_nz(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_jp_z(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_call(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_call_nc(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_call_nz(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_call_z(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_ret(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_ret_nc(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_ret_nz(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_ret_z(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_reti(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_sbc(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_prefix_cb(&mut self, _instr: &Instruction) -> CPUOpResult {
        todo!();
    }

    pub fn op_invalid(&mut self, _instr: &Instruction) -> CPUOpResult {
        panic!("Invalid opcode");
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::bus::testbus::Testbus;
    use super::*;

    fn cpu(code: &[u8]) -> CPU {
        let bus = Testbus::from(code);
        CPU::new(Box::new(bus))
    }

    fn cpu_run(cpu: &mut CPU) {
        cpu.step().unwrap();
    }

    fn run(code: &[u8]) -> CPU {
        let mut cpu = cpu(code);
        cpu_run(&mut cpu);
        cpu
    }

    #[test]
    fn op_ld_reg_imm16() {
        let cpu = run(&[0x31, 0x34, 0x12]); // LD SP,0x1234
        assert_eq!(cpu.regs.sp, 0x1234);
    }

    #[test]
    fn op_xor_reg() {
        let mut c = cpu(&[0xA8]); // XOR A
        c.regs.a = 0x55;
        c.regs.b = 0xAA;
        cpu_run(&mut c);
        assert_eq!(c.regs.a, 0xFF);
        assert!(!c.regs.test_flag(Flag::Z));

        let mut c = cpu(&[0xA8]); // XOR A
        c.regs.a = 0xAA;
        c.regs.b = 0xAA;
        cpu_run(&mut c);
        assert_eq!(c.regs.a, 0x00);
        assert!(c.regs.test_flag(Flag::Z));
    }

    #[test]
    fn op_ld_ind_reg_dec_reg() {
        let mut c = cpu(&[0x32]); // LD (HL-),A
        (c.regs.h, c.regs.l) = (0x11, 0x22);
        c.regs.a = 0x5A;
        cpu_run(&mut c);
        assert_eq!((c.regs.h, c.regs.l), (0x11, 0x21));
        assert_eq!(c.bus.read(0x1122), 0x5A);
    }

    #[test]
    fn op_ld_reg_reg() {
        let mut c = cpu(&[0x78]); // LD A,B
        c.regs.b = 0x55;
        cpu_run(&mut c);
        assert_eq!(c.regs.a, 0x55);
    }
}
