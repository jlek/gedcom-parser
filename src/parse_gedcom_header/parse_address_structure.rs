use super::parse_address::parse_address;
use crate::gedcom_entities::AddressStructure;
use crate::parse_gedcom_line::parse_gedcom_line_value_only;
use nom::{
  combinator::{map, opt},
  sequence::tuple,
  IResult,
};

pub fn parse_address_structure<'input>(
  level: u8,
) -> impl Fn(&'input str) -> IResult<&'input str, AddressStructure> {
  map(
    tuple((
      parse_address(level),
      opt(parse_gedcom_line_value_only(level, "WWW", 5, 120)),
    )),
    |(address, web_page)| AddressStructure { address, web_page },
  )
}

#[test]
fn parse_address_structure_valid() {
  // Arrange
  let input = "3 ADDR The Glebe, 6 Chapel Place, Rivington Street\n4 CITY London\n4 POST EC2A 3DQ\n4 CTRY England\n3 WWW www.findmypast.com\n";
  let level = 3;

  // Act
  let (remaining_input, address_structure) =
    parse_address_structure(level)(input).expect("In this test it should not return an error.");

  // Assert
  assert_eq!(remaining_input, "");
  assert_eq!(
    address_structure,
    AddressStructure {
      address: crate::gedcom_entities::Address {
        address_line: "The Glebe, 6 Chapel Place, Rivington Street".to_string(),
        city: Some("London".to_string()),
        post_code: Some("EC2A 3DQ".to_string()),
        country: Some("England".to_string()),
      },
      web_page: Some("www.findmypast.com".to_string())
    }
  )
}
