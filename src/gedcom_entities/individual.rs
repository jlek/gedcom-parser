use super::{
  deserialise_date_exact, deserialise_date_greg, deserialise_time_value, DateExact, DateGreg,
  TimeValue,
};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, PartialEq)]
pub struct Individual<'input> {
  #[serde(borrow, rename = "NAME")]
  pub name: Vec<PersonalName<'input>>,
  #[serde(rename = "SEX")]
  pub sex: Option<SexValue>,
  // TODO model events more like the Gedcom spec - should probably be a vec of some enum.
  #[serde(rename = "BIRT")]
  pub birth_event: Option<BirthEvent<'input>>,
  #[serde(rename = "_UID")]
  pub uid: Option<&'input str>,
  #[serde(rename = "CHAN")]
  pub change_date: Option<ChangeDate>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PersonalName<'input> {
  #[serde(rename = "GIVN")]
  pub given_name: &'input str,
  #[serde(rename = "SURN")]
  pub surname: &'input str,
  #[serde(rename = "_PRIM", deserialize_with = "deserialize_boolean")]
  pub is_primary: bool,
}

pub fn deserialize_boolean<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
  D: Deserializer<'de>,
{
  let bool_as_string = String::deserialize(deserializer)?;
  if bool_as_string == "Y" {
    Ok(true)
  } else if bool_as_string == "N" {
    Ok(false)
  } else {
    Err(serde::de::Error::custom("Failed to parse boolean"))
  }
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum SexValue {
  #[serde(rename = "M")]
  Male,
  #[serde(rename = "F")]
  Female,
  #[serde(rename = "U")]
  Undetermined,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct BirthEvent<'input> {
  #[serde(rename = "_PRIM", deserialize_with = "deserialize_boolean")]
  pub is_primary: bool,
  #[serde(rename = "DATE", deserialize_with = "deserialise_date_greg")]
  pub date: DateGreg,
  #[serde(rename = "PLAC")]
  pub place: Option<&'input str>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ChangeDate {
  #[serde(rename = "DATE")]
  pub date_time: ChangeDateDateTime,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ChangeDateDateTime {
  #[serde(rename = "DATE", deserialize_with = "deserialise_date_exact")]
  pub date: DateExact,
  #[serde(rename = "TIME", deserialize_with = "deserialise_time_value")]
  pub time: TimeValue,
}
