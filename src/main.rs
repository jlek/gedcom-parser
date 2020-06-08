// Not sure what this does, but apparently it's now needed to compile:
#![type_length_limit = "8388608"]

extern crate nom;
mod gedcom_data_format;
mod gedcom_entities;
mod parsers;
mod relation_entities;
mod tests;
mod transform_gedcom_to_relation;

use gedcom_data_format::from_str;
use gedcom_entities::Record;
use serde_json::to_string_pretty;
use std::fs::{read_to_string, write};
use transform_gedcom_to_relation::transform_gedcom_to_relation;

const SOURCE_FILE_PATH: &'static str = "src/tests/one-node.ged";
const TARGET_FILE_PATH: &'static str = "src/tests/one-node.json";

fn main() {
  let file = read_to_string(SOURCE_FILE_PATH).expect("File should exist");
  let records: Vec<Record> = from_str(&file).expect("program no crashy");
  let family_tree = transform_gedcom_to_relation(&records);
  let json = to_string_pretty(&family_tree).expect("program no crashy");
  write(TARGET_FILE_PATH, json).expect("program no crashy");
}
