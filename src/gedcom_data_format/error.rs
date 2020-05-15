use serde::{de, ser};
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
  Message(String),
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
      Error::TrailingCharacters => formatter.write_str("Trailing characters were left"),
    }
  }
}

impl std::error::Error for Error {}
