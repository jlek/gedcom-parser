use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct SubmissionRecord<'input> {
  #[serde(rename = "NAME")]
  pub name: &'input str,
}
