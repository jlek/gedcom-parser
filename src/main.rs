// Not sure what this does, but apparently it's now needed to compile:
#![type_length_limit = "8388608"]

extern crate nom;
mod gedcom_data_format;
mod gedcom_entities;
mod parsers;
mod tests;

use gedcom_data_format::from_str;
use gedcom_entities::Record;
use std::fs::read_to_string;

fn main() {
  let file = read_to_string("src/tests/one-node.ged").expect("File should exist");
  let records: Vec<Record> = from_str(&file).expect("testy no crashy");
  println!("{:?}", records);
}
