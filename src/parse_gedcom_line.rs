use nom::{bytes::complete::take_while_m_n, combinator::map_res, IResult};

#[derive(Debug, PartialEq)]
pub struct GedcomLine {
  level: u8,
  tag: String,
  value: String,
}

// =====
// Level
// =====

fn from_decimal(input: &str) -> Result<u8, std::num::ParseIntError> {
  u8::from_str_radix(input, 10)
}

fn is_decimal_digit(c: char) -> bool {
  c.is_digit(10)
}

fn parse_level(input: &str) -> IResult<&str, u8> {
  map_res(take_while_m_n(1, 2, is_decimal_digit), from_decimal)(input)
}

// ==========
// GedcomLine
// ==========

fn parse_gedcom_line(input: &str) -> IResult<&str, GedcomLine> {
  let (input, level) = parse_level(input)?;
  let tag = "".to_string();
  let value = "".to_string();

  Ok((input, GedcomLine { level, tag, value }))
}

#[test]
fn parse_level_single_digit() {
  // Arrange
  let input = "0 TAGG Some value";

  // Act
  let (remaining_text, level) = parse_level(input).unwrap();

  // Assert
  assert_eq!(remaining_text, " TAGG Some value");
  assert_eq!(level, 0);
}

#[test]
fn parse_level_two_digits() {
  // Arrange
  let input = "42 TAGG Some value";

  // Act
  let (remaining_text, level) = parse_level(input).unwrap();

  // Assert
  assert_eq!(remaining_text, " TAGG Some value");
  assert_eq!(level, 42);
}

#[test]
fn parse_level_three_digits() {
  // Arrange
  let input = "123 TAGG Some value";

  // Act
  let (remaining_text, level) = parse_level(input).unwrap();

  // Assert
  assert_eq!(remaining_text, "3 TAGG Some value");
  assert_eq!(level, 12);
}

#[test]
fn parse_level_invalid_input() {
  // Arrange
  let input = "invalid input";

  // Act
  let result = parse_level(input);

  // Assert
  assert!(result.is_err());
}

#[test]
#[ignore]
fn parse_color() {
  // Arrange
  let input = "0 TAGG Some value";

  // Act
  let (remaining_text, gedcom_line) = parse_gedcom_line(input).unwrap();

  // Assert
  assert_eq!(remaining_text, "");
  assert_eq!(
    gedcom_line,
    GedcomLine {
      level: 0,
      tag: "TAGG".to_string(),
      value: "Some value".to_string(),
    }
  );
}
