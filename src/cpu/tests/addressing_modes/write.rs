use crate::{
    cpu::{
        executor::Executor,
        instructions::opcode::OpCode,
        tests::{
            addressing_modes::prepare::{AddressingMode, OPCODE_ADDR},
            TestMemory,
        },
        Cpu,
    },
    memory::Memory,
};

pub trait TestWriteInstruction {
    fn prepare(cpu: &mut Cpu, arg: u8);
    fn expected_value(cpu: &Cpu) -> u8;
}

pub trait TestWriteZeropage: TestWriteInstruction {
    const OPCODE: OpCode;

    fn test_zeropage() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::Zeropage,
            expected_clock_cycles: 3,
        });
    }
}

pub trait TestWriteZeropageX: TestWriteInstruction {
    const OPCODE: OpCode;

    fn test_zeropage_x() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::ZeropageX,
            expected_clock_cycles: 4,
        });
    }

    fn test_zeropage_x_overflow() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::ZeropageXOverflow,
            expected_clock_cycles: 4,
        });
    }
}

pub trait TestWriteZeropageY: TestWriteInstruction {
    const OPCODE: OpCode;

    fn test_zeropage_y() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::ZeropageY,
            expected_clock_cycles: 4,
        });
    }

    fn test_zeropage_y_overflow() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::ZeropageYOverflow,
            expected_clock_cycles: 4,
        });
    }
}

pub trait TestWriteAbsolute: TestWriteInstruction {
    const OPCODE: OpCode;

    fn test_absolute() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::Absolute,
            expected_clock_cycles: 4,
        });
    }
}

pub trait TestWriteAbsoluteX: TestWriteInstruction {
    const OPCODE: OpCode;

    fn test_absolute_x() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::AbsoluteX,
            expected_clock_cycles: 5,
        });
    }

    fn test_absolute_x_overflow() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::AbsoluteXOverflow,
            expected_clock_cycles: 5,
        });
    }
}

pub trait TestWriteAbsoluteY: TestWriteInstruction {
    const OPCODE: OpCode;

    fn test_absolute_y() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::AbsoluteY,
            expected_clock_cycles: 5,
        });
    }

    fn test_absolute_y_overflow() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::AbsoluteYOverflow,
            expected_clock_cycles: 5,
        });
    }
}

pub trait TestWriteIndirectX: TestWriteInstruction {
    const OPCODE: OpCode;

    fn test_indirect_x() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::IndirectX,
            expected_clock_cycles: 6,
        });
    }

    fn test_indirect_x_overflow() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::IndirectXOverflow,
            expected_clock_cycles: 6,
        });
    }

    fn test_indirect_x_page_split() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::IndirectXPageSplit,
            expected_clock_cycles: 6,
        });
    }
}

pub trait TestWriteIndirectY: TestWriteInstruction {
    const OPCODE: OpCode;

    fn test_indirect_y() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::IndirectY,
            expected_clock_cycles: 6,
        });
    }

    fn test_indirect_y_overflow() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::IndirectYOverflow,
            expected_clock_cycles: 6,
        });
    }

    fn test_indirect_y_page_split() {
        test_instruction::<Self>(TestInstructionOptions {
            opcode: Self::OPCODE,
            addressing_mode: AddressingMode::IndirectYPageSplit,
            expected_clock_cycles: 6,
        });
    }
}

#[derive(Debug, Clone, Copy)]
struct TestInstructionOptions {
    opcode: OpCode,
    addressing_mode: AddressingMode,
    expected_clock_cycles: u64,
}

fn test_instruction<I: TestWriteInstruction + ?Sized>(
    TestInstructionOptions {
        opcode,
        addressing_mode,
        expected_clock_cycles,
    }: TestInstructionOptions,
) {
    for arg in u8::MIN..u8::MAX {
        let mut cpu = Cpu::new();
        let mut memory = TestMemory::new();

        cpu.pc = OPCODE_ADDR;
        I::prepare(&mut cpu, arg);
        memory.store(OPCODE_ADDR, opcode as u8);

        let mut executor = Executor {
            cpu: &mut cpu,
            memory: &mut memory,
        };

        addressing_mode.prepare(&mut executor);
        executor.execute_next_instruction();
        assert_eq!(
            executor.memory.load(addressing_mode.value_addr()),
            I::expected_value(executor.cpu),
            "value written must match the expected value"
        );
        assert_eq!(
            executor.cpu.clock_cycle_count, expected_clock_cycles,
            "instruction {opcode:?} must take {expected_clock_cycles} clock cycles"
        );
        let instruction_length = addressing_mode.instruction_length();
        assert_eq!(
            executor.cpu.pc,
            OPCODE_ADDR + instruction_length,
            "after instruction {opcode:?} PC must be incremented {instruction_length} times"
        );
    }
}
