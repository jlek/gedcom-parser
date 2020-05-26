use super::error::{Error, Result};
use crate::parse_gedcom_line::{parse_gedcom_line, parse_level, GedcomLine};
use serde::{
  de::{self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor},
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

#[derive(Debug)]
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
      _ => {
        if self.remaining_input.is_empty() {
          return self.deserialize_str(visitor);
        }
        let (_, next_level) = parse_level(self.remaining_input)?;
        if next_level == self.current_line.level + 1 {
          self.deserialize_map(visitor)
        } else {
          self.deserialize_str(visitor)
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

  fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    let value = visitor.visit_seq(GedcomSequenceAccess {
      de: &mut self,
      first: true,
    })?;

    if self.remaining_input.is_empty() {
      Ok(value)
    } else {
      Err(Error::ExpectedMapEnd)
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

  fn deserialize_enum<V>(
    self,
    _name: &'static str,
    _variants: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    visitor.visit_enum(GedcomEnumAccess { de: self })
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char string
      bytes byte_buf option unit unit_struct newtype_struct tuple
      tuple_struct struct identifier ignored_any
  }
}

struct GedcomSequenceAccess<'a, 'de: 'a> {
  de: &'a mut Deserializer<'de>,
  first: bool,
}

impl<'de, 'a> SeqAccess<'de> for GedcomSequenceAccess<'a, 'de> {
  type Error = Error;

  fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
  where
    T: DeserializeSeed<'de>,
  {
    if self.first {
      self.first = false;
      return seed.deserialize(&mut *self.de).map(Some);
    }
    if self.de.remaining_input.is_empty() {
      return Ok(None);
    }

    self.de.parse_next_line()?;
    self.de.state = DeserializerState::DeserialisingValue;
    seed.deserialize(&mut *self.de).map(Some)
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

struct GedcomEnumAccess<'a, 'de: 'a> {
  de: &'a mut Deserializer<'de>,
}

impl<'de, 'a> EnumAccess<'de> for GedcomEnumAccess<'a, 'de> {
  type Error = Error;
  type Variant = Self;

  fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
  where
    V: DeserializeSeed<'de>,
  {
    self.de.state = DeserializerState::DeserialisingKey;
    let variant = seed.deserialize(&mut *self.de)?;
    self.de.state = DeserializerState::DeserialisingLine;
    Ok((variant, self))
  }
}

impl<'de, 'a> VariantAccess<'de> for GedcomEnumAccess<'a, 'de> {
  type Error = Error;

  fn unit_variant(self) -> Result<()> {
    Ok(())
  }

  fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
  where
    T: DeserializeSeed<'de>,
  {
    seed.deserialize(self.de)
  }

  fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }

  fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    unimplemented!()
  }
}

#[test]
fn test_simple_struct() {
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

#[test]
fn test_struct_with_irrelevant_field() {
  use serde::Deserialize;

  #[derive(Deserialize, PartialEq, Debug)]
  struct Foo<'a> {
    #[serde(rename(deserialize = "BAR"))]
    bar: &'a str,
  }

  let input = "0 FOO\n1 BAR bar\n1 BAZ baz\n";
  let result: Foo = from_str(input).expect("No errors during this test");
  assert_eq!(result, Foo { bar: "bar" });
}

#[test]
fn test_struct_with_multiple_fields() {
  use serde::Deserialize;

  #[derive(Deserialize, PartialEq, Debug)]
  struct Foo<'a> {
    #[serde(rename(deserialize = "BAR"))]
    bar: &'a str,
    #[serde(rename(deserialize = "BAZ"))]
    baz: &'a str,
    #[serde(rename(deserialize = "QUX"))]
    qux: &'a str,
  }

  let input = "0 FOO\n1 BAR bar\n1 BAZ baz\n1 QUX qux\n";
  let result: Foo = from_str(input).expect("No errors during this test");
  assert_eq!(
    result,
    Foo {
      bar: "bar",
      baz: "baz",
      qux: "qux"
    }
  );
}

#[test]
fn test_nested_struct() {
  use serde::Deserialize;

  #[derive(Deserialize, PartialEq, Debug)]
  struct Foo<'a> {
    #[serde(borrow, rename(deserialize = "BAR"))]
    bar: Bar<'a>,
  }

  #[derive(Deserialize, PartialEq, Debug)]
  struct Bar<'a> {
    #[serde(rename(deserialize = "BAZ"))]
    baz: &'a str,
    #[serde(rename(deserialize = "QUX"))]
    qux: &'a str,
  }

  let input = "0 FOO\n1 BAR\n2 BAZ baz\n2 QUX qux\n";
  let result: Foo = from_str(input).expect("No errors during this test");
  assert_eq!(
    result,
    Foo {
      bar: Bar {
        baz: "baz",
        qux: "qux"
      }
    }
  );
}

#[test]
fn test_struct_with_array_field() {
  use serde::Deserialize;

  #[derive(Deserialize, PartialEq, Debug)]
  struct Foo<'a> {
    #[serde(borrow, rename(deserialize = "BAR"))]
    bar: Vec<&'a str>,
  }

  let input = "0 FOO\n1 BAR bar1\n1 BAR bar2\n1 BAR bar3\n";
  let result: Foo = from_str(input).expect("No errors during this test");
  assert_eq!(
    result,
    Foo {
      bar: vec!["bar1", "bar2", "bar3"]
    }
  );
}

#[test]
fn test_struct_with_array_field_but_only_one_value() {
  use serde::Deserialize;

  #[derive(Deserialize, PartialEq, Debug)]
  struct Foo<'a> {
    #[serde(borrow, rename(deserialize = "BAR"))]
    bar: Vec<&'a str>,
  }

  let input = "0 FOO\n1 BAR bar1\n";
  let result: Foo = from_str(input).expect("No errors during this test");
  assert_eq!(result, Foo { bar: vec!["bar1"] });
}

#[test]
fn test_enum() {
  use serde::Deserialize;

  #[derive(Deserialize, PartialEq, Debug)]
  struct Foo<'a> {
    #[serde(rename(deserialize = "BAR"))]
    bar: &'a str,
  }

  #[derive(Deserialize, PartialEq, Debug)]
  struct Baz<'a> {
    #[serde(rename(deserialize = "QUX"))]
    qux: &'a str,
  }

  #[derive(Deserialize, PartialEq, Debug)]
  enum FooBaz<'a> {
    #[serde(borrow, rename = "FOO")]
    Foo(Foo<'a>),
    #[serde(borrow, rename = "BAR")]
    Baz(Baz<'a>),
  }

  let input = "0 FOO\n1 BAR bar\n";
  let result: FooBaz = from_str(input).expect("No errors during this test");
  assert_eq!(result, FooBaz::Foo(Foo { bar: "bar" }));
}

#[test]
fn test_enum_unit() {
  use serde::Deserialize;

  #[derive(Deserialize, PartialEq, Debug)]
  enum Foo {
    #[serde(rename = "BAR")]
    Bar,
  }

  let input = "0 BAR\n";
  let result: Foo = from_str(input).expect("No errors during this test");
  assert_eq!(result, Foo::Bar);
}
