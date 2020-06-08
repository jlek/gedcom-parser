// Not sure what this does, but apparently it's now needed to compile:
#![type_length_limit = "8388608"]

extern crate nom;
mod gedcom_data_format;
mod gedcom_entities;
mod parsers;
mod relation_entities;
mod tests;
mod transform_gedcom_to_relation;

use clap::{App, Arg};
use gedcom_data_format::from_str;
use gedcom_entities::Record;
use serde_json::to_string_pretty;
use std::fs::{read_to_string, write};
use transform_gedcom_to_relation::transform_gedcom_to_relation;

fn main() {
  let app = App::new("Gedcom Parser")
    .arg(Arg::with_name("source").required(true))
    .arg(Arg::with_name("target").required(true));
  let matches = app.get_matches();
  let source_file_path = matches.value_of("source").unwrap();
  let target_file_path = matches.value_of("target").unwrap();

  let file = read_to_string(source_file_path).expect("File should exist");
  let records: Vec<Record> = from_str(&file).expect("program no crashy");
  let family_tree = transform_gedcom_to_relation(&records);
  let json = to_string_pretty(&family_tree).expect("program no crashy");
  write(target_file_path, json).expect("program no crashy");
}
