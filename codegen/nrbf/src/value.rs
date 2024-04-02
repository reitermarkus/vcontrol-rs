use std::{
  collections::{BTreeMap, HashMap},
  fmt, iter,
};

#[cfg(feature = "serde")]
use serde::{
  de::{self, value::Error, Expected, IntoDeserializer, Visitor},
  forward_to_deserialize_any, Deserializer,
};

use crate::data_type::{DateTime, Decimal, Int32, TimeSpan};

#[derive(Debug, Clone, PartialEq)]
pub struct Object<'i> {
  pub class: &'i str,
  pub library: Option<&'i str>,
  pub members: HashMap<&'i str, Value<'i>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value<'i> {
  Object(Object<'i>),
  Array(Vec<Value<'i>>),
  Boolean(bool),
  Byte(u8),
  Char(char),
  Decimal(Decimal),
  Double(f64),
  Int16(i16),
  Int32(i32),
  Int64(i64),
  SByte(i8),
  Single(f32),
  TimeSpan(TimeSpan),
  DateTime(DateTime),
  UInt16(u16),
  UInt32(u32),
  UInt64(u64),
  String(&'i str),
  Null(usize),
  Ref(Int32),
}

struct ExpectedInArray(usize);

impl Expected for ExpectedInArray {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    if self.0 == 1 {
      formatter.write_str("1 element in array")
    } else {
      write!(formatter, "{} elements in array", self.0)
    }
  }
}

pub(crate) struct ArrayDeserializer<'de, 'o, I> {
  objects: &'o BTreeMap<Int32, Value<'de>>,
  iter: iter::Fuse<I>,
  null_count: usize,
  count: usize,
}

impl<'de, 'o, I> ArrayDeserializer<'de, 'o, I>
where
  I: Iterator,
{
  pub fn new(objects: &'o BTreeMap<Int32, Value<'de>>, iter: I) -> Self {
    Self { objects, iter: iter.fuse(), null_count: 0, count: 0 }
  }
}

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

pub(crate) struct ValueDeserializer<'de, 'o> {
  objects: &'o BTreeMap<Int32, Value<'de>>,
  object: &'o Value<'de>,
}

impl<'de, 'o> ValueDeserializer<'de, 'o> {
  pub fn new(objects: &'o BTreeMap<Int32, Value<'de>>, object: &'o Value<'de>) -> Self {
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
      Value::Object(Object { class, library, members }) => {
        if library.is_some() {
          return Err(Error::invalid_type(Unexpected::Other(class), &visitor))
        }

        let value = if let Some(value) = members.get("m_value") {
          value
        } else {
          return Err(Error::invalid_type(Unexpected::Other(class), &visitor))
        };

        match (*class, value) {
          ("System.Boolean", Value::Boolean(n)) => visitor.visit_bool((*n).into()),
          ("System.Byte", Value::Byte(n)) => visitor.visit_u8((*n).into()),
          ("System.SByte", Value::SByte(n)) => visitor.visit_i8((*n).into()),
          ("System.Char", Value::Char(c)) => visitor.visit_char((*c).into()),
          ("System.Decimal", Value::Decimal(_c)) => unimplemented!(),
          ("System.Double", Value::Double(n)) => visitor.visit_f64((*n).into()),
          ("System.Single", Value::Single(n)) => visitor.visit_f32((*n).into()),
          ("System.Int32", Value::Int32(n)) => visitor.visit_i32((*n).into()),
          ("System.UInt32", Value::UInt32(n)) => visitor.visit_u32((*n).into()),
          ("System.Int64", Value::Int64(n)) => visitor.visit_i64((*n).into()),
          ("System.UInt64", Value::UInt64(n)) => visitor.visit_u64((*n).into()),
          ("System.Int16", Value::Int16(n)) => visitor.visit_i16((*n).into()),
          ("System.UInt16", Value::UInt16(n)) => visitor.visit_u16((*n).into()),
          (name, _) => Err(Error::custom(format!("invalid system type: {}", name))),
        }
      },
      Value::Array(members) => ArrayDeserializer::new(self.objects, members.into_iter()).deserialize_any(visitor),
      Value::Ref(id) => {
        if let Some(object) = self.objects.get(&id) {
          Self::new(self.objects, object).deserialize_any(visitor)
        } else {
          Err(Error::invalid_value(Unexpected::Other("unresolved object ID"), &visitor))
        }
      },
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
      Value::Decimal(v) => visitor.visit_string(v.0.to_string()),
      Value::TimeSpan(v) => visitor.visit_i64((*v).into()),
      Value::DateTime(v) => visitor.visit_i64((*v).into()),
      Value::String(s) => visitor.visit_borrowed_str(s),
      Value::Null(1) => visitor.visit_none(),
      Value::Null(_) => Err(Error::invalid_value(Unexpected::Other("unresolved null object"), &visitor)),
    }
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map struct enum identifier ignored_any
  }
}
