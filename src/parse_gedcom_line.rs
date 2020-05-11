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

fn from_decimal(input: &str) -> Result<u8, std::num::ParseIntError> {
  u8::from_str_radix(input, 10)
}

fn is_decimal_digit(character: char) -> bool {
  character.is_digit(10)
}

fn parse_level(input: &str) -> IResult<&str, u8> {
  map_res(take_while_m_n(1, 2, is_decimal_digit), from_decimal)(input)
}

fn parse_specific_level<'level, 'input>(
  level: u8,
) -> impl Fn(&'input str) -> IResult<&'input str, u8> {
  if level >= 100 {
    panic!(
      "Level {} is too large to parse; Gedcom only support two-digits levels.",
      level
    );
  }

  move |input| map(specific_characters(&level.to_string()[..]), move |_| level)(input)
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

fn parse_specific_tag<'input>(
  tag: &str,
) -> impl Fn(&'input str) -> IResult<&'input str, &'input str> {
  if tag.len() > 32 {
    panic!(
      "Cannot parse tag \"{}\", because it is too long - tags can not be longer than 32 characters",
      tag
    );
  }
  let tag_owned = tag.to_owned();
  move |input| specific_characters(&tag_owned[..])(input)
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

fn parse_line_value_m_n<'input>(
  min_length: u8,
  max_length: u8,
) -> impl Fn(&'input str) -> IResult<&'input str, &'input str> {
  take_while_m_n(min_length as usize, max_length as usize, is_not_terminator)
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

#[derive(Debug, PartialEq)]
pub struct GedcomLine<'tag, 'value> {
  level: u8,
  tag: &'tag str,
  value: Option<&'value str>,
}

#[derive(Debug, PartialEq)]
pub struct GedcomLineWithoutValue<'tag> {
  level: u8,
  tag: &'tag str,
}

#[derive(Debug, PartialEq)]
pub struct GedcomLineWithValue<'tag, 'value> {
  level: u8,
  tag: &'tag str,
  pub value: &'value str,
}

impl From<GedcomLineWithValue<'_, '_>> for String {
  fn from(line: GedcomLineWithValue) -> Self {
    line.value.to_string()
  }
}

fn parse_gedcom_line(input: &str) -> IResult<&str, GedcomLine> {
  let (remaining_input, (level, _delim, tag, value, _terminator)) = tuple((
    parse_level,
    parse_delim,
    parse_tag,
    opt(preceded(parse_delim, parse_line_value)),
    parse_terminator,
  ))(input)?;

  Ok((remaining_input, GedcomLine { level, tag, value }))
}

pub fn parse_gedcom_line_without_value<'input>(
  level: u8,
  tag: &str,
) -> impl Fn(&'input str) -> IResult<&'input str, GedcomLineWithoutValue<'input>> {
  map(
    tuple((
      parse_specific_level(level),
      parse_delim,
      parse_specific_tag(tag),
      parse_terminator,
    )),
    |(level, _delim, tag, _terminator)| GedcomLineWithoutValue { tag, level },
  )
}

pub fn parse_gedcom_line_with_value<'input>(
  level: u8,
  tag: &str,
  min_line_length: u8,
  max_line_length: u8,
) -> impl Fn(&'input str) -> IResult<&'input str, GedcomLineWithValue<'input, 'input>> {
  map(
    tuple((
      parse_specific_level(level),
      parse_delim,
      parse_specific_tag(tag),
      parse_delim,
      parse_line_value_m_n(min_line_length, max_line_length),
      parse_terminator,
    )),
    |(level, _delim, tag, _delim_2, value, _terminator)| GedcomLineWithValue { level, tag, value },
  )
}

pub fn parse_gedcom_line_value_only<'input>(
  level: u8,
  tag: &str,
  min_line_length: u8,
  max_line_length: u8,
) -> impl Fn(&'input str) -> IResult<&'input str, String> {
  map(
    parse_gedcom_line_with_value(level, tag, min_line_length, max_line_length),
    String::from,
  )
}

#[test]
fn parse_gedcom_line_without_value_valid() {
  // Arrange
  let input = "0 TAG\n";

  // Act
  let (remaining_text, gedcom_line) = parse_gedcom_line_without_value(0, "TAG")(input)
    .expect("In this test, it should not return an error result.");

  // Assert
  assert_eq!(remaining_text, "");
  assert_eq!(
    gedcom_line,
    GedcomLineWithoutValue {
      level: 0,
      tag: "TAG",
    }
  );
}

#[test]
fn parse_gedcom_line_with_value_valid() {
  // Arrange
  let input = "0 TAG some value\n";

  // Act
  let (remaining_text, gedcom_line) = parse_gedcom_line_with_value(0, "TAG", 1, 10)(input)
    .expect("In this test, it should not return an error result.");

  // Assert
  assert_eq!(remaining_text, "");
  assert_eq!(
    gedcom_line,
    GedcomLineWithValue {
      level: 0,
      tag: "TAG",
      value: "some value"
    }
  );
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
