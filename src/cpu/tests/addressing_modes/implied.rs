use crate::{
    cpu::{Cpu, executor::Executor, opcode::Opcode, tests::TestMemory},
    memory::Memory,
};

use super::prepare::OPCODE_ADDR;

pub trait TestImpliedInstruction {
    fn prepare(cpu: &mut Cpu, arg: u8);

    fn verify(cpu: &Cpu, arg: u8);
}

pub trait TestImplied: TestImpliedInstruction {
    const OPCODE: Opcode;

    fn test_implied() {
        const EXPECTED_CLOCK_CYCLES: u64 = 2;
        const INSTRUCTION_LENGTH: u16 = 1;

        for arg in 0..u8::MAX {
            let mut cpu = Cpu::new();
            let mut memory = TestMemory::new();

            cpu.pc = OPCODE_ADDR;
            Self::prepare(&mut cpu, arg);
            memory.store(OPCODE_ADDR, Self::OPCODE as u8);

            let mut executor = Executor {
                cpu: &mut cpu,
                memory: &mut memory,
            };

            executor.execute_next_instruction();

            assert_eq!(
                executor.cpu.clock_cycle_count,
                EXPECTED_CLOCK_CYCLES,
                "instruction {:?} must take {EXPECTED_CLOCK_CYCLES} clock cycles",
                Self::OPCODE,
            );

            assert_eq!(
                executor.cpu.pc,
                OPCODE_ADDR + INSTRUCTION_LENGTH,
                "after instruction {:?} PC must be incremented {INSTRUCTION_LENGTH} times",
                Self::OPCODE
            );

            Self::verify(executor.cpu, arg);
        }
    }
}
