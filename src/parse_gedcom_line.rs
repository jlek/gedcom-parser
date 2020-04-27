use nom::{
  bytes::complete::{tag as specific_characters, take_while, take_while_m_n},
  combinator::map_res,
  IResult,
};

const DELIM: &str = " "; // Should be 0x20

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

fn is_decimal_digit(character: char) -> bool {
  character.is_digit(10)
}

fn parse_level(input: &str) -> IResult<&str, u8> {
  map_res(take_while_m_n(1, 2, is_decimal_digit), from_decimal)(input)
}

// ===
// Tag
// ===

fn is_alphanumeric(character: char) -> bool {
  character.is_alphanumeric()
}

fn parse_tag(input: &str) -> IResult<&str, &str> {
  take_while_m_n(1, 32, is_alphanumeric)(input)
}

// ==========
// GedcomLine
// ==========

fn parse_gedcom_line(input: &str) -> IResult<&str, GedcomLine> {
  let (input, level) = parse_level(input)?;
  let (input, _) = specific_characters(DELIM)(input)?;
  let (input, tag) = parse_tag(input)?;
  let (input, _) = specific_characters(DELIM)(input)?;
  let value = "".to_string();

  Ok((
    input,
    GedcomLine {
      level,
      tag: tag.to_string(),
      value,
    },
  ))
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
fn parse_tag_valid() {
  // Arrange
  let input = "TAGG Some value";

  // Act
  let (remaining_text, tag) = parse_tag(input).unwrap();

  // Assert
  assert_eq!(remaining_text, " Some value");
  assert_eq!(tag, "TAGG");
}

#[test]
fn parse_tag_too_long() {
  // Arrange
  let input = "TAGTAGTAGTAGTAGTAGTAGTAGTAGTAGTAG Some value"; // Tag is more than 32 characters

  // Act
  let (remaining_text, tag) = parse_tag(input).unwrap();

  // Assert
  assert_eq!(remaining_text, "G Some value");
  assert_eq!(tag, "TAGTAGTAGTAGTAGTAGTAGTAGTAGTAGTA");
}

#[test]
#[ignore]
fn parse_gedcom_line_valid() {
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
