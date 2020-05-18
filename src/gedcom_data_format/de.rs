use super::error::{Error, Result};
use crate::parse_gedcom_line::{parse_gedcom_line, parse_level, GedcomLine};
use serde::{
  de::{self, DeserializeSeed, MapAccess, Visitor},
  forward_to_deserialize_any, Deserialize,
};

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
  T: Deserialize<'a>,
{
  let mut deserializer = Deserializer::from_str(s)?;
  let t = T::deserialize(&mut deserializer)?;
  if deserializer.remaining_input.is_empty() {
    Ok(t)
  } else {
    Err(Error::TrailingCharacters)
  }
}

enum DeserializerState {
  DeserialisingLine,
  DeserialisingKey,
  DeserialisingValue,
}

pub struct Deserializer<'de> {
  remaining_input: &'de str,
  current_line: GedcomLine<'de, 'de>,
  state: DeserializerState,
}

impl<'de> Deserializer<'de> {
  pub fn from_str(input: &'de str) -> Result<Self> {
    let (remaining_input, current_line) = parse_gedcom_line(input)?;
    Ok(Deserializer {
      remaining_input,
      current_line,
      state: DeserializerState::DeserialisingLine,
    })
  }

  fn parse_next_line(&mut self) -> Result<()> {
    let (remaining_input, next_line) = parse_gedcom_line(self.remaining_input)?;
    self.current_line = next_line;
    self.remaining_input = remaining_input;
    Ok(())
  }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
  type Error = Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    match self.state {
      DeserializerState::DeserialisingKey => self.deserialize_str(visitor),
      DeserializerState::DeserialisingValue => self.deserialize_str(visitor),
      DeserializerState::DeserialisingLine => {
        let (_, next_level) = parse_level(self.remaining_input)?;
        if next_level == self.current_line.level + 1 {
          self.deserialize_map(visitor)
        } else {
          unimplemented!()
        }
      }
    }
  }

  fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    match self.state {
      DeserializerState::DeserialisingKey => visitor.visit_borrowed_str(self.current_line.tag),
      DeserializerState::DeserialisingValue => {
        let value = self
          .current_line
          .value
          .ok_or(Error::ExpectedGedcomLineWithValue)?;
        visitor.visit_borrowed_str(value)
      }
      _ => panic!("Aaaah!"),
    }
  }

  fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    let (_, next_level) = parse_level(self.remaining_input)?;
    if next_level != self.current_line.level + 1 {
      return Err(Error::ExpectedMap);
    }

    let value = visitor.visit_map(GedcomMapAccess { de: &mut self })?;

    if self.remaining_input.is_empty() {
      Ok(value)
    } else {
      Err(Error::ExpectedMapEnd)
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
}

impl<'de, 'a> MapAccess<'de> for GedcomMapAccess<'a, 'de> {
  type Error = Error;

  fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
  where
    K: DeserializeSeed<'de>,
  {
    if self.de.remaining_input.is_empty() {
      return Ok(None);
    }
    self.de.parse_next_line()?;
    self.de.state = DeserializerState::DeserialisingKey;
    seed.deserialize(&mut *self.de).map(Some)
  }

  fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
  where
    V: DeserializeSeed<'de>,
  {
    self.de.state = DeserializerState::DeserialisingValue;
    seed.deserialize(&mut *self.de)
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
