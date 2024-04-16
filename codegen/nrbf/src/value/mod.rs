//! Representation of an NRBF value.

#[cfg(feature = "serde")]
use std::{fmt, iter};

#[cfg(feature = "serde")]
use serde::{
  de::{self, value::Error, Expected, IntoDeserializer, Visitor},
  forward_to_deserialize_any, Deserializer,
};

mod date_time;
pub use date_time::{DateTime, DateTimeKind};
mod decimal;
pub use decimal::Decimal;
mod object;
pub use object::Object;
#[cfg(feature = "serde")]
use object::ObjectDeserializer;
mod time_span;
pub use time_span::TimeSpan;

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
  Null,
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
pub(crate) struct ArrayDeserializer<I> {
  iter: iter::Fuse<I>,
  count: usize,
}

#[cfg(feature = "serde")]
impl<I> ArrayDeserializer<I>
where
  I: Iterator,
{
  pub fn new(iter: I) -> Self {
    Self { iter: iter.fuse(), count: 0 }
  }
}

#[cfg(feature = "serde")]
impl<'de, 'o, I> ArrayDeserializer<I>
where
  'de: 'o,
  I: Iterator<Item = &'o Value<'de>>,
{
  /// Check for remaining elements after passing an `ArrayDeserializer` to
  /// `Visitor::visit_seq`.
  pub fn end<E: de::Error>(self) -> Result<(), E> {
    let remaining = self.iter.count();
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
impl<'de, 'o, I> de::Deserializer<'de> for ArrayDeserializer<I>
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
impl<'de, 'o, I> de::SeqAccess<'de> for ArrayDeserializer<I>
where
  'de: 'o,
  I: Iterator<Item = &'o Value<'de>>,
{
  type Error = Error;

  fn next_element_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>, Self::Error>
  where
    V: de::DeserializeSeed<'de>,
  {
    match self.iter.next() {
      Some(object) => {
        self.count += 1;
        seed.deserialize(ValueDeserializer::new(object)).map(Some)
      },
      None => Ok(None),
    }
  }
}

#[cfg(feature = "serde")]
#[derive(Debug)]
pub(crate) struct ValueDeserializer<'de, 'o> {
  object: &'o Value<'de>,
}

#[cfg(feature = "serde")]
impl<'de, 'o> ValueDeserializer<'de, 'o> {
  pub fn new(object: &'o Value<'de>) -> Self {
    Self { object }
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
    match self.object {
      Value::Object(object) => ObjectDeserializer::new(object).deserialize_any(visitor),
      Value::Array(members) => ArrayDeserializer::new(members.iter()).deserialize_any(visitor),
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
      Value::TimeSpan(v) => visitor.visit_i64(v.0.into()),
      Value::DateTime(v) => visitor.visit_i64(v.0.into()),
      Value::String(s) => visitor.visit_borrowed_str(s),
      Value::Null => visitor.visit_unit(),
    }
  }

  fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    if matches!(self.object, Value::Null) {
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
      Value::Object(object) => ObjectDeserializer::new(object).deserialize_struct(name, fields, visitor),
      _ => self.deserialize_any(visitor),
    }
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf unit unit_struct newtype_struct seq tuple
      tuple_struct map enum identifier ignored_any
  }
}
