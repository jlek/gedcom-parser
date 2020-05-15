use super::error::{Error, Result};
use crate::parse_gedcom_line::{parse_gedcom_line, parse_level};
use serde::{
  de::{self, DeserializeSeed, MapAccess, Visitor},
  forward_to_deserialize_any, Deserialize,
};

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
  T: Deserialize<'a>,
{
  let mut deserializer = Deserializer::from_str(s);
  let t = T::deserialize(&mut deserializer)?;
  if deserializer.input.is_empty() {
    Ok(t)
  } else {
    Err(Error::TrailingCharacters)
  }
}

pub struct Deserializer<'de> {
  input: &'de str,
  parse_tag: bool,
  parse_value: bool,
}

impl<'de> Deserializer<'de> {
  pub fn from_str(input: &'de str) -> Self {
    Deserializer {
      input,
      parse_tag: false,
      parse_value: false,
    }
  }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
  type Error = Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    let (remaining_input, line) = parse_gedcom_line(self.input).unwrap();

    if self.parse_tag || self.parse_value {
      return self.deserialize_str(visitor);
    }

    let (_, next_level) = parse_level(remaining_input).unwrap();
    if (next_level > line.level) {
      self.deserialize_map(visitor)
    } else {
      Err(Error::Message(
        "Need to parse value, but that is not yet implemented".to_owned(),
      ))
    }
  }

  fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    let (remaining_input, line) = parse_gedcom_line(self.input).unwrap();
    if (self.parse_tag) {
      return visitor.visit_borrowed_str(line.tag);
    } else {
      self.input = remaining_input;
      return visitor.visit_borrowed_str(line.value.unwrap());
    }
  }

  fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    let (remaining_input, line) = parse_gedcom_line(self.input).unwrap();
    let (_, next_level) = parse_level(remaining_input).unwrap();
    if !(next_level > line.level) {
      return Err(Error::Message("Expected a map".to_owned()));
    }
    self.input = remaining_input;

    let value = visitor.visit_map(GedcomMapAccess {
      de: &mut self,
      level: line.level,
    })?;

    if (self.input.is_empty()) {
      return Ok(value);
    } else {
      return Err(Error::Message("Expected map end".to_owned()));
    }
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct struct enum identifier ignored_any
  }
}

struct GedcomMapAccess<'a, 'de: 'a> {
  de: &'a mut Deserializer<'de>,
  level: u8,
}

impl<'de, 'a> MapAccess<'de> for GedcomMapAccess<'a, 'de> {
  type Error = Error;

  fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
  where
    K: DeserializeSeed<'de>,
  {
    if self.de.input.is_empty() {
      return Ok(None);
    }
    self.de.parse_tag = true;
    let result = seed.deserialize(&mut *self.de).map(Some);
    self.de.parse_tag = false;
    result
  }

  fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
  where
    V: DeserializeSeed<'de>,
  {
    self.de.parse_value = true;
    let result = seed.deserialize(&mut *self.de);
    self.de.parse_value = false;
    result
  }
}

#[test]
fn test_struct() {
  use serde::Deserialize;

  #[derive(Deserialize, PartialEq, Debug)]
  struct Foo<'a> {
    #[serde(rename(deserialize = "BAR"))]
    bar: &'a str,
  }

  let input = "0 FOO\n1 BAR bar\n";
  let result: Foo = from_str(input).expect("No errors during this test");
  assert_eq!(result, Foo { bar: "bar" });
}
