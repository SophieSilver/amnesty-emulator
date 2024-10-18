use utils::test_opcode;

use crate::cpu::dispatch::OpCode;

mod utils;

#[test]
fn test_ldx_immediate() {
    test_opcode(OpCode::LdxImmediate, [0x69], [], 2, |cpu, _memory| {
        assert_eq!(cpu.x_index, 0x69)
    });
}

#[test]
fn test_ldx_zeropage() {
    test_opcode(
        OpCode::LdxZeroPage,
        [0x89],
        [(0x89, 0x23)],
        3,
        |cpu, _memory| assert_eq!(cpu.x_index, 0x23),
    );
}

// #[test]
// fn test_ldx_zeropage_y() {
//     // test_opcode(OpCode::LdxZeroPageY, [0x79], [()], 2, |cpu, _memory| {
//     //     assert_eq!(cpu.x_index, 0x69)
//     // });
// }

// #[test]
// fn test_ldx_immediate() {
//     test_opcode(OpCode::LdxImmediate, [0x69], [], 2, |cpu, _memory| {
//         assert_eq!(cpu.x_index, 0x69)
//     });
// }

// #[test]
// fn lda_test() {
//     test_opcode(OpCode::LdxImmediate, &[0x1], 2, &[], |cpu, _memory| {
//         assert_eq!(cpu.x_index, 1)
//     });

//     let mut ram = Ram::new();
//     let mut cpu = Cpu::new();
//     let mut memory = MemoryMapping { ram: &mut ram };
//     cpu.program_counter = 0;

//     #[rustfmt::skip]
//     let mem_state = [
//         // LDA #1
//         0xA9, 0x1,
//         // LDA $89
//         0xA5, 0x89,
//         // LDA $89, X       (X = 0x01)
//         0xB5, 0x89,
//         // LDA $89, X       (X = 0xB5) (test page overflow behavior)
//         0xB5, 0x89,
//         // LDA 0x0489
//         0xAD, 0x89, 0x04,
//         // LDA 0x0489, X    (X = 0x01)
//         0xBD, 0x89, 0x04,
//         // LDA 0x0489, X    (X = 0xB5) (test page overflow behavior)
//         0xBD, 0x89, 0x04,
//         // LDA 0x0489, Y    (Y = 0x02)
//         0xB9, 0x89, 0x04,
//         // LDA 0x0489, Y    (Y = 0xB6) (test page overflow behavior)
//         0xB9, 0x89, 0x04,
//         // LDA ($0x78, X)
//         0xA1, 0x78,
//         // LDA($0x77, X)
//         0xA1, 0x77,
//         // LDA($0x70),Y
//         0xB1, 0x70,
//         0xB1, 0x70,
//     ];
//     for (i, byte) in mem_state.into_iter().enumerate() {
//         memory.store(i as u16, byte);
//     }
//     // zero page 0x89
//     memory.store(0x0089, 0x02);
//     // zero page 0x89 + 1
//     memory.store(0x008A, 0x03);
//     // zero page + 0xB5 (0x003E), page is wrapped around
//     memory.store(0x003E, 0x04);
//     // absolute 0x0489
//     memory.store(0x0489, 0x05);
//     // absolute 0x0489 + 1
//     memory.store(0x048A, 0x06);
//     // absolute 0x0489 + 0xB5
//     memory.store(0x053E, 0x07);
//     // absolute 0x0489 + 2
//     memory.store(0x048B, 0x08);
//     // absolute 0x0489 + 0xB6
//     memory.store(0x053F, 0x09);
//     // indirect 0x78 + 1 -> 0x0234,
//     memory.store(0x0079, 0x34);
//     memory.store(0x007A, 0x02);
//     memory.store(0x0234, 0x0A);
//     // indirect 0x77 + 255 -> 0x0235,
//     memory.store(0x0076, 0x35);
//     memory.store(0x0077, 0x02);
//     memory.store(0x0235, 0x0B);

//     // indirect 0x70 -> 0x0250 + 1
//     memory.store(0x0070, 0x50);
//     memory.store(0x0071, 0x02);
//     memory.store(0x0251, 0x0C);
//     memory.store(0x034F, 0x0D);

//     // LDA immediate
//     (0..2).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_eq!(cpu.program_counter, 2);
//     assert_eq!(cpu.accumulator, 0x1);

//     // // LDA zeropage
//     (0..3).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_eq!(cpu.program_counter, 4);
//     assert_eq!(cpu.accumulator, 0x2);

//     // LDA zeropage + X
//     cpu.x_index = 0x1;
//     (0..4).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_eq!(cpu.program_counter, 6);
//     assert_eq!(cpu.accumulator, 0x3);

//     // LDA zeropage + X overflow
//     cpu.x_index = 0xB5;
//     (0..4).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_eq!(cpu.program_counter, 8);
//     assert_eq!(cpu.accumulator, 0x4);

//     // LDX absolute
//     (0..4).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_eq!(cpu.program_counter, 11);
//     assert_eq!(cpu.accumulator, 0x5);

//     // LDA absolute + X
//     cpu.x_index = 0x1;
//     (0..4).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_eq!(cpu.program_counter, 14);
//     assert_eq!(cpu.accumulator, 0x6);

