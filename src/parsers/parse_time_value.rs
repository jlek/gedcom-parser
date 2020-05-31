use super::utilities::{from_decimal, is_decimal_digit};
use crate::gedcom_entities::TimeValue;
use nom::{
  bytes::complete::{tag, take_while_m_n},
  combinator::{map_res, opt},
  sequence::{preceded, tuple},
  IResult,
};

// TODO Add time validation
// TODO Add decimal fraction of a second
pub fn parse_time_value(input: &str) -> IResult<&str, TimeValue> {
  let (remaining_input, (hours, _, minutes, seconds)) = tuple((
    parse_hours,
    parse_separator,
    parse_minutes,
    opt(preceded(parse_separator, parse_seconds)),
  ))(input)?;

  Ok((
    remaining_input,
    TimeValue {
      hours,
      minutes,
      seconds,
    },
  ))
}

fn parse_hours(input: &str) -> IResult<&str, u8> {
  map_res(take_while_m_n(1, 2, is_decimal_digit), from_decimal)(input)
}

fn parse_separator(input: &str) -> IResult<&str, &str> {
  tag(":")(input)
}

fn parse_minutes(input: &str) -> IResult<&str, u8> {
  map_res(take_while_m_n(1, 2, is_decimal_digit), from_decimal)(input)
}

fn parse_seconds(input: &str) -> IResult<&str, u8> {
  map_res(take_while_m_n(1, 2, is_decimal_digit), from_decimal)(input)
}
