use super::*;
use assert2::assert;

#[test]
fn writing_data_to_a_zero_capacity_buffer_fails() {
    // Given
    const BUFFER_CAPACITY: usize = 0;
    let mut buffer = [0_u8; BUFFER_CAPACITY];
    let data = "Hello";
    let mut sut = ConcatStr::new(&mut buffer);

    // When
    let result = sut.write_str(data);

    // Then
    assert!(let Err(core::fmt::Error) = result);
}

#[test]
fn writing_data_exceeding_a_non_zero_buffer_capacity_fails() {
    // Given
    const BUFFER_CAPACITY: usize = 4;
    let mut buffer = [0_u8; BUFFER_CAPACITY];
    let data = "Hello";
    let mut sut = ConcatStr::new(&mut buffer);

    // When
    let result = sut.write_str(data);

    // Then
    assert!(let Err(core::fmt::Error) = result);
}

#[test]
fn writing_data_under_buffer_capacity_succeeds() {
    // Given
    const BUFFER_CAPACITY: usize = 42;
    let mut buffer = [0_u8; BUFFER_CAPACITY];
    let data = "Hello";
    let mut sut = ConcatStr::new(&mut buffer);

    // When
    let result = sut.write_str(data);

    // Then
    assert!(let Ok(()) = result);
}

#[test]
fn writing_data_at_buffer_capacity_succeeds() {
    // Given
    const BUFFER_CAPACITY: usize = 5;
    let mut buffer = [0_u8; BUFFER_CAPACITY];
    let data = "Hello";
    let mut sut = ConcatStr::new(&mut buffer);

    // When
    let result = sut.write_str(data);

    // Then
    assert!(let Ok(()) = result);
}

#[test]
fn writing_data_two_times_exceeding_a_non_zero_buffer_capacity_fails_both_times() {
    // Given
    const BUFFER_CAPACITY: usize = 4;
    let mut buffer = [0_u8; BUFFER_CAPACITY];
    let data = "Hello";
    let data_2 = ", world!";
    let mut sut = ConcatStr::new(&mut buffer);
    sut.write_str(data).unwrap_err();

    // When
    let result = sut.write_str(data_2);

    // Then
    assert!(let Err(core::fmt::Error) = result);
}

#[test]
fn writing_data_two_times_first_exceeding_then_fitting_a_non_zero_buffer_capacity_fails() {
    // Given
    const BUFFER_CAPACITY: usize = 4;
    let mut buffer = [0_u8; BUFFER_CAPACITY];
    let data = "Hello";
    let data_2 = "Hi!";
    let mut sut = ConcatStr::new(&mut buffer);
    sut.write_str(data).unwrap_err();

    // When
    let result = sut.write_str(data_2);

    // Then
    assert!(let Err(core::fmt::Error) = result);
}

#[test]
fn writing_data_two_times_staying_under_buffer_capacity_succeeds() {
    // Given
    const BUFFER_CAPACITY: usize = 999;
    let mut buffer = [0_u8; BUFFER_CAPACITY];
    let data = "Hello";
    let data_2 = "world";
    let mut sut = ConcatStr::new(&mut buffer);
    sut.write_str(data).unwrap();

    // When
    let result = sut.write_str(data_2);

    // Then
    assert!(let Ok(()) = result);
}