//     // LDA absolute + X overflow
//     cpu.x_index = 0xB5;
//     (0..4).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_ne!(cpu.accumulator, 0x7); // shouldn't be ready yet coz of page boundary
//     cpu.run_cycle(&mut memory);
//     assert_eq!(cpu.program_counter, 17);
//     assert_eq!(cpu.accumulator, 0x7); // NOW it's ready
//                                       // the current instruction must be finished
//     assert_eq!(cpu.current_cycle, 0);

//     // LDA absolute + Y
//     cpu.y_index = 0x2;
//     (0..4).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_eq!(cpu.program_counter, 20);
//     assert_eq!(cpu.accumulator, 0x8);

//     // LDA absolute + Y overflow
//     cpu.y_index = 0xB6;
//     (0..4).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_ne!(cpu.accumulator, 0x9); // shouldn't be ready yet coz of page boundary
//     cpu.run_cycle(&mut memory);
//     assert_eq!(cpu.program_counter, 23);
//     assert_eq!(cpu.accumulator, 0x9); // NOW it's ready
//                                       // the current instruction must be finished
//     assert_eq!(cpu.current_cycle, 0);

//     cpu.x_index = 0x1;
//     (0..6).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_eq!(cpu.program_counter, 25);
//     assert_eq!(cpu.accumulator, 0xA);

//     cpu.x_index = 0xFF;
//     (0..6).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_eq!(cpu.program_counter, 27);
//     assert_eq!(cpu.accumulator, 0xB);

//     cpu.y_index = 0x1;
//     (0..5).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_eq!(cpu.program_counter, 29);
//     assert_eq!(cpu.accumulator, 0xC);

//     cpu.y_index = 0xFF;
//     (0..5).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_ne!(cpu.accumulator, 0xD);
//     cpu.run_cycle(&mut memory);
//     assert_eq!(cpu.accumulator, 0xD);
//     assert_eq!(cpu.program_counter, 31);
// }

// #[test]
// fn ldx_test() {
//     let mut ram = Ram::new();
//     let mut cpu = Cpu::new();
//     let mut memory = MemoryMapping { ram: &mut ram };
//     cpu.program_counter = 0;

//     #[rustfmt::skip]
//     let mem_state = [
//         // LDX #1
//         0xA2, 0x1,
//         // LDX $89
//         0xA6, 0x89,
//         // LDX $89, Y       (Y = 0x01)
//         0xB6, 0x89,
//         // LDX $89, Y       (Y = 0xB6) (test page overflow behavior)
//         0xB6, 0x89,
//         // LDX 0x0489
//         0xAE, 0x89, 0x04,
//         // LDX 0x0489, Y    (Y = 0x01)
//         0xBE, 0x89, 0x04,
//         // LDX 0x0489, Y    (Y = 0xB6) (test page overflow behavior)
//         0xBE, 0x89, 0x04,
//     ];
//     for (i, byte) in mem_state.into_iter().enumerate() {
//         memory.store(i as u16, byte);
//     }
//     // zero page 0x89
//     memory.store(0x0089, 0x02);
//     // zero page 0x89 + 1
//     memory.store(0x008A, 0x03);
//     // zero page 0x89 + 0xB6 (0x003F), page is wrapped around
//     memory.store(0x003F, 0x04);
//     // absolute 0x0489
//     memory.store(0x0489, 0x05);
//     // absolute 0x0489 + 1
//     memory.store(0x048A, 0x06);
//     // absolute 0x0489 + 0xB6 (0x053F), page boundary must be crossed
//     memory.store(0x053F, 0x07);

//     // LDX immediate
//     (0..2).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_eq!(cpu.program_counter, 2);
//     assert_eq!(cpu.x_index, 0x1);

//     // LDX zeropage
//     (0..3).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_eq!(cpu.program_counter, 4);
//     assert_eq!(cpu.x_index, 0x2);

//     // LDX zeropage + Y
//     cpu.y_index = 0x1;
//     (0..4).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_eq!(cpu.program_counter, 6);
//     assert_eq!(cpu.x_index, 0x3);

//     // LDX zeropage + Y overflow
//     cpu.y_index = 0xB6;
//     (0..4).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_eq!(cpu.program_counter, 8);
//     assert_eq!(cpu.x_index, 0x4);

//     // LDX absolute
//     (0..4).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_eq!(cpu.program_counter, 11);
//     assert_eq!(cpu.x_index, 0x5);

//     // LDX absolute + Y
//     cpu.y_index = 0x1;
//     (0..4).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_eq!(cpu.program_counter, 14);
//     assert_eq!(cpu.x_index, 0x6);

//     // LDX absolute + Y overflow
//     cpu.y_index = 0xB6;
//     (0..4).for_each(|_| cpu.run_cycle(&mut memory));
//     assert_ne!(cpu.x_index, 0x7); // shouldn't be ready yet coz of page boundary

//     cpu.run_cycle(&mut memory);
//     assert_eq!(cpu.program_counter, 17);
//     assert_eq!(cpu.x_index, 0x7); // NOW it's ready

//     // the current instruction must be finished
//     assert_eq!(cpu.current_cycle, 0);
// }
