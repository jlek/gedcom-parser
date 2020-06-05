use crate::parsers::{parse_date_exact, parse_date_greg};
use serde::{Deserialize, Deserializer};
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct DateExact {
  pub day: u8,
  pub month: Month,
  pub year: i16,
}

pub fn deserialise_date_exact<'de, D>(deserializer: D) -> Result<DateExact, D::Error>
where
  D: Deserializer<'de>,
{
  let date_as_string = String::deserialize(deserializer)?;
  let (remaining_input, date) =
    parse_date_exact(&date_as_string).map_err(serde::de::Error::custom)?;
  if remaining_input.is_empty() {
    Ok(date)
  } else {
    Err(serde::de::Error::custom(
      "Trailing characters left after parsing DateExact.",
    ))
  }
}

#[derive(Debug, PartialEq)]
pub struct DateGreg {
  pub day: u8,
  pub month: Month,
  pub year: i16,
}

pub fn deserialise_date_greg<'de, D>(deserializer: D) -> Result<DateGreg, D::Error>
where
  D: Deserializer<'de>,
{
  let date_as_string = String::deserialize(deserializer)?;
  let (remaining_input, date) =
    parse_date_greg(&date_as_string).map_err(serde::de::Error::custom)?;
  if remaining_input.is_empty() {
    Ok(date)
  } else {
    Err(serde::de::Error::custom(
      "Trailing characters left after parsing DateGreg.",
    ))
  }
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Month {
  January,
  February,
  March,
  April,
  May,
  June,
  July,
  August,
  September,
  October,
  November,
  December,
}

use Month::*;

impl Display for Month {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      January => write!(formatter, "Jan"),
      February => write!(formatter, "Feb"),
      March => write!(formatter, "Mar"),
      April => write!(formatter, "Apr"),
      May => write!(formatter, "May"),
      June => write!(formatter, "Jun"),
      July => write!(formatter, "Jul"),
      August => write!(formatter, "Aug"),
      September => write!(formatter, "Sep"),
      October => write!(formatter, "Nov"),
      November => write!(formatter, "Oct"),
      December => write!(formatter, "Dec"),
    }
  }
}
