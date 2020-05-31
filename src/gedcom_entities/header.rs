use crate::parsers::{parse_date_exact, parse_time_value};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, PartialEq)]
pub struct Header<'input> {
  #[serde(rename = "SOUR")]
  pub source: Source<'input>,
  #[serde(rename = "DEST")]
  pub receiving_sytem_name: &'input str,
  #[serde(rename = "DATE")]
  pub transmission_date_time: TransmissionDateTime,
  #[serde(rename = "SUBM")]
  pub submission_record_id: &'input str,
  #[serde(rename = "FILE")]
  pub file_name: &'input str,
  #[serde(rename = "GEDC")]
  pub gedcom: Gedcom<'input>,
  #[serde(rename = "CHAR")]
  pub character_set: CharacterSet,
  #[serde(rename = "LANG")]
  pub language: Option<Language>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Source<'input> {
  #[serde(rename = "SOUR")]
  pub id: &'input str,
  #[serde(rename = "NAME")]
  pub name: Option<&'input str>,
  #[serde(rename = "VERS")]
  pub version: Option<&'input str>,
  #[serde(rename = "CORP")]
  pub business: Option<Business<'input>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Business<'input> {
  #[serde(rename = "CORP")]
  pub name: &'input str,
  #[serde(rename = "ADDR")]
  pub address: Option<Address<'input>>,
  #[serde(rename = "WWW")]
  pub web_page: Vec<&'input str>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Address<'input> {
  #[serde(rename = "ADDR")]
  pub address_line: &'input str,
  #[serde(rename = "CITY")]
  pub city: Option<&'input str>,
  #[serde(rename = "POST")]
  pub post_code: Option<&'input str>,
  #[serde(rename = "CTRY")]
  pub country: Option<&'input str>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TransmissionDateTime {
  #[serde(rename = "DATE", deserialize_with = "deserialise_date_exact")]
  pub date: DateExact,
  #[serde(rename = "TIME", deserialize_with = "deserialise_time_value")]
  pub time: TimeValue,
}

#[derive(Debug, PartialEq)]
pub struct DateExact {
  pub day: u8,
  pub month: Month,
  pub year: i16,
}

fn deserialise_date_exact<'de, D>(deserializer: D) -> Result<DateExact, D::Error>
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

#[derive(Debug, PartialEq)]
pub struct TimeValue {
  pub hours: u8,
  pub minutes: u8,
  pub seconds: Option<u8>,
}

fn deserialise_time_value<'de, D>(deserializer: D) -> Result<TimeValue, D::Error>
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

#[derive(Debug, Deserialize, PartialEq)]
pub struct Gedcom<'input> {
  #[serde(rename = "VERS")]
  pub version_number: &'input str,
  #[serde(rename = "FORM")]
  pub form: GedcomForm,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum GedcomForm {
  #[serde(rename = "LINEAGE-LINKED")]
  LineageLinked,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum CharacterSet {
  #[serde(rename = "ANSEL")]
  Ansel,
  #[serde(rename = "UTF-8")]
  Utf8,
  #[serde(rename = "UNICODE")]
  Unicode,
  #[serde(rename = "ASCII")]
  Ascii,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Language {
  English,
}
