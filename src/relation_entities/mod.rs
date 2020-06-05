use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FamilyTree<'input> {
  #[serde(rename = "Persons")]
  pub persons: Vec<Person<'input>>,
  #[serde(rename = "Familys")]
  pub familys: Vec<()>,
  #[serde(rename = "Childs")]
  pub childs: Vec<()>,
  #[serde(rename = "SourceRepos")]
  pub source_repos: Vec<()>,
  #[serde(rename = "MasterSources")]
  pub master_sources: Vec<()>,
  #[serde(rename = "Medias")]
  pub medias: Vec<()>,
  #[serde(rename = "FactTypes")]
  pub fact_types: Vec<()>,
}

#[derive(Debug, Serialize)]
pub struct Person<'input> {
  #[serde(rename = "IsLiving")]
  pub is_living: bool,
  #[serde(rename = "Gender")]
  pub gender: u8,
  #[serde(rename = "DateCreated")]
  pub date_created: String,
  #[serde(rename = "Names")]
  pub names: Vec<Name<'input>>,
  #[serde(rename = "Facts")]
  pub facts: Vec<Fact<'input>>,
}

#[derive(Debug, Serialize)]
pub struct Name<'input> {
  #[serde(rename = "FactTypeId")]
  pub fact_type_id: u16,
  #[serde(rename = "GivenNames")]
  pub given_names: &'input str,
  #[serde(rename = "Surnames")]
  pub surnames: &'input str,
}

#[derive(Debug, Serialize)]
pub struct Fact<'input> {
  #[serde(rename = "FactTypeId")]
  pub fact_type_id: u16,
  #[serde(rename = "DateDetail")]
  pub date_detail: String,
  #[serde(rename = "Place")]
  pub place: Place<'input>,
  #[serde(rename = "Preferred")]
  pub preferred: bool,
}

#[derive(Debug, Serialize)]
pub struct Place<'input> {
  #[serde(rename = "PlaceName")]
  pub place_name: Option<&'input str>,
}
