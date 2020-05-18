use serde::{de, ser};
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
  Message(String),
  InvalidGedcomLine,
  ExpectedGedcomLineWithValue,
  ExpectedMap,
  ExpectedMapEnd,
  TrailingCharacters,
}

impl ser::Error for Error {
  fn custom<T: Display>(msg: T) -> Self {
    Error::Message(msg.to_string())
  }
}

impl de::Error for Error {
  fn custom<T: Display>(msg: T) -> Self {
    Error::Message(msg.to_string())
  }
}

impl Display for Error {
  fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
    match self {
      Error::Message(msg) => formatter.write_str(msg),
      Error::InvalidGedcomLine => formatter.write_str("Invalid Gedcom line"),
      Error::ExpectedGedcomLineWithValue => formatter.write_str("Expected Gedcom Line with value"),
      Error::ExpectedMap => formatter.write_str("Expected map"),
      Error::ExpectedMapEnd => {
        formatter.write_str("Expected map to end (next level should be current level - 1)")
      }
      Error::TrailingCharacters => formatter.write_str("Trailing characters were left"),
    }
  }
}

impl std::convert::From<nom::Err<(&str, nom::error::ErrorKind)>> for Error {
  fn from(_nom_error: nom::Err<(&str, nom::error::ErrorKind)>) -> Error {
    Error::InvalidGedcomLine
  }
}

impl std::error::Error for Error {}
