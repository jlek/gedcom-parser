use nom::{
  branch::alt,
  bytes::complete::{tag as specific_characters, take_till1, take_while_m_n},
  combinator::{map_res, opt},
  IResult,
};

const DELIM: &str = " "; // Should be 0x20

#[derive(Debug, PartialEq)]
pub struct GedcomLine<'tag, 'value> {
  level: u8,
  tag: &'tag str,
  value: Option<&'value str>,
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
// Line value
// ==========

fn is_terminator(character: char) -> bool {
  parse_terminator(&character.to_string()).is_ok()
}

fn parse_optional_line_value(input: &str) -> IResult<&str, Option<&str>> {
  opt(take_till1(is_terminator))(input)
}

// ==========
// Whitespace
// ==========

fn parse_delim(input: &str) -> IResult<&str, &str> {
  specific_characters(DELIM)(input)
}

fn parse_optional_delim(input: &str) -> IResult<&str, Option<&str>> {
  opt(parse_delim)(input)
}

fn parse_terminator(input: &str) -> IResult<&str, &str> {
  alt((
    specific_characters("\r\n"),
    specific_characters("\n\r"),
    specific_characters("\r"),
    specific_characters("\n"),
  ))(input)
}

// ==========
// GedcomLine
// ==========

fn parse_gedcom_line(input: &str) -> IResult<&str, GedcomLine> {
  let (input, level) = parse_level(input)?;
  let (input, _) = parse_delim(input)?;
  let (input, tag) = parse_tag(input)?;
  let (input, _) = parse_optional_delim(input)?;
  let (input, value) = parse_optional_line_value(input)?;
  let (input, _) = parse_terminator(input)?;

  Ok((input, GedcomLine { level, tag, value }))
}

#[test]
fn parse_gedcom_line_valid() {
  // Arrange
  let input = "0 TAG Some value\n";

  // Act
  let (remaining_text, gedcom_line) = parse_gedcom_line(input).unwrap();

  // Assert
  assert_eq!(remaining_text, "");
  assert_eq!(
    gedcom_line,
    GedcomLine {
      level: 0,
      tag: "TAG",
      value: Some("Some value"),
    }
  );
}

#[test]
fn parse_gedcom_line_double_digit_level() {
  // Arrange
  let input = "10 TAG Some value\n";

  // Act
  let (remaining_text, gedcom_line) = parse_gedcom_line(input).unwrap();

  // Assert
  assert_eq!(remaining_text, "");
  assert_eq!(
    gedcom_line,
    GedcomLine {
      level: 10,
      tag: "TAG",
      value: Some("Some value"),
    }
  );
}

#[test]
fn parse_gedcom_line_no_line_value() {
  // Arrange
  let input = "0 TAG\n";

  // Act
  let (remaining_text, gedcom_line) = parse_gedcom_line(input).unwrap();

  // Assert
  assert_eq!(remaining_text, "");
  assert_eq!(
    gedcom_line,
    GedcomLine {
      level: 0,
      tag: "TAG",
      value: None,
    }
  );
}

#[test]
fn parse_gedcom_line_no_level() {
  // Arrange
  let input = "TAG Some value\n";

  // Act
  let result = parse_gedcom_line(input);

  // Assert
  assert!(result.is_err());
}

#[test]
fn parse_gedcom_line_level_too_long() {
  // Arrange
  let input = "123 TAG Some value\n";

  // Act
  let result = parse_gedcom_line(input);

  // Assert
  assert!(result.is_err());
}

#[test]
fn parse_gedcom_line_tag_too_long() {
  // Arrange
  let input = "0 TAGTAGTAGTAGTAGTAGTAGTAGTAGTAGTAG Some value"; // Tag is more than 32 characters

  // Act
  let result = parse_gedcom_line(input);

  // Assert
  assert!(result.is_err());
}
