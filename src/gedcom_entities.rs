#[derive(Debug, PartialEq)]
pub struct Header {
  pub source: Source,
  pub receiving_sytem_name: String,
  pub transmission_date_time: TransmissionDateTime,
  pub submission_record_id: String,
  pub file_name: String,
  pub gedcom: Gedcom,
  pub character_set: CharacterSet,
  pub language: Option<Language>,
}

#[derive(Debug, PartialEq)]
pub struct Source {
  pub id: String,
  pub name: Option<String>,
  pub version: Option<String>,
  pub business: Option<Business>,
}

#[derive(Debug, PartialEq)]
pub struct Business {
  pub name: String,
  pub address_structure: Option<AddressStructure>,
}

#[derive(Debug, PartialEq)]
pub struct AddressStructure {
  pub address: Address,
  pub web_page: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct Address {
  pub address_line: String,
  pub city: Option<String>,
  pub post_code: Option<String>,
  pub country: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct TransmissionDateTime {
  pub date: ExactDate,
  pub time: TimeValue,
}

#[derive(Debug, PartialEq)]
pub struct ExactDate {
  pub day: u8,
  pub month: Month,
  pub year: i16,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Gedcom {
  pub version_number: String,
  pub form: GedcomForm,
}

#[derive(Debug, PartialEq)]
pub enum GedcomForm {
  LineageLinked,
}

#[derive(Debug, PartialEq)]
pub enum CharacterSet {
  Ansel,
  Utf8,
  Unicode,
  Ascii,
}

#[derive(Debug, PartialEq)]
pub enum Language {
  English,
}
