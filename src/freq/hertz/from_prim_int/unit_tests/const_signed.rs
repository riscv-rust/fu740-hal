use core::convert::TryFrom;
use assert2::assert;
use super::*;

// // This test is commented out because it fails to compile when it passes.  Uncomment test to verify behavior.
// #[test]
// #[should_panic(expected = "Hertz value is below minimum possible frequency.")]
// fn from_negative_i8() {
//     // Given
//     const VALUE: i8 = -1;
//
//     // When
//     const _: Hertz = Hertz::from(VALUE);
//
//     // Then
//     unreachable!();
// }

// // This test is commented out because it fails to compile when it passes.  Uncomment test to verify behavior.
// #[test]
// #[should_panic(expected = "Hertz value is below minimum possible frequency.")]
// fn from_negative_i16() {
//     // Given
//     const VALUE: i16 = -2;
//
//     // When
//     const _: Hertz = Hertz::from(VALUE);
//
//     // Then
//     unreachable!();
// }

// // This test is commented out because it fails to compile when it passes.  Uncomment test to verify behavior.
// #[test]
// #[should_panic(expected = "Hertz value is below minimum possible frequency.")]
// fn from_negative_i32() {
//     // Given
//     const VALUE: i32 = -3;
//
//     // When
//     const _: Hertz = Hertz::from(VALUE);
//
//     // Then
//     unreachable!();
// }

// // This test is commented out because it fails to compile when it passes.  Uncomment test to verify behavior.
// #[test]
// #[should_panic(expected = "Hertz value is below minimum possible frequency.")]
// fn from_negative_i64() {
//     // Given
//     const VALUE: i64 = -5;
//
//     // When
//     const _: Hertz = Hertz::from(VALUE);
//
//     // Then
//     unreachable!();
// }

// This test is commented out because it fails to compile when it passes.  Uncomment test to verify behavior.
// #[test]
// #[should_panic(expected = "Hertz value is below minimum possible frequency.")]
// fn from_negative_i128() {
//     // Given
//     const VALUE: i128 = -7;
//
//     // When
//     const _: Hertz = Hertz::from(VALUE);
//
//     // Then
//     unreachable!();
// }

// // This test is commented out because it fails to compile when it passes.  Uncomment test to verify behavior.
// #[test]
// #[should_panic(expected = "Hertz value is below minimum possible frequency.")]
// fn from_negative_isize() {
//     // Given
//     const VALUE: isize = -11;
//
//     // When
//     const _: Hertz = Hertz::from(VALUE);
//
//     // Then
//     unreachable!();
// }

#[test]
fn from_valid_i8() {
    // Given
    const VALUE: i8 = 0;
    let expected = Hertz::from(0_u64);

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

#[test]
fn from_valid_i16() {
    // Given
    const VALUE: i16 = 1;
    let expected = Hertz::from(1_u64);

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

#[test]
fn from_valid_i32() {
    // Given
    const VALUE: i32 = 2;
    let expected = Hertz::from(2_u64);

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

#[test]
fn from_valid_i64() {
    // Given
    const VALUE: i64 = 3;
    let expected = Hertz::from(3_u64);

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

#[test]
fn from_valid_i128() {
    // Given
    const VALUE: i128 = 5;
    let expected = Hertz::from(5_u64);

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

#[test]
fn from_valid_isize() {
    // Given
    const VALUE: isize = 7;
    let expected = Hertz::from(7_u64);

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

#[test]
fn from_max_i8() {
    // Given
    const VALUE: i8 = i8::MAX;
    let expected = Hertz::from(u64::try_from(i8::MAX).unwrap());

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

#[test]
fn from_max_i16() {
    // Given
    const VALUE: i16 = i16::MAX;
    let expected = Hertz::from(u64::try_from(i16::MAX).unwrap());

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

#[test]
fn from_max_i32() {
    // Given
    const VALUE: i32 = i32::MAX;
    let expected = Hertz::from(u64::try_from(i32::MAX).unwrap());

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

#[test]
fn from_max_i64() {
    // Given
    const VALUE: i64 = i64::MAX;
    let expected = Hertz::from(u64::try_from(i64::MAX).unwrap());

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

// // This test is commented out because it fails to compile when it passes.  Uncomment test to verify behavior.
// #[test]
// #[should_panic(expected = "Hertz value overflows `HertzInner` type.")]
// fn from_max_i128() {
//     // Given
//     const VALUE: i128 = i128::MAX;
//
//     // When
//     const _: Hertz = Hertz::from(VALUE);
//
//     // Then
//     unreachable!();
// }

#[test]
fn from_max_isize() {
    // Given
    const VALUE: isize = isize::MAX;
    let expected = Hertz::from(u64::try_from(isize::MAX).unwrap());

    // When
    const RESULT: Hertz = Hertz::from(VALUE);

    // Then
    assert!(RESULT == expected);
}

// #[cfg(target_pointer_width = "128")]
// // This test is commented out because it fails to compile when it passes.  Uncomment test to verify behavior.
// #[test]
// #[should_panic(expected = "Hertz value overflows `HertzInner` type.")]
// fn from_max_isize() {
//     // Given
//     const VALUE: isize = isize::MAX;
//
//     // When
//     const _: Hertz = Hertz::from(VALUE);
//
//     // Then
//     unreachable!();
// }
