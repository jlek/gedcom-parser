use crate::parsers::parse_time_value;
use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub struct TimeValue {
  pub hours: u8,
  pub minutes: u8,
  pub seconds: Option<u8>,
}

pub fn deserialise_time_value<'de, D>(deserializer: D) -> Result<TimeValue, D::Error>
where
  D: Deserializer<'de>,
{
  let date_as_string = String::deserialize(deserializer)?;
  let (remaining_input, time) =
    parse_time_value(&date_as_string).map_err(serde::de::Error::custom)?;
  if remaining_input.is_empty() {
    Ok(time)
  } else {
    Err(serde::de::Error::custom(
      "Trailing characters left after parsing TimeValue.",
    ))
  }
}
