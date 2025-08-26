use crate::cpu::StatusFlags;

pub fn check_negative_and_zero_flags(value: u8, flags: StatusFlags) {
    assert_eq!(
        flags.contains(StatusFlags::NEGATIVE),
        (value as i8).is_negative(),
        "NEGATIVE flag set incorrectly"
    );
    assert_eq!(
        flags.contains(StatusFlags::ZERO),
        value == 0,
        "ZERO flag set incorrectly"
    );
}
