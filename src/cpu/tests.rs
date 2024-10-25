#![allow(clippy::arithmetic_side_effects)]

use utils::TestOpcodeOptions;

use crate::cpu::dispatch::OpCode;

#[allow(dead_code)]
mod utils;

mod adc;
mod and;
mod lda;
mod ldx;
mod ldy;
