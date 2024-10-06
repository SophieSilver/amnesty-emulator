use crate::memory::{ram::Ram, MemoryMapping};

use super::{clock_tick, load_first_opcode, CpuState};

#[test]
fn ldx_test() {
    let mut ram = Ram::new();
    let mut cpu_state = CpuState::new();
    let mut memory = MemoryMapping { ram: &mut ram };
    cpu_state.program_counter = 0;

    #[rustfmt::skip]
    let mem_state = [
        // LDX #1
        0xA2, 0x1,
        // LDX $89
        0xA6, 0x89,
        // LDX $89, Y       (Y = 0x01)
        0xB6, 0x89,
        // LDX $89, Y       (Y = 0xB6) (test page overflow behavior)
        0xB6, 0x89,
        // LDX 0x0489
        0xAE, 0x89, 0x04,
        // LDX 0x0489, Y    (Y = 0x01)
        0xBE, 0x89, 0x04,
        // LDX 0x0489, Y    (Y = 0xB6) (test page overflow behavior)
        0xBE, 0x89, 0x04,
    ];
    for (i, byte) in mem_state.into_iter().enumerate() {
        memory.store(i as u16, byte);
    }
    // zero page 0x89
    memory.store(0x0089, 0x02);
    // zero page 0x89 + 1
    memory.store(0x008A, 0x03);
    // zero page 0x89 + 0xB6 (0x003F), page is wrapped around
    memory.store(0x003F, 0x04);
    // absolute 0x0489
    memory.store(0x0489, 0x05);
    // absolute 0x0489 + 1
    memory.store(0x048A, 0x06);
    // absolute 0x0489 + 0xB6 (0x053F), page boundary must be crossed
    memory.store(0x053F, 0x07);

    // LDX immediate
    load_first_opcode(&mut cpu_state, &mut memory);
    clock_tick(&mut cpu_state, &mut memory);
    assert_eq!(cpu_state.program_counter, 2);
    assert_eq!(cpu_state.x_index, 0x1);

    // LDX zeropage
    (0..3).for_each(|_| clock_tick(&mut cpu_state, &mut memory));
    assert_eq!(cpu_state.program_counter, 4);
    assert_eq!(cpu_state.x_index, 0x2);

    // LDX zeropage + Y
    cpu_state.y_index = 0x1;
    (0..4).for_each(|_| clock_tick(&mut cpu_state, &mut memory));
    assert_eq!(cpu_state.program_counter, 6);
    assert_eq!(cpu_state.x_index, 0x3);

    // LDX zeropage + Y overflow
    cpu_state.y_index = 0xB6;
    (0..4).for_each(|_| clock_tick(&mut cpu_state, &mut memory));
    assert_eq!(cpu_state.program_counter, 8);
    assert_eq!(cpu_state.x_index, 0x4);

    // LDX absolute
    (0..4).for_each(|_| clock_tick(&mut cpu_state, &mut memory));
    assert_eq!(cpu_state.program_counter, 11);
    assert_eq!(cpu_state.x_index, 0x5);

    // LDX absolute + Y
    cpu_state.y_index = 0x1;
    (0..4).for_each(|_| clock_tick(&mut cpu_state, &mut memory));
    assert_eq!(cpu_state.program_counter, 14);
    assert_eq!(cpu_state.x_index, 0x6);

    // LDX absolute + Y overflow
    cpu_state.y_index = 0xB6;
    (0..4).for_each(|_| clock_tick(&mut cpu_state, &mut memory));
    assert_ne!(cpu_state.x_index, 0x7);     // shouldn't be ready yet coz of page boundary

    clock_tick(&mut cpu_state, &mut memory);
    assert_eq!(cpu_state.program_counter, 17);
    assert_eq!(cpu_state.x_index, 0x7);     // NOW it's ready

    // this should start new instruction
    clock_tick(&mut cpu_state, &mut memory);
    assert_eq!(cpu_state.current_cycle, 0);
}
