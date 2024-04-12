use std::collections::HashMap;
#[cfg(feature = "serde")]
use std::fmt;

#[cfg(feature = "serde")]
use serde::{
  de::Expected,
  de::{self, value::Error, Visitor},
  forward_to_deserialize_any,
};

use super::Value;
#[cfg(feature = "serde")]
use super::{ArrayDeserializer, ValueDeserializer};
#[cfg(feature = "serde")]
use crate::data_type::{Boolean, Byte, Char, Double, Int16, Int32, Int64, Int8, Single, UInt16, UInt32, UInt64};

/// An NRBF object.
#[derive(Debug, Clone, PartialEq)]
pub struct Object<'i> {
  /// The class name.
  pub class: &'i str,
  /// The library name, if present.
  pub library: Option<&'i str>,
  /// The member fields.
  pub members: HashMap<&'i str, Value<'i>>,
}

#[cfg(feature = "serde")]
#[derive(Debug)]
pub(crate) struct ObjectDeserializer<'de, 'o> {
  object: &'o Object<'de>,
}

#[cfg(feature = "serde")]
impl<'de, 'o> ObjectDeserializer<'de, 'o> {
  pub fn new(object: &'o Object<'de>) -> Self {
    Self { object }
  }
}

#[cfg(feature = "serde")]
use serde::de::{value::BorrowedStrDeserializer, IntoDeserializer};

#[cfg(feature = "serde")]
struct StrDeserializer<'i>(&'i str);

#[cfg(feature = "serde")]
impl<'de, E> IntoDeserializer<'de, E> for StrDeserializer<'de>
where
  E: de::Error,
{
  type Deserializer = BorrowedStrDeserializer<'de, E>;

  fn into_deserializer(self) -> BorrowedStrDeserializer<'de, E> {
    BorrowedStrDeserializer::new(self.0)
  }
}

#[cfg(feature = "serde")]
impl<'de, 'o> de::Deserializer<'de> for ObjectDeserializer<'de, 'o> {
  type Error = Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: de::Visitor<'de>,
  {
    use serde::{
      de::{value::MapDeserializer, Error},
      Deserialize,
    };

    let Object { class, library, members } = self.object;

    let map_deserializer: MapDeserializer<
      'de,
      std::iter::Map<std::collections::hash_map::Iter<'_, &'de str, Value<'de>>, _>,
      _,
    > = MapDeserializer::new(members.iter().map(|(key, value)| (StrDeserializer(key), ValueDeserializer::new(value))));

    if library.is_some() {
      return map_deserializer.deserialize_map(visitor)
    }

    let class_name = class.split_once('`').map(|(s, _)| s).unwrap_or(*class);

    match class_name {
      "System.Boolean" => {
        let v = Boolean::deserialize(map_deserializer)?;
        return visitor.visit_bool(v.into())
      },
      "System.Byte" => {
        let v = Byte::deserialize(map_deserializer)?;
        return visitor.visit_u8(v.into())
      },
      "System.SByte" => {
        let v = Int8::deserialize(map_deserializer)?;
        return visitor.visit_i8(v.into())
      },
      "System.Char" => {
        let v = Char::deserialize(map_deserializer)?;
        return visitor.visit_char(v.into())
      },
      "System.Double" => {
        let v = Double::deserialize(map_deserializer)?;
        return visitor.visit_f64(v.into())
      },
      "System.Single" => {
        let v = Single::deserialize(map_deserializer)?;
        return visitor.visit_f32(v.into())
      },
      "System.Int32" => {
        let v = Int32::deserialize(map_deserializer)?;
        return visitor.visit_i32(v.into())
      },
      "System.UInt32" => {
        let v = UInt32::deserialize(map_deserializer)?;
        return visitor.visit_u32(v.into())
      },
      "System.Int64" => {
        let v = Int64::deserialize(map_deserializer)?;
        return visitor.visit_i64(v.into())
      },
      "System.UInt64" => {
        let v = UInt64::deserialize(map_deserializer)?;
        return visitor.visit_u64(v.into())
      },
      "System.Int16" => {
        let v = Int16::deserialize(map_deserializer)?;
        return visitor.visit_i16(v.into())
      },
      "System.UInt16" => {
        let v = UInt16::deserialize(map_deserializer)?;
        return visitor.visit_u16(v.into())
      },
      "System.Collections.Generic.List" => {
        if members.len() == 3 {
          if let (Some(Value::Array(items)), Some(Value::Int32(size)), Some(Value::Int32(_version))) =
            (members.get("_items"), members.get("_size"), members.get("_version"))
          {
            return ListDeserializer::new(items.iter(), (*size) as usize).deserialize_any(visitor)
          }
        }
      },
      _ => return map_deserializer.deserialize_map(visitor),
    }

    Err(Error::custom(format!("invalid system class: {}", class_name)))
  }

  fn deserialize_struct<V>(
    self,
    _name: &'static str,
    _fields: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    use serde::de::value::MapDeserializer;

    let Object { class: _, library: _, members } = self.object;

    MapDeserializer::new(members.iter().map(|(key, value)| (*key, ValueDeserializer::new(value))))
      .deserialize_map(visitor)
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map enum identifier ignored_any
  }
}

#[cfg(feature = "serde")]
#[derive(Debug)]
struct ExpectedInList(usize);

#[cfg(feature = "serde")]
impl Expected for ExpectedInList {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    if self.0 == 1 {
      formatter.write_str("1 element in list")
    } else {
      write!(formatter, "{} elements in list", self.0)
    }
  }
}

#[cfg(feature = "serde")]
#[derive(Debug)]
pub(crate) struct ListDeserializer<I> {
  array_deserializer: ArrayDeserializer<I>,
  count: usize,
  size: usize,
}

#[cfg(feature = "serde")]
impl<I> ListDeserializer<I>
where
  I: Iterator,
{
  pub fn new(iter: I, size: usize) -> Self {
    Self { array_deserializer: ArrayDeserializer::new(iter), count: 0, size }
  }
}

#[cfg(feature = "serde")]
impl<'de, 'o, I> ListDeserializer<I>
where
  'de: 'o,
  I: Iterator<Item = &'o Value<'de>>,
{
  /// Check for remaining elements after passing a `ListDeserializer` to
  /// `Visitor::visit_seq`.
  pub fn end<E: de::Error>(self) -> Result<(), E> {
    if self.count == self.size {
      Ok(())
    } else {
      // First argument is the number of elements in the data, second
      // argument is the number of elements expected by the Deserialize.
      Err(de::Error::invalid_length(self.size, &ExpectedInList(self.count)))
    }
  }
}

#[cfg(feature = "serde")]
impl<'de, 'o, I> de::Deserializer<'de> for ListDeserializer<I>
where
  'de: 'o,
  I: Iterator<Item = &'o Value<'de>>,
{
  type Error = Error;

  fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: de::Visitor<'de>,
  {
    let v = visitor.visit_seq(&mut self)?;
    self.end()?;
    Ok(v)
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map struct enum identifier ignored_any
  }
}

#[cfg(feature = "serde")]
impl<'de, 'o, I> de::SeqAccess<'de> for ListDeserializer<I>
where
  'de: 'o,
  I: Iterator<Item = &'o Value<'de>>,
{
  type Error = Error;

  fn next_element_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>, Self::Error>
  where
    V: de::DeserializeSeed<'de>,
  {
    if self.count < self.size {
      let res = self.array_deserializer.next_element_seed(seed)?;
      self.count += 1;
      return Ok(res)
    }

    Ok(None)
  }
}
