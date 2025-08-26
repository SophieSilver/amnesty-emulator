use crate::cpu::{
    instructions::{opcode::OpCode, Bit},
    tests::{addressing_modes::read::*, test_args::BytePairs},
    Cpu, StatusFlags,
};

impl TestReadInstruction for Bit {
    type Args = BytePairs;

    fn prepare(cpu: &mut Cpu, _: u8, a: u8) {
        cpu.a = a;
    }

    fn verify(cpu: &Cpu, b: u8, a: u8) {
        let result = a & b;
        let zero = result == 0;
        let negative_flag = (b as i8) < 0;
        let overflow_flag = (b >> 6) & 1 == 1;

        assert_eq!(
            cpu.flags.contains(StatusFlags::ZERO),
            zero,
            "ZERO flag set incorrectly"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::NEGATIVE),
            negative_flag,
            "NEGATIVE flag set incorrectly"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::OVERFLOW),
            overflow_flag,
            "OVERFLOW flag set incorrectly"
        );
    }
}

impl TestReadZeropage for Bit {
    const OPCODE: OpCode = OpCode::BitZeropage;
}

impl TestReadAbsolute for Bit {
    const OPCODE: OpCode = OpCode::BitAbsolute;
}

#[test]
fn zeropage() {
    Bit::test_zeropage();
}

#[test]
fn absolute() {
    Bit::test_absolute();
}
