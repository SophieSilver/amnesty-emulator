use crate::cpu::tests::addressing_modes::read::TestReadArgs;

pub struct BytePairsWithCarry;

impl TestReadArgs for BytePairsWithCarry {
    type AdditionalArgs = (u8, bool);

    fn args() -> impl Iterator<Item = u8> {
        u8::MIN..u8::MAX
    }

    fn additional_args() -> impl Iterator<Item = Self::AdditionalArgs> {
        (u8::MIN..u8::MAX).flat_map(|byte| [(byte, false), (byte, true)])
    }
}
