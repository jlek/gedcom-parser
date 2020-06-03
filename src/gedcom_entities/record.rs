use super::{Header, Individual, SubmissionRecord};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub enum Record<'input> {
  #[serde(borrow, rename = "HEAD")]
  Header(Header<'input>),
  #[serde(borrow, rename = "SUBM")]
  SubmissionRecord(SubmissionRecord<'input>),
  #[serde(rename = "INDI")]
  Individual(Individual<'input>),
  #[serde(rename = "TRLR")]
  Trailer,
}
