use crate::gedcom_entities::Address;
use crate::parse_gedcom_line::parse_gedcom_line_value_only;
use nom::{
  combinator::{map, opt},
  sequence::tuple,
  IResult,
};

pub fn parse_address<'input>(level: u8) -> impl Fn(&'input str) -> IResult<&'input str, Address> {
  map(
    tuple((
      parse_gedcom_line_value_only(level, "ADDR", 1, 60),
      opt(parse_gedcom_line_value_only(level + 1, "CITY", 1, 60)),
      opt(parse_gedcom_line_value_only(level + 1, "POST", 1, 10)),
      opt(parse_gedcom_line_value_only(level + 1, "CTRY", 1, 60)),
    )),
    |(address_line, city, post_code, country)| Address {
      address_line,
      city,
      post_code,
      country,
    },
  )
}

#[test]
fn parse_address_valid() {
  // Arrange
  let input = "3 ADDR The Glebe, 6 Chapel Place, Rivington Street\n4 CITY London\n4 POST EC2A 3DQ\n4 CTRY England\n";
  let level = 3;

  // Act
  let (remaining_input, address) =
    parse_address(level)(input).expect("In this test it should not return an error.");

  // Assert
  assert_eq!(remaining_input, "");
  assert_eq!(
    address,
    Address {
      address_line: "The Glebe, 6 Chapel Place, Rivington Street".to_string(),
      city: Some("London".to_string()),
      post_code: Some("EC2A 3DQ".to_string()),
      country: Some("England".to_string()),
    }
  )
}

#[test]
#[ignore]
fn parse_address_changed_sequence() {
  // Arrange
  let input = "3 ADDR The Glebe, 6 Chapel Place, Rivington Street\n4 CTRY England\n4 CITY London\n4 POST EC2A 3DQ\n";
  let level = 3;

  // Act
  let (remaining_input, address) =
    parse_address(level)(input).expect("In this test it should not return an error.");

  // Assert
  // assert_eq!(remaining_input, "");
  assert_eq!(
    address,
    Address {
      address_line: "The Glebe, 6 Chapel Place, Rivington Street".to_string(),
      city: Some("London".to_string()),
      post_code: Some("EC2A 3DQ".to_string()),
      country: Some("England".to_string()),
    }
  )
}
