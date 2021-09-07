use super::*;
use assert2::assert;

#[test]
fn constructing_with_a_zero_capacity_buffer_returns_expected_len() {
    // Given
    const BUFFER_CAPACITY: usize = 0;
    let mut buffer = [0_u8; BUFFER_CAPACITY];
    let expected = 0;
    let sut = ConcatStr::new(&mut buffer);

    // When
    let result = sut.len();

    // Then
    assert!(result == expected);
}

#[test]
fn constructing_with_a_non_zero_capacity_buffer_returns_expected_len() {
    // Given
    const BUFFER_CAPACITY: usize = 42;
    let mut buffer = [0_u8; BUFFER_CAPACITY];
    let expected = 0;
    let sut = ConcatStr::new(&mut buffer);

    // When
    let result = sut.len();

    // Then
    assert!(result == expected);
}

#[test]
fn writing_nothing_to_a_zero_capacity_buffer_returns_expected_len() {
    // Given
    const BUFFER_CAPACITY: usize = 0;
    let mut buffer = [0_u8; BUFFER_CAPACITY];
    let data = "";
    let expected = 0;
    let sut = {
        let mut tmp = ConcatStr::new(&mut buffer);
        tmp.write_str(data).unwrap();
        tmp
    };

    // When
    let result = sut.len();

    // Then
    assert!(result == expected);
}

#[test]
fn writing_nothing_to_a_non_zero_capacity_buffer_returns_expected_len() {
    // Given
    const BUFFER_CAPACITY: usize = 42;
    let mut buffer = [0_u8; BUFFER_CAPACITY];
    let data = "";
    let expected = 0;
    let sut = {
        let mut tmp = ConcatStr::new(&mut buffer);
        tmp.write_str(data).unwrap();
        tmp
    };

    // When
    let result = sut.len();

    // Then
    assert!(result == expected);
}

#[test]
fn writing_data_to_exactly_fill_the_buffer_capacity_returns_expected_len() {
    // Given
    const BUFFER_CAPACITY: usize = 5;
    let mut buffer = [0_u8; BUFFER_CAPACITY];
    let data = "Hello";
    let expected = 5;
    let sut = {
        let mut tmp = ConcatStr::new(&mut buffer);
        tmp.write_str(data).unwrap();
        tmp
    };

    // When
    let result = sut.len();

    // Then
    assert!(result == expected);
}
