use core::convert::TryFrom;
use assert2::assert;
use super::*;

#[test]
#[should_panic(expected = "Hertz value is below minimum possible frequency.")]
fn from_negative_i8() {
    // Given
    let value = -1_i8;

    // When
    let _ = Hertz::from(value);

    // Then
    unreachable!();
}

#[test]
#[should_panic(expected = "Hertz value is below minimum possible frequency.")]
fn from_negative_i16() {
    // Given
    let value = -2_i16;

    // When
    let _ = Hertz::from(value);

    // Then
    unreachable!();
}

#[test]
#[should_panic(expected = "Hertz value is below minimum possible frequency.")]
fn from_negative_i32() {
    // Given
    let value = -3_i32;

    // When
    let _ = Hertz::from(value);

    // Then
    unreachable!();
}

#[test]
#[should_panic(expected = "Hertz value is below minimum possible frequency.")]
fn from_negative_i64() {
    // Given
    let value = -5_i64;

    // When
    let _ = Hertz::from(value);

    // Then
    unreachable!();
}

#[test]
#[should_panic(expected = "Hertz value is below minimum possible frequency.")]
fn from_negative_i128() {
    // Given
    let value = -7_i128;

    // When
    let _ = Hertz::from(value);

    // Then
    unreachable!();
}

#[test]
#[should_panic(expected = "Hertz value is below minimum possible frequency.")]
fn from_negative_isize() {
    // Given
    let value = -11_isize;

    // When
    let _ = Hertz::from(value);

    // Then
    unreachable!();
}

#[test]
fn from_valid_i8() {
    // Given
    let value = 0_i8;
    let expected = Hertz::from(0_u64);

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
fn from_valid_i16() {
    // Given
    let value = 1_i16;
    let expected = Hertz::from(1_u64);

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
fn from_valid_i32() {
    // Given
    let value = 2_i32;
    let expected = Hertz::from(2_u64);

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
fn from_valid_i64() {
    // Given
    let value = 3_i64;
    let expected = Hertz::from(3_u64);

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
fn from_valid_i128() {
    // Given
    let value = 5_i128;
    let expected = Hertz::from(5_u64);

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
fn from_valid_isize() {
    // Given
    let value = 7_isize;
    let expected = Hertz::from(7_u64);

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
fn from_max_i8() {
    // Given
    let value = i8::MAX;
    let expected = Hertz::from(u64::try_from(i8::MAX).unwrap());

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
fn from_max_i16() {
    // Given
    let value = i16::MAX;
    let expected = Hertz::from(u64::try_from(i16::MAX).unwrap());

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
fn from_max_i32() {
    // Given
    let value = i32::MAX;
    let expected = Hertz::from(u64::try_from(i32::MAX).unwrap());

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
fn from_max_i64() {
    // Given
    let value = i64::MAX;
    let expected = Hertz::from(u64::try_from(i64::MAX).unwrap());

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}

#[test]
#[should_panic(expected = "Hertz value overflows `HertzInner` type.")]
fn from_max_i128() {
    // Given
    let value = i128::MAX;

    // When
    let _ = Hertz::from(value);

    // Then
    unreachable!();
}

#[test]
fn from_max_isize() {
    // Given
    let value = isize::MAX;
    let expected = Hertz::from(u64::try_from(isize::MAX).unwrap());

    // When
    let result = Hertz::from(value);

    // Then
    assert!(result == expected);
}
