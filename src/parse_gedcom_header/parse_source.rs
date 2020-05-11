use super::parse_business::parse_business;
use crate::gedcom_entities::Source;
use crate::parse_gedcom_line::parse_gedcom_line_value_only;
use nom::{
  combinator::{map, opt},
  sequence::tuple,
  IResult,
};

pub fn parse_source<'input>(level: u8) -> impl Fn(&'input str) -> IResult<&'input str, Source> {
  map(
    tuple((
      parse_gedcom_line_value_only(level, "SOUR", 1, 20),
      opt(parse_gedcom_line_value_only(level + 1, "NAME", 1, 90)),
      opt(parse_gedcom_line_value_only(level + 1, "VERS", 1, 15)),
      opt(parse_business(level + 1)),
    )),
    |(id, name, version, business)| Source {
      id,
      name,
      version,
      business,
    },
  )
}

#[test]
fn parse_source_valid() {
  // Arrange
  let input = "1 SOUR FINDMYPAST\n2 NAME Findmypast Family Tree\n2 VERS 2.0\n2 CORP DC Thomson Family History\n3 ADDR The Glebe, 6 Chapel Place, Rivington Street\n4 CITY London\n4 POST EC2A 3DQ\n4 CTRY England\n3 WWW www.findmypast.com\n";
  let level = 1;

  // Act
  let (remaining_input, source) =
    parse_source(level)(input).expect("In this test it should not return an error.");

  // Assert
  assert_eq!(remaining_input, "");
  assert_eq!(
    source,
    Source {
      id: "FINDMYPAST".to_string(),
      name: Some("Findmypast Family Tree".to_string()),
      version: Some("2.0".to_string()),
      business: Some(crate::gedcom_entities::Business {
        name: "DC Thomson Family History".to_string(),
        address_structure: Some(crate::gedcom_entities::AddressStructure {
          address: crate::gedcom_entities::Address {
            address_line: "The Glebe, 6 Chapel Place, Rivington Street".to_string(),
            city: Some("London".to_string()),
            post_code: Some("EC2A 3DQ".to_string()),
            country: Some("England".to_string()),
          },
          web_page: Some("www.findmypast.com".to_string())
        })
      })
    }
  )
}
