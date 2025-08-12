/// Add two numbers and a carry bit
pub fn add_with_carry(a: u8, b: u8, carry: bool) -> (u8, bool) {
    // this is equivalent to the implementation of the unstable `carrying_add`
    let (sum1, carry1) = a.overflowing_add(b);
    let (sum2, carry2) = sum1.overflowing_add(carry as u8);
    // note: no intermediate overflow is required (https://github.com/rust-lang/rust/issues/85532#issuecomment-1032214946).
    (sum2, carry1 | carry2)
}

/// Check if adding two numbers and a carry bit would result in signed overflow
pub fn add_overflows(a: i8, b: i8, carry: bool) -> bool {
    let (sum1, overflow1) = a.overflowing_add(b);
    let (_, overflow2) = sum1.overflowing_add(carry as i8);

    // note: no intermediate overflow is required (https://github.com/rust-lang/rust/issues/85532#issuecomment-1032214946).
    // the behavior described in the comment above can be observed at http://www.visual6502.org/JSSim/expert.html
    overflow1 != overflow2
}

/// Subtract two numbers and a carry bit, which acts as an inverse of the borrow flag
pub fn sub_with_carry(a: u8, b: u8, carry: bool) -> (u8, bool) {
    // see https://www.reddit.com/r/EmuDev/comments/k5hzuo/comment/gegnkq5/
    add_with_carry(a, !b, carry)
}

/// Check if subtracting two numbers and a carry bit would result in signed overflow
pub fn sub_overflows(a: i8, b: i8, carry: bool) -> bool {
    add_overflows(a, !b, carry)
}
