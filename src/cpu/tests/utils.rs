use presets::apply_preset;

use crate::{
    cpu::{dispatch::OpCode, Cpu},
    memory::Memory,
};

use super::TestMemory;

mod presets;

/// Returns an iterator of all possible pairs of values between 0 and `u8::MAX`
pub fn possible_byte_pairs() -> impl Iterator<Item = (u8, u8)> {
    (u8::MIN..=u8::MAX).flat_map(|a| (u8::MIN..=u8::MAX).map(move |b| (a, b)))
}

/// Returns an iterator of all possible triplets of values, the first two values are
/// numbers between 0 and `u8::MAX`, the third is a boolean
pub fn possible_pairs_with_carry() -> impl Iterator<Item = (u8, u8, bool)> {
    possible_byte_pairs().flat_map(|(a, b)| [(a, b, false), (a, b, true)])
}

pub fn possible_values_with_2_bools() -> impl Iterator<Item = (u8, bool, bool)> {
    let possible_bools = [(false, false), (false, true), (true, false), (true, true)];

    (0..u8::MAX).flat_map(move |value| possible_bools.map(|(b1, b2)| (value, b1, b2)))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    Immediate(u8),
    ZeroPage(u8),
    ZeroPageX(u8),
    ZeroPageXOverflow(u8),
    ZeroPageY(u8),
    ZeroPageYOverflow(u8),
    Absolute(u8),
    AbsoluteX(u8),
    AbsoluteXOverflow(u8),
    AbsoluteY(u8),
    AbsoluteYOverflow(u8),
    IndirectX(u8),
    IndirectXOverflow(u8),
    IndirectXPageSplit(u8),
    IndirectY(u8),
    IndirectYOverflow(u8),
    IndirectYPageSplit(u8),
}

const OPCODE_ADDR: u16 = 0x0200;

pub trait ArgumentSource {
    fn apply(&self, cpu: &mut Cpu, memory: &mut TestMemory) -> u16;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmptySource;

impl ArgumentSource for EmptySource {
    fn apply(&self, _cpu: &mut Cpu, _memory: &mut TestMemory) -> u16 {
        0
    }
}

impl ArgumentSource for Preset {
    fn apply(&self, cpu: &mut Cpu, memory: &mut TestMemory) -> u16 {
        apply_preset(*self, cpu, memory)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExplicitSource<'a> {
    /// Arguments that follow the opcode
    ///
    /// They will be loaded at addresses `0x1..arguments.len()`
    pub arguments: &'a [u8],

    /// Additional values to place at specified memory addresses
    pub additional_values: &'a [(u16, u8)],
}

impl<'a> ArgumentSource for ExplicitSource<'a> {
    fn apply(&self, _cpu: &mut Cpu, memory: &mut TestMemory) -> u16 {
        for (i, &value) in self.arguments.iter().enumerate() {
            // address wrapped in 0..u16::MAX range
            let addr = (i + OPCODE_ADDR as usize + 1) % u16::MAX as usize;
            let addr = addr as u16;

            memory.store(addr, value);
        }

        for &(addr, value) in self.additional_values {
            memory.store(addr, value);
        }

        self.arguments
            .len()
            .try_into()
            .expect("Length of the arguments fits in a u16")
    }
}

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
#[derive(Debug)]
pub struct TestOpcodeOptions<PrepareFunc, VerifyFunc, A = EmptySource>
where
    A: ArgumentSource,
    PrepareFunc: FnOnce(&mut Cpu),
    VerifyFunc: FnOnce(&mut Cpu, &mut TestMemory),
{
    /// What opcode to test
    ///
    /// This will be loaded at address `0x0200`
    pub opcode: OpCode,

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

    pub argument_source: A,
}

// using a function pointer here for less headache with existential types
impl<VerifyFunc> TestOpcodeOptions<fn(&mut Cpu), VerifyFunc>
where
    VerifyFunc: FnOnce(&mut Cpu, &mut TestMemory),
{
    #[must_use]
    pub fn new(opcode: OpCode, expected_cycles: u8, verify: VerifyFunc) -> Self {
        Self {
            opcode,
            expected_cycles,
            check_pc: true,
            prepare: |_| {},
            verify,
            argument_source: EmptySource,
        }
    }
}

impl<PrepareFunc, VerifyFunc, A> TestOpcodeOptions<PrepareFunc, VerifyFunc, A>
where
    A: ArgumentSource,
    PrepareFunc: FnOnce(&mut Cpu),
    VerifyFunc: FnOnce(&mut Cpu, &mut TestMemory),
{
    /// Run the test with current Options
    pub fn test(self) {
        // initialize the CPU and the memory
        let mut cpu = Cpu::new();
        let mut memory = TestMemory::new();

        // prepare memory
        memory.store(OPCODE_ADDR, self.opcode.into());

        let argument_len = self.argument_source.apply(&mut cpu, &mut memory);

        // prepare the cpu
        cpu.program_counter = OPCODE_ADDR;
        (self.prepare)(&mut cpu);

        // execute
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
            let expected_pc = OPCODE_ADDR + 1 + argument_len;
            assert_eq!(
                cpu.program_counter, expected_pc,
                "instruction pointer incorrectly set after instruction"
            );
        }

        (self.verify)(&mut cpu, &mut memory);
    }

