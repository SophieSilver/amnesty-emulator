use crate::{
    cpu::{dispatch::OpCode, Cpu},
    memory::{ram::Ram, MemoryMapping},
};

/// Options for quickly and conveniently testing opcodes
///
/// ## Test structure
/// The test will:
/// - load the opcode and its arguments at address 0x0200
/// - load additional values at specified addresses
/// - call the provided `prepare` closure to set up initial CPU state
/// - set PC to 0x0200 and run the cpu for `expected_cycles`
///
/// ## Checks
/// The test will fail (panic) if:
/// - The instruction terminates too early or does not terminate in `expected_cycles`
///     (by checking `cpu.current_cycle`)
/// - `check_pc` is `true` AND
///     after executing the instruction the PC isn't equal to `0x200 + arguments.len()`
/// - provided `verify` closure panics
///
#[derive(Debug)]
pub struct TestOpcodeOptions<'a, PrepareFunc, VerifyFunc>
where
    PrepareFunc: FnOnce(&mut Cpu),
    VerifyFunc: FnOnce(&mut Cpu, &mut MemoryMapping),
{
    /// What opcode to test
    ///
    /// This will be loaded at address `0x0200`
    pub opcode: OpCode,

    /// Arguments that follow the opcode
    ///
    /// They will be loaded at addresses `0x1..arguments.len()`
    pub arguments: &'a [u8],

    /// Additional values to place at specified memory addresses
    pub additional_values: &'a [(u16, u8)],

    /// How many cycles is the instruction expected to take
    pub expected_cycles: u8,

    /// Whether to check that the PC was incrementes 1 + `arguments.len()` times
    ///
    /// Turn off when testing instructions that modify PC, like jumps
    pub check_pc: bool,

    /// Function to run before the test,
    ///
    /// Use to, e.g., set up CPU registers
    pub prepare: PrepareFunc,

    /// Function to run after the test
    ///
    /// Use to verify that the instruction did everything correctly
    pub verify: VerifyFunc,
}

// using a function pointer here for less headache with existential types
impl<'a, VerifyFunc> TestOpcodeOptions<'a, fn(&mut Cpu), VerifyFunc>
where
    VerifyFunc: FnOnce(&mut Cpu, &mut MemoryMapping),
{
    #[must_use]
    pub fn new(opcode: OpCode, expected_cycles: u8, verify: VerifyFunc) -> Self {
        Self {
            opcode,
            arguments: &[],
            additional_values: &[],
            expected_cycles,
            check_pc: true,
            prepare: |_| {},
            verify,
        }
    }
}

impl<'a, PrepareFunc, VerifyFunc> TestOpcodeOptions<'a, PrepareFunc, VerifyFunc>
where
    PrepareFunc: FnOnce(&mut Cpu),
    VerifyFunc: FnOnce(&mut Cpu, &mut MemoryMapping),
{
    /// Run the test with current Options
    pub fn test(self) {
        const OPCODE_ADDR: u16 = 0x0200;

        assert!(self.arguments.len() < u16::MAX as usize);

        // initialize the CPU and the memory
        let mut ram = Ram::new();
        let mut cpu = Cpu::new();
        let mut memory = MemoryMapping { ram: &mut ram };

        // prepare memory
        memory.store(OPCODE_ADDR, self.opcode.into());

        for (i, &value) in self.arguments.iter().enumerate() {
            // address wrapped in 0..u16::MAX range
            let addr = (i + OPCODE_ADDR as usize + 1) % u16::MAX as usize;
            let addr = addr as u16;

            memory.store(addr, value);
        }

        for &(addr, value) in self.additional_values {
            memory.store(addr, value);
        }

        // prepare the cpu
        (self.prepare)(&mut cpu);

        // execute
        cpu.program_counter = OPCODE_ADDR;
        for i in 0..self.expected_cycles {
            cpu.run_cycle(&mut memory);

            if i != self.expected_cycles - 1 {
                assert_ne!(cpu.current_cycle, 0, "instruction ended prematurely");
            } else {
                assert_eq!(
                    cpu.current_cycle, 0,
                    "instruction hasn't ended when expected"
                );
            }
        }

        if self.check_pc {
            let argument_len = self.arguments.len() as u16;
            let expected_pc = OPCODE_ADDR + 1 + argument_len;
            assert_eq!(
                cpu.program_counter, expected_pc,
                "instruction pointer incorrectly set after instruction"
            );
        }

        (self.verify)(&mut cpu, &mut memory);
    }

    #[must_use]
    pub fn with_arguments(self, arguments: &'a impl AsRef<[u8]>) -> Self {
        Self {
            arguments: arguments.as_ref(),
            ..self
        }
    }

    #[must_use]
    pub fn with_additional_values(self, additional_values: &'a impl AsRef<[(u16, u8)]>) -> Self {
        Self {
            additional_values: additional_values.as_ref(),
            ..self
        }
    }

    #[must_use]
    pub fn with_check_pc(self, check_pc: bool) -> Self {
        Self { check_pc, ..self }
    }

    #[must_use]
    pub fn with_prepare<NewPrepareFunc: FnOnce(&mut Cpu)>(
        self,
        prepare: NewPrepareFunc,
    ) -> TestOpcodeOptions<'a, NewPrepareFunc, VerifyFunc> //
    {
        // can't do ..self coz different generic types
        TestOpcodeOptions {
            prepare,
            opcode: self.opcode,
            arguments: self.arguments,
            additional_values: self.additional_values,
            expected_cycles: self.expected_cycles,
            check_pc: self.check_pc,
            verify: self.verify,
        }
    }
}
