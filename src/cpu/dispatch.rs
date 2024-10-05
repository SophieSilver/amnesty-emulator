use num_enum::{FromPrimitive, IntoPrimitive};

use crate::memory::MemoryMapping;

use super::CpuState;

mod instructions;

#[derive(Debug, Clone, Copy, IntoPrimitive, FromPrimitive, Default)]
#[non_exhaustive]
#[repr(u8)]
pub enum OpCode {
    LdxImmediate = 0xA2,
    LdxZeroPage = 0xA6,
    LdxZeroPageY = 0xB6,
    LdxAbs = 0xAE,
    LdxAbsY = 0xBE,

    #[default]
    Unimplemented,
}

pub fn dispatch_opcode(cpu_state: &mut CpuState, memory: &mut MemoryMapping) {
    match cpu_state.current_opcode {
        OpCode::LdxImmediate => {
            
        }
        _ => todo!()
    }
}
