use super::*;
use assert2::assert;

#[test]
fn a_zero_capacity_buffer_shows_an_empty_str() {
    // Given
    const BUFFER_CAPACITY: usize = 0;
    let mut buffer = [0_u8; BUFFER_CAPACITY];
    let data = "";
    let expected = Ok("");

    // When
    let result = show(&mut buffer, format_args!("{}", data));

    // Then
    assert!(result == expected);
}

#[test]
fn an_empty_non_zero_capacity_buffer_shows_an_empty_str() {
    // Given
    const BUFFER_CAPACITY: usize = 42;
    let mut buffer = [0_u8; BUFFER_CAPACITY];
    let data = "";
    let expected = Ok("");

    // When
    let result = show(&mut buffer, format_args!("{}", data));

    // Then
    assert!(result == expected);
}

#[test]
fn a_non_empty_buffer_at_capacity_shows_expected_str() {
    // Given
    const BUFFER_CAPACITY: usize = 5;
    let mut buffer = [0_u8; BUFFER_CAPACITY];
    let data = "Hello";
    let expected = Ok("Hello");

    // When
    let result = show(&mut buffer, format_args!("{}", data));

    // Then
    assert!(result == expected);
}

#[test]
fn a_non_empty_buffer_under_capacity_shows_expected_str() {
    // Given
    const BUFFER_CAPACITY: usize = 42;
    let mut buffer = [0_u8; BUFFER_CAPACITY];
    let data = "Hello";
    let expected = Ok("Hello");

    // When
    let result = show(&mut buffer, format_args!("{}", data));

    // Then
    assert!(result == expected);
}

#[test]
fn data_written_two_times_staying_under_buffer_capacity_shows_concatenated_data() {
    // Given
    const BUFFER_CAPACITY: usize = 999;
    let mut buffer = [0_u8; BUFFER_CAPACITY];
    let data = "Hello";
    let data_2 = "world";
    let expected = Ok("Hello, world!");
    let mut sut = ConcatStr::new(&mut buffer);
    sut.write_str(data).unwrap();

    // When
    let result = show(&mut buffer, format_args!("{}, {}!", data, data_2));

    // Then
    assert!(result == expected);
}
