use super::parse_address_structure::parse_address_structure;
use crate::gedcom_entities::Business;
use crate::parse_gedcom_line::parse_gedcom_line_value_only;
use nom::{
  combinator::{map, opt},
  sequence::tuple,
  IResult,
};

pub fn parse_business<'input>(level: u8) -> impl Fn(&'input str) -> IResult<&'input str, Business> {
  map(
    tuple((
      parse_gedcom_line_value_only(level, "CORP", 1, 90),
      opt(parse_address_structure(level + 1)),
    )),
    |(name, address_structure)| Business {
      name,
      address_structure,
    },
  )
}

#[test]
fn parse_business_valid() {
  // Arrange
  let input = "2 CORP DC Thomson Family History\n3 ADDR The Glebe, 6 Chapel Place, Rivington Street\n4 CITY London\n4 POST EC2A 3DQ\n4 CTRY England\n3 WWW www.findmypast.com\n";
  let level = 2;

  // Act
  let (remaining_input, business) =
    parse_business(level)(input).expect("In this test it should not return an error.");

  // Assert
  assert_eq!(remaining_input, "");
  assert_eq!(
    business,
    Business {
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
    }
  )
}