    #[must_use]
    pub fn with_check_pc(self, check_pc: bool) -> Self {
        Self { check_pc, ..self }
    }

    #[must_use]
    pub fn with_prepare<NewPrepareFunc: FnOnce(&mut Cpu)>(
        self,
        prepare: NewPrepareFunc,
    ) -> TestOpcodeOptions<NewPrepareFunc, VerifyFunc, A> //
    {
        // can't do ..self coz different generic types
        TestOpcodeOptions {
            prepare,
            opcode: self.opcode,
            expected_cycles: self.expected_cycles,
            check_pc: self.check_pc,
            verify: self.verify,
            argument_source: self.argument_source,
        }
    }
}

impl<PrepareFunc, VerifyFunc> TestOpcodeOptions<PrepareFunc, VerifyFunc>
where
    PrepareFunc: FnOnce(&mut Cpu),
    VerifyFunc: FnOnce(&mut Cpu, &mut TestMemory),
{
    #[must_use]
    pub fn with_arguments<'a>(
        self,
        arguments: &'a impl AsRef<[u8]>,
    ) -> TestOpcodeOptions<PrepareFunc, VerifyFunc, ExplicitSource<'a>> //
    {
        TestOpcodeOptions {
            opcode: self.opcode,
            expected_cycles: self.expected_cycles,
            check_pc: self.check_pc,
            prepare: self.prepare,
            verify: self.verify,
            argument_source: ExplicitSource {
                arguments: &[],
                additional_values: &[],
            },
        }
        .with_arguments(arguments)
    }

    #[must_use]
    pub fn with_additional_values<'a>(
        self,
        additional_values: &'a impl AsRef<[(u16, u8)]>,
    ) -> TestOpcodeOptions<PrepareFunc, VerifyFunc, ExplicitSource<'a>> //
    {
        TestOpcodeOptions {
            opcode: self.opcode,
            expected_cycles: self.expected_cycles,
            check_pc: self.check_pc,
            prepare: self.prepare,
            verify: self.verify,
            argument_source: ExplicitSource {
                arguments: &[],
                additional_values: &[],
            },
        }
        .with_additional_values(additional_values)
    }

    #[must_use]
    pub fn with_preset(self, preset: Preset) -> TestOpcodeOptions<PrepareFunc, VerifyFunc, Preset> {
        TestOpcodeOptions {
            opcode: self.opcode,
            expected_cycles: self.expected_cycles,
            check_pc: self.check_pc,
            prepare: self.prepare,
            verify: self.verify,
            argument_source: preset,
        }
    }
}

impl<'a, PrepareFunc, VerifyFunc> TestOpcodeOptions<PrepareFunc, VerifyFunc, ExplicitSource<'a>>
where
    PrepareFunc: FnOnce(&mut Cpu),
    VerifyFunc: FnOnce(&mut Cpu, &mut TestMemory),
{
    #[must_use]
    pub fn with_arguments(self, arguments: &'a impl AsRef<[u8]>) -> Self {
        Self {
            argument_source: ExplicitSource {
                arguments: arguments.as_ref(),
                ..self.argument_source
            },
            ..self
        }
    }

    #[must_use]
    pub fn with_additional_values(self, additional_values: &'a impl AsRef<[(u16, u8)]>) -> Self {
        Self {
            argument_source: ExplicitSource {
                additional_values: additional_values.as_ref(),
                ..self.argument_source
            },
            ..self
        }
    }
}
