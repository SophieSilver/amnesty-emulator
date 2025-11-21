use crate::{
    cpu::{
        Cpu, StatusFlags,
        executor::Executor,
        opcode::Opcode,
        tests::{TestMemory, flags::check_nz_flags},
    },
    memory::Memory,
};

use super::prepare::{AddressingMode, OPCODE_ADDR};

pub trait TestRmwInstruction {
    fn verify(cpu: &Cpu, arg: u8, previous_carry: bool) -> u8;
}

pub trait TestRmwAccumulator: TestRmwInstruction {
    const OPCODE: Opcode;

    fn test_accumulator() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::Accumulator,
            expected_clock_cycles: 2,
        });
    }
}

pub trait TestRmwZeropage: TestRmwInstruction {
    const OPCODE: Opcode;

    fn test_zeropage() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::Zeropage,
            expected_clock_cycles: 5,
        });
    }
}

pub trait TestRmwZeropageX: TestRmwInstruction {
    const OPCODE: Opcode;

    fn test_zeropage_x() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::ZeropageX,
            expected_clock_cycles: 6,
        });
    }

    fn test_zeropage_x_overflow() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::ZeropageXOverflow,
            expected_clock_cycles: 6,
        });
    }
}

pub trait TestRmwAbsolute: TestRmwInstruction {
    const OPCODE: Opcode;

    fn test_absolute() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::Absolute,
            expected_clock_cycles: 6,
        });
    }
}

pub trait TestRmwAbsoluteX: TestRmwInstruction {
    const OPCODE: Opcode;

    fn test_absolute_x() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::AbsoluteX,
            expected_clock_cycles: 7,
        });
    }

    fn test_absolute_x_overflow() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::AbsoluteXOverflow,
            expected_clock_cycles: 7,
        });
    }
}

#[derive(Debug, Clone, Copy)]
struct TestInstructionOptions {
    opcode: Opcode,
    addressing_mode: AddressingMode,
    expected_clock_cycles: u64,
}

fn test_instruction<I: TestRmwInstruction + ?Sized>(
    TestInstructionOptions {
        opcode,
        addressing_mode,
        expected_clock_cycles,
    }: TestInstructionOptions,
) {
    for arg in u8::MIN..u8::MAX {
        for carry in [false, true] {
            let mut cpu = Cpu::new();
            let mut memory = TestMemory::new();

            cpu.pc = OPCODE_ADDR;
            cpu.flags.set(StatusFlags::CARRY, carry);
            memory.store(OPCODE_ADDR, opcode as u8);

            let mut executor = Executor {
                cpu: &mut cpu,
                memory: &mut memory,
            };

            if addressing_mode == AddressingMode::Accumulator {
                executor.cpu.a = arg;
            } else {
                executor.memory.store(addressing_mode.value_addr(), arg);
            }

            addressing_mode.prepare(&mut executor);
            executor.execute_next_instruction();

            let expected_value = I::verify(executor.cpu, arg, carry);

            let gotten_value = if addressing_mode == AddressingMode::Accumulator {
                cpu.a
            } else {
                memory.load(addressing_mode.value_addr())
            };

            assert_eq!(expected_value, gotten_value);
            check_nz_flags(gotten_value, cpu.flags);
            assert_eq!(
                cpu.clock_cycle_count, expected_clock_cycles,
                "instruction {opcode:?} must take {expected_clock_cycles} clock cycles"
            );
            let instruction_length = addressing_mode.instruction_length();
            assert_eq!(
                cpu.pc,
                OPCODE_ADDR + instruction_length,
                "after instruction {opcode:?} PC must be incremented {instruction_length} times"
            );
        }
    }
}
