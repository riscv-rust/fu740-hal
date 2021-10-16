use core::convert::TryFrom;
use assert2::assert;
use super::*;

#[test]
fn from_valid_u8() {
    // Given
    const VALUE: u8 = 0;
    let expected = Hertz::from(0_u64);

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

#[test]
fn from_valid_u16() {
    // Given
    const VALUE: u16 = 1;
    let expected = Hertz::from(1_u64);

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

#[test]
fn from_valid_u32() {
    // Given
    const VALUE: u32 = 2;
    let expected = Hertz::from(2_u64);

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

#[test]
fn from_valid_u64() {
    // Given
    const VALUE: u64 = 3;
    let expected = Hertz::from(3_u64);

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

#[test]
fn from_valid_u128() {
    // Given
    const VALUE: u128 = 5;
    let expected = Hertz::from(5_u64);

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

#[test]
fn from_valid_usize() {
    // Given
    const VALUE: usize = 7;
    let expected = Hertz::from(7_u64);

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

#[test]
fn from_max_u8() {
    // Given
    const VALUE: u8 = u8::MAX;
    let expected = Hertz::from(u64::from(u8::MAX));

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

#[test]
fn from_max_u16() {
    // Given
    const VALUE: u16 = u16::MAX;
    let expected = Hertz::from(u64::from(u16::MAX));

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

#[test]
fn from_max_u32() {
    // Given
    const VALUE: u32 = u32::MAX;
    let expected = Hertz::from(u64::from(u32::MAX));

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

#[test]
fn from_max_u64() {
    // Given
    const VALUE: u64 = u64::MAX;
    let expected = Hertz::from(u64::MAX);

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

// // This test is commented out because it fails to compile when it passes.  Uncomment test to verify behavior.
// #[test]
// #[should_panic(expected = "Hertz value overflows `HertzInner` type.")]
// fn from_max_u128() {
//     // Given
//     const VALUE: u128 = u128::MAX;
//
//     // When
//     const _: Hertz = Hertz::from(VALUE);
//
//     // Then
//     unreachable!();
// }

#[cfg(any(target_pointer_width = "16", target_pointer_width = "32", target_pointer_width = "64"))]
#[test]
fn from_max_usize() {
    // Given
    const VALUE: usize = usize::MAX;
    let expected = Hertz::from(u64::try_from(usize::MAX).unwrap());

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

// #[cfg(target_pointer_width = "128")]
// // This test is commented out because it fails to compile when it passes.  Uncomment test to verify behavior.
// #[test]
// #[should_panic(expected = "Hertz value overflows `HertzInner` type.")]
// fn from_max_usize() {
//     // Given
//     const VALUE: usize = usize::MAX;
//
//     // When
//     const _: Hertz = Hertz::from(VALUE);
//
//     // Then
//     unreachable!();
// }
