use crate::cpu::StatusFlags;

pub fn check_nz_flags(value: u8, flags: StatusFlags) {
    assert_eq!(
        flags.contains(StatusFlags::NEGATIVE),
        (value as i8) < 0,
        "NEGATIVE flag set incorrectly"
    );
    assert_eq!(
        flags.contains(StatusFlags::ZERO),
        value == 0,
        "ZERO flag set incorrectly"
    );
}
