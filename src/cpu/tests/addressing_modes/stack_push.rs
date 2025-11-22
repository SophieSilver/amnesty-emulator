use crate::{
    cpu::{
        Cpu,
        executor::Executor,
        opcode::Opcode,
        tests::{TestMemory, addressing_modes::prepare::OPCODE_ADDR},
    },
    memory::Memory,
};

pub trait TestStackPushInstruction {
    fn prepare(cpu: &mut Cpu, arg: u8);

    fn verify(cpu: &mut Cpu, value_pushed: u8);
}

pub trait TestStackPush: TestStackPushInstruction {
    const OPCODE: Opcode;

    fn test_stack_push() {
        const EXPECTED_CLOCK_CYCLES: u64 = 3;
        const INSTRUCTION_LENGTH: u16 = 1;

        for arg in 0..u8::MAX {
            let mut cpu = Cpu::new();
            let mut memory = TestMemory::new();

            cpu.pc = OPCODE_ADDR;
            Self::prepare(&mut cpu, arg);
            cpu.sp = 0xF7;
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

            let value_pushed = executor
                .memory
                .load(0x100 << 8 | executor.cpu.sp.wrapping_add(1) as u16);

            Self::verify(executor.cpu, value_pushed);
        }
    }
}
