use super::utilities::{from_decimal, is_decimal_digit};
use nom::{
  branch::alt,
  bytes::complete::{tag as specific_characters, take_till1, take_while_m_n},
  combinator::{map, map_res, opt},
  sequence::{preceded, tuple},
  IResult,
};

// =====
// Level
// =====

fn parse_level(input: &str) -> IResult<&str, u8> {
  map_res(take_while_m_n(1, 2, is_decimal_digit), from_decimal)(input)
}

// ===
// Tag
// ===

fn is_alphanumeric_or_underscore(character: char) -> bool {
  character.is_alphanumeric() || character == '_'
}

fn parse_tag(input: &str) -> IResult<&str, &str> {
  take_while_m_n(1, 32, is_alphanumeric_or_underscore)(input)
}

// ==========
// Line value
// ==========

fn is_terminator(character: char) -> bool {
  parse_terminator(&character.to_string()).is_ok()
}

fn is_not_terminator(character: char) -> bool {
  !is_terminator(character)
}

fn parse_line_value(input: &str) -> IResult<&str, &str> {
  take_till1(is_terminator)(input)
}

// ============
// Deliminators
// ============

struct Delim {}

struct InputIsNotADelimError<'input> {
  input: &'input str,
}

const DELIM: &str = " "; // Should be 0x20

fn from_delim(input: &str) -> Result<Delim, InputIsNotADelimError> {
  match input {
    DELIM => Ok(Delim {}),
    _ => Err(InputIsNotADelimError { input }),
  }
}

fn parse_delim(input: &str) -> IResult<&str, Delim> {
  map_res(specific_characters(DELIM), from_delim)(input)
}

// ===========
// Terminators
// ===========

struct Terminator {}

struct InputIsNotATerminatorError<'input> {
  input: &'input str,
}

fn from_terminator(input: &str) -> Result<Terminator, InputIsNotATerminatorError> {
  match input {
    "\r\n" | "\n\r" | "\n" | "\r" => Ok(Terminator {}),
    _ => Err(InputIsNotATerminatorError { input }),
  }
}

fn parse_terminator(input: &str) -> IResult<&str, Terminator> {
  map_res(
    alt((
      specific_characters("\r\n"),
      specific_characters("\r\n"),
      specific_characters("\r"),
      specific_characters("\n"),
    )),
    from_terminator,
  )(input)
}

// ==========
// GedcomLine
// ==========

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GedcomLine<'input> {
  pub level: u8,
  pub tag: &'input str,
  pub value: Option<&'input str>,
}

pub fn parse_gedcom_line(input: &str) -> IResult<&str, GedcomLine> {
  let (remaining_input, (level, _delim, tag, value, _terminator)) = tuple((
    parse_level,
    parse_delim,
    parse_tag,
    opt(preceded(parse_delim, parse_line_value)),
    parse_terminator,
  ))(input)?;

  Ok((remaining_input, GedcomLine { level, tag, value }))
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
