use crate::{
    cpu::{dispatch::OpCode, Cpu},
    memory::{ram::Ram, MemoryMapping},
};

// TODO: add option to set initial registers

/// Test the execution of an opcode
///
/// Executes the opcode and checks the following:
///  - instruction execites in the expected amount of cycles
///  - after the instruction PC is incremented by 1 + amount of arguments
///  - runs the provided verification function at the end
///
/// This function might not be used to test jumps
pub fn test_opcode<F: FnOnce(&Cpu, &mut MemoryMapping)>(
    opcode: OpCode,
    arguments: impl AsRef<[u8]>,
    additional_values: impl AsRef<[(u16, u8)]>,
    expected_cycles: u8,
    verify: F,
) {
    let arguments = arguments.as_ref();
    let additional_values = additional_values.as_ref();

    let mut ram = Ram::new();
    let mut cpu = Cpu::new();
    let mut memory = MemoryMapping { ram: &mut ram };
    cpu.program_counter = 0;

    memory.store(0x0, opcode.into());

    for (i, &value) in arguments.iter().enumerate() {
        let addr = i
            .checked_add(1)
            .and_then(|i| i.try_into().ok())
            .expect("argument list length is too large");

        memory.store(addr, value);
    }

    for &(addr, value) in additional_values {
        memory.store(addr, value);
    }

    let argument_len: u16 = arguments
        .len()
        .try_into()
        .expect("Opcode argument list must fit into u16");

    let expected_pc = 1 + argument_len;

    // execute
    for i in 0..expected_cycles {
        cpu.run_cycle(&mut memory);

        if i != expected_cycles - 1 {
            assert_ne!(cpu.current_cycle, 0, "instruction ended prematurely");
        } else {
            assert_eq!(
                cpu.current_cycle, 0,
                "instruction hasn't ended when expected"
            );
        }
    }

    assert_eq!(
        cpu.program_counter, expected_pc,
        "instruction pointer incorrectly set after instruction"
    );

    verify(&cpu, &mut memory);
}
