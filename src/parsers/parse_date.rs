use super::utilities::{from_decimal, is_decimal_digit};
use crate::gedcom_entities::{DateExact, DateGreg, Month, Month::*};
use nom::{
  branch::alt,
  bytes::complete::{tag, tag_no_case, take_while_m_n},
  combinator::{map, map_res},
  sequence::tuple,
  IResult,
};

// TODO Add date validation
pub fn parse_date_exact(input: &str) -> IResult<&str, DateExact> {
  let (remaining_input, (day, _, month, _, year)) =
    tuple((parse_day, parse_space, parse_month, parse_space, parse_year))(input)?;

  Ok((remaining_input, DateExact { day, month, year }))
}

// TODO Add date validation
pub fn parse_date_greg(input: &str) -> IResult<&str, DateGreg> {
  let (remaining_input, (day, _, month, _, year)) =
    tuple((parse_day, parse_space, parse_month, parse_space, parse_year))(input)?;

  Ok((remaining_input, DateGreg { day, month, year }))
}

fn parse_day(input: &str) -> IResult<&str, u8> {
  map_res(take_while_m_n(1, 2, is_decimal_digit), from_decimal)(input)
}

fn parse_space(input: &str) -> IResult<&str, &str> {
  tag(" ")(input)
}

fn parse_month(input: &str) -> IResult<&str, Month> {
  let (remaining_input, month) = alt((
    map(tag_no_case("JAN"), |_| January),
    map(tag_no_case("FEB"), |_| February),
    map(tag_no_case("MAR"), |_| March),
    map(tag_no_case("APR"), |_| April),
    map(tag_no_case("MAY"), |_| May),
    map(tag_no_case("JUN"), |_| June),
    map(tag_no_case("JUL"), |_| July),
    map(tag_no_case("AUG"), |_| August),
    map(tag_no_case("SEP"), |_| September),
    map(tag_no_case("OCT"), |_| October),
    map(tag_no_case("NOV"), |_| November),
    map(tag_no_case("DEC"), |_| December),
  ))(input)?;

  Ok((remaining_input, month))
}

// TODO Add support for /YY year modifier
fn parse_year(input: &str) -> IResult<&str, i16> {
  map_res(take_while_m_n(3, 4, is_decimal_digit), |year| {
    i16::from_str_radix(year, 10)
  })(input)
}
