use super::*;
use assert2::assert;

#[test]
fn constructing_with_a_zero_capacity_buffer_returns_expected_capacity() {
    // Given
    const BUFFER_CAPACITY: usize = 0;
    let mut buffer = [0_u8; BUFFER_CAPACITY];
    let expected = BUFFER_CAPACITY;
    let sut = ConcatStr::new(&mut buffer);

    // When
    let result = sut.capacity();

    // Then
    assert!(result == expected);
}

#[test]
fn constructing_with_a_non_zero_capacity_buffer_returns_expected_capacity() {
    // Given
    const BUFFER_CAPACITY: usize = 42;
    let mut buffer = [0_u8; BUFFER_CAPACITY];
    let expected = BUFFER_CAPACITY;
    let sut = ConcatStr::new(&mut buffer);

    // When
    let result = sut.capacity();

    // Then
    assert!(result == expected);
}
