// Not sure what this does, but apparently it's now needed to compile:
#![type_length_limit = "8388608"]

extern crate nom;
mod gedcom_data_format;
mod gedcom_entities;
mod parse_gedcom_header;
mod parse_gedcom_line;

fn main() {}
