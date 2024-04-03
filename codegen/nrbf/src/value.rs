//! Representation of an NRBF value.

use std::collections::HashMap;
#[cfg(feature = "serde")]
use std::{collections::BTreeMap, fmt, iter};

#[cfg(feature = "serde")]
use serde::{
  de::{self, value::Error, Expected, IntoDeserializer, Visitor},
  forward_to_deserialize_any, Deserializer,
};

use crate::data_type::{self};

#[cfg(feature = "serde")]
fn resolve_object<'de, 'o, V: Visitor<'de>>(
  objects: &'o BTreeMap<i32, Value<'de>>,
  id: &i32,
  visitor: &V,
) -> Result<&'o Value<'de>, Error> {
  use serde::de::{Error, Unexpected};

  if let Some(object) = objects.get(id) {
    Ok(object)
  } else {
    Err(Error::invalid_value(Unexpected::Other("unresolved object ID"), visitor))
  }
}

/// A decimal number.
#[derive(Debug, Clone, PartialEq)]
pub struct Decimal(pub(crate) data_type::Decimal);

/// A time span.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeSpan(pub(crate) data_type::TimeSpan);

impl TimeSpan {
  /// Duration as the number of 100 nanoseconds. The values range from -10675199 days, 2 hours, 48 minutes, and 05.4775808
  /// seconds to 10675199 days, 2 hours, 48 minutes, and 05.4775807 seconds inclusive.
  pub fn value(&self) -> i64 {
    self.0.into()
  }
}

/// Time-zone information for [`DateTime`].
pub enum DateTimeKind {
  /// The time specified is in the Coordinated Universal Time (UTC) time zone.
  Utc,
  /// The time specified is in the local time zone.
  Local,
}

/// An date-time value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DateTime(pub(crate) data_type::DateTime);

impl DateTime {
  /// Number of 100 nanoseconds that have elapsed since 12:00:00, January 1, 0001.
  /// The value can represent time instants in a granularity of 100 nanoseconds
  /// until 23:59:59.9999999, December 31, 9999.
  pub fn ticks(&self) -> i64 {
    i64::from(self.0) >> 2
  }

  /// Provides the time-zone information.
  pub fn kind(&self) -> Option<DateTimeKind> {
    match i64::from(self.0) & 0b11 {
      1 => Some(DateTimeKind::Utc),
      2 => Some(DateTimeKind::Utc),
      _ => None,
    }
  }
}

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

