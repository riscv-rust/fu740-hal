use core::convert::TryFrom;
use assert2::assert;
use super::*;

#[test]
fn from_valid_u8() {
    // Given
    let value = 0_u8;
    let expected = Hertz::from(0_u64);

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
fn from_valid_u16() {
    // Given
    let value = 1_u16;
    let expected = Hertz::from(1_u64);

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
fn from_valid_u32() {
    // Given
    let value = 2_u32;
    let expected = Hertz::from(2_u64);

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
fn from_valid_u64() {
    // Given
    let value = 3_u64;
    let expected = Hertz::from(3_u64);

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
fn from_valid_u128() {
    // Given
    let value = 5_u128;
    let expected = Hertz::from(5_u64);

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
fn from_valid_usize() {
    // Given
    let value = 7_usize;
    let expected = Hertz::from(7_u64);

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
fn from_max_u8() {
    // Given
    let value = u8::MAX;
    let expected = Hertz::from(u64::from(u8::MAX));

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
fn from_max_u16() {
    // Given
    let value = u16::MAX;
    let expected = Hertz::from(u64::from(u16::MAX));

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
fn from_max_u32() {
    // Given
    let value = u32::MAX;
    let expected = Hertz::from(u64::from(u32::MAX));

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
fn from_max_u64() {
    // Given
    let value = u64::MAX;
    let expected = Hertz::from(u64::MAX);

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
#[should_panic(expected = "Hertz value overflows `HertzInner` type.")]
fn from_max_u128() {
    // Given
    let value = u128::MAX;

    // When
    let _ = Hertz::from(value);

    // Then
    unreachable!();
}

#[test]
fn from_max_usize() {
    // Given
    let value = usize::MAX;
    let expected = Hertz::from(u64::try_from(usize::MAX).unwrap());

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}