/// An NRBF value.
#[derive(Debug, Clone, PartialEq)]
pub enum Value<'i> {
  /// An object.
  Object(Object<'i>),
  /// An array.
  Array(Vec<Value<'i>>),
  /// An boolean value.
  Boolean(bool),
  /// A byte.
  Byte(u8),
  /// A character.
  Char(char),
  /// A decimal number.
  Decimal(Decimal),
  /// A double precision floating point number.
  Double(f64),
  /// A 16-bit signed integer.
  Int16(i16),
  /// A 64-bit signed integer.
  Int32(i32),
  /// A 64-bit signed integer.
  Int64(i64),
  /// A signed byte.
  SByte(i8),
  /// A single precision floating point number.
  Single(f32),
  /// A time span.
  TimeSpan(TimeSpan),
  /// A date-time.
  DateTime(DateTime),
  /// A 16-bit unsigned integer.
  UInt16(u16),
  /// A 32-bit unsigned integer.
  UInt32(u32),
  /// A 64-bit unsigned integer.
  UInt64(u64),
  /// A string.
  String(&'i str),
  /// A null value.
  Null(usize),
  /// A value reference.
  Ref(i32),
}

#[cfg(feature = "serde")]
#[derive(Debug)]
pub(crate) struct ObjectDeserializer<'de, 'o> {
  objects: &'o BTreeMap<i32, Value<'de>>,
  object: &'o Object<'de>,
}

#[cfg(feature = "serde")]
impl<'de, 'o> ObjectDeserializer<'de, 'o> {
  pub fn new(objects: &'o BTreeMap<i32, Value<'de>>, object: &'o Object<'de>) -> Self {
    Self { objects, object }
  }
}

#[cfg(feature = "serde")]
impl<'de, 'o> de::Deserializer<'de> for ObjectDeserializer<'de, 'o> {
  type Error = Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: de::Visitor<'de>,
  {
    use serde::de::{Error, Unexpected};

    let Object { class, library, members } = self.object;

    if library.is_some() {
      return Err(Error::invalid_type(Unexpected::Other(class), &visitor))
    }

    let class_name = class.split_once('`').map(|(s, _)| s).unwrap_or(*class);

    match class_name {
      "System.Boolean" => {
        if members.len() == 1 {
          if let Some(Value::Boolean(n)) = members.get("m_value") {
            return visitor.visit_bool((*n).into())
          }
        }
      },
      "System.Byte" => {
        if members.len() == 1 {
          if let Some(Value::Byte(n)) = members.get("m_value") {
            return visitor.visit_u8((*n).into())
          }
        }
      },
      "System.SByte" => {
        if members.len() == 1 {
          if let Some(Value::SByte(n)) = members.get("m_value") {
            return visitor.visit_i8((*n).into())
          }
        }
      },
      "System.Char" => {
        if members.len() == 1 {
          if let Some(Value::Char(c)) = members.get("m_value") {
            return visitor.visit_char((*c).into())
          }
        }
      },
      "System.Decimal" => {
        if members.len() == 1 {
          if let Some(Value::Decimal(_c)) = members.get("m_value") {
            unimplemented!()
          }
        }
      },
      "System.Double" => {
        if members.len() == 1 {
          if let Some(Value::Double(n)) = members.get("m_value") {
            return visitor.visit_f64((*n).into())
          }
        }
      },
      "System.Single" => {
        if members.len() == 1 {
          if let Some(Value::Single(n)) = members.get("m_value") {
            return visitor.visit_f32((*n).into())
          }
        }
      },
      "System.Int32" => {
        if members.len() == 1 {
          if let Some(Value::Int32(n)) = members.get("m_value") {
            return visitor.visit_i32((*n).into())
          }
        }
      },
      "System.UInt32" => {
        if members.len() == 1 {
          if let Some(Value::UInt32(n)) = members.get("m_value") {
            return visitor.visit_u32((*n).into())
          }
        }
      },
      "System.Int64" => {
        if members.len() == 1 {
          if let Some(Value::Int64(n)) = members.get("m_value") {
            return visitor.visit_i64((*n).into())
          }
        }
      },
      "System.UInt64" => {
        if members.len() == 1 {
          if let Some(Value::UInt64(n)) = members.get("m_value") {
            return visitor.visit_u64((*n).into())
          }
        }
      },
      "System.Int16" => {
        if members.len() == 1 {
          if let Some(Value::Int16(n)) = members.get("m_value") {
            return visitor.visit_i16((*n).into())
          }
        }
      },
      "System.UInt16" => {
        if members.len() == 1 {
          if let Some(Value::UInt16(n)) = members.get("m_value") {
            return visitor.visit_u16((*n).into())
          }
        }
      },
      "System.Collections.Generic.List" => {
        if members.len() == 3 {
          if let (Some(mut items), Some(Value::Int32(size)), Some(Value::Int32(_version))) =
            (members.get("_items"), members.get("_size"), members.get("_version"))
          {
            if let Value::Ref(id) = items {
              items = resolve_object(self.objects, id, &visitor)?;
            }

            if let Value::Array(items) = items {
              return ArrayDeserializer::new(self.objects, items.into_iter().take(i32::from(*size) as usize))
                .deserialize_any(visitor)
            }
          }
        }
      },
      _ => return Err(Error::invalid_type(Unexpected::Other(class_name), &visitor)),
    }

    Err(Error::custom(format!("invalid system type: {}", class_name)))
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

    MapDeserializer::new(members.into_iter().map(|(key, value)| (*key, ValueDeserializer::new(self.objects, value))))
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
struct ExpectedInArray(usize);

#[cfg(feature = "serde")]
impl Expected for ExpectedInArray {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    if self.0 == 1 {
      formatter.write_str("1 element in array")
    } else {
      write!(formatter, "{} elements in array", self.0)
    }
  }
}

#[cfg(feature = "serde")]
#[derive(Debug)]
pub(crate) struct ArrayDeserializer<'de, 'o, I> {
  objects: &'o BTreeMap<i32, Value<'de>>,
  iter: iter::Fuse<I>,
  null_count: usize,
  count: usize,
}

#[cfg(feature = "serde")]
impl<'de, 'o, I> ArrayDeserializer<'de, 'o, I>
where
  I: Iterator,
{
  pub fn new(objects: &'o BTreeMap<i32, Value<'de>>, iter: I) -> Self {
    Self { objects, iter: iter.fuse(), null_count: 0, count: 0 }
  }
}

#[cfg(feature = "serde")]
impl<'de, 'o, I> ArrayDeserializer<'de, 'o, I>
where
  I: Iterator<Item = &'o Value<'de>>,
{
  /// Check for remaining elements after passing an `ArrayDeserializer` to
  /// `Visitor::visit_seq`.
  pub fn end<E: de::Error>(self) -> Result<(), E> {
    let remaining =
      self.iter.map(|value| if let Value::Null(n) = value { *n } else { 1 }).sum::<usize>() + self.null_count;
    if remaining == 0 {
      Ok(())
    } else {
      // First argument is the number of elements in the data, second
      // argument is the number of elements expected by the Deserialize.
      Err(de::Error::invalid_length(self.count + remaining, &ExpectedInArray(self.count)))
    }
  }
}

#[cfg(feature = "serde")]
impl<'de, 'o, I> de::Deserializer<'de> for ArrayDeserializer<'de, 'o, I>
where
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
impl<'de, 'o, I> de::SeqAccess<'de> for ArrayDeserializer<'de, 'o, I>
where
  I: Iterator<Item = &'o Value<'de>>,
{
  type Error = Error;

  fn next_element_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>, Self::Error>
  where
    V: de::DeserializeSeed<'de>,
  {
    if self.null_count > 0 {
      self.count += 1;
      self.null_count -= 1;
      return seed.deserialize(ValueDeserializer::new(self.objects, &Value::Null(1))).map(Some)
    }

    match self.iter.next() {
      Some(Value::Null(null_count @ 2..)) => {
        self.count += 1;
        self.null_count = null_count - 1;
        seed.deserialize(ValueDeserializer::new(self.objects, &Value::Null(1))).map(Some)
      },
      Some(object) => {
        self.count += 1;
        seed.deserialize(ValueDeserializer::new(self.objects, object)).map(Some)
      },
      None => Ok(None),
    }
  }
}

#[cfg(feature = "serde")]
#[derive(Debug)]
pub(crate) struct ValueDeserializer<'de, 'o> {
  objects: &'o BTreeMap<i32, Value<'de>>,
  object: &'o Value<'de>,
}

#[cfg(feature = "serde")]
impl<'de, 'o> ValueDeserializer<'de, 'o> {
  pub fn new(objects: &'o BTreeMap<i32, Value<'de>>, object: &'o Value<'de>) -> Self {
    Self { objects, object }
  }
}

#[cfg(feature = "serde")]
impl<'de> IntoDeserializer<'de, Error> for ValueDeserializer<'de, '_> {
  type Deserializer = Self;

  fn into_deserializer(self) -> Self::Deserializer {
    self
  }
}

#[cfg(feature = "serde")]
impl<'de> Deserializer<'de> for ValueDeserializer<'de, '_> {
  type Error = Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    use serde::de::{Error, Unexpected};

    match self.object {
      Value::Object(object) => ObjectDeserializer::new(self.objects, object).deserialize_any(visitor),
      Value::Array(members) => ArrayDeserializer::new(self.objects, members.into_iter()).deserialize_any(visitor),
      Value::Ref(id) => Self::new(self.objects, resolve_object(self.objects, id, &visitor)?).deserialize_any(visitor),
      Value::Boolean(v) => visitor.visit_bool(*v),
      Value::SByte(v) => visitor.visit_i8(*v),
      Value::Int16(v) => visitor.visit_i16(*v),
      Value::Int32(v) => visitor.visit_i32(*v),
      Value::Int64(v) => visitor.visit_i64(*v),
      Value::Byte(v) => visitor.visit_u8(*v),
      Value::UInt16(v) => visitor.visit_u16(*v),
      Value::UInt32(v) => visitor.visit_u32(*v),
      Value::UInt64(v) => visitor.visit_u64(*v),
      Value::Single(v) => visitor.visit_f32(*v),
      Value::Double(v) => visitor.visit_f64(*v),
      Value::Char(v) => visitor.visit_char(*v),
      Value::Decimal(v) => visitor.visit_string((v.0).0.to_string()),
      Value::TimeSpan(v) => visitor.visit_i64((*v).0.into()),
      Value::DateTime(v) => visitor.visit_i64((*v).0.into()),
      Value::String(s) => visitor.visit_borrowed_str(s),
      Value::Null(1) => visitor.visit_none(),
      Value::Null(_) => Err(Error::invalid_value(Unexpected::Other("unresolved null object"), &visitor)),
    }
  }

  fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    if matches!(self.object, Value::Null(1)) {
      visitor.visit_none()
    } else {
      visitor.visit_some(self)
    }
  }

  fn deserialize_struct<V>(
    self,
    name: &'static str,
    fields: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    match self.object {
      Value::Object(object) => ObjectDeserializer::new(self.objects, object).deserialize_struct(name, fields, visitor),
      Value::Ref(id) => {
        Self::new(self.objects, resolve_object(self.objects, id, &visitor)?).deserialize_struct(name, fields, visitor)
      },
      _ => self.deserialize_any(visitor),
    }
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf unit unit_struct newtype_struct seq tuple
      tuple_struct map enum identifier ignored_any
  }
}
