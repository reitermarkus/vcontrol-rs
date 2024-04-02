use std::{collections::BTreeMap, fmt, iter};

#[cfg(feature = "serde")]
use serde::{
  de::{self, value::Error, Expected, IntoDeserializer, Visitor},
  forward_to_deserialize_any, Deserializer,
};

use crate::{data_type::Int32, record::MemberPrimitiveUnTyped};

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectClass<'i> {
  pub name: &'i str,
  pub library: Option<&'i str>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object<'i> {
  Object { class: ObjectClass<'i>, members: BTreeMap<&'i str, Object<'i>> },
  Array(Vec<Object<'i>>),
  Primitive(MemberPrimitiveUnTyped),
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

pub(crate) struct ObjectArrayDeserializer<'de, 'o, I> {
  objects: &'o BTreeMap<Int32, Object<'de>>,
  iter: iter::Fuse<I>,
  null_count: usize,
  count: usize,
}

impl<'de, 'o, I> ObjectArrayDeserializer<'de, 'o, I>
where
  I: Iterator,
{
  pub fn new(objects: &'o BTreeMap<Int32, Object<'de>>, iter: I) -> Self {
    Self { objects, iter: iter.fuse(), null_count: 0, count: 0 }
  }
}

impl<'de, I> ObjectArrayDeserializer<'de, '_, I>
where
  I: Iterator,
{
  /// Check for remaining elements after passing a `SeqDeserializer` to
  /// `Visitor::visit_seq`.
  pub fn end<E: de::Error>(self) -> Result<(), E> {
    let remaining = self.iter.count() + self.null_count;
    if remaining == 0 {
      Ok(())
    } else {
      // First argument is the number of elements in the data, second
      // argument is the number of elements expected by the Deserialize.
      Err(de::Error::invalid_length(self.count + remaining, &ExpectedInArray(self.count)))
    }
  }
}

impl<'de, 'o, I> de::Deserializer<'de> for ObjectArrayDeserializer<'de, 'o, I>
where
  I: Iterator<Item = &'o Object<'de>>,
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

impl<'de, 'o, I> de::SeqAccess<'de> for ObjectArrayDeserializer<'de, 'o, I>
where
  I: Iterator<Item = &'o Object<'de>>,
{
  type Error = Error;

  fn next_element_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>, Self::Error>
  where
    V: de::DeserializeSeed<'de>,
  {
    if self.null_count > 0 {
      self.count += 1;
      self.null_count -= 1;
      return seed.deserialize(Object::Null(1)).map(Some)
    }

    match self.iter.next() {
      Some(Object::Null(null_count @ 2..)) => {
        self.count += 1;
        self.null_count = null_count - 1;
        seed.deserialize(Object::Null(1)).map(Some)
      },
      Some(object) => {
        self.count += 1;
        seed.deserialize(ObjectDeserializer::new(self.objects, object)).map(Some)
      },
      None => Ok(None),
    }
  }
}

pub(crate) struct ObjectDeserializer<'de, 'o> {
  objects: &'o BTreeMap<Int32, Object<'de>>,
  object: &'o Object<'de>,
}

impl<'de, 'o> ObjectDeserializer<'de, 'o> {
  pub fn new(objects: &'o BTreeMap<Int32, Object<'de>>, object: &'o Object<'de>) -> Self {
    Self { objects, object }
  }
}

#[cfg(feature = "serde")]
impl<'de> IntoDeserializer<'de, Error> for ObjectDeserializer<'de, '_> {
  type Deserializer = Self;

  fn into_deserializer(self) -> Self::Deserializer {
    self
  }
}

#[cfg(feature = "serde")]
impl<'de> Deserializer<'de> for ObjectDeserializer<'de, '_> {
  type Error = Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    use serde::de::{Error, Unexpected};

    match self.object {
      Object::Object { class, members } => {
        if class.library.is_some() {
          return Err(Error::invalid_type(Unexpected::Other(class.name), &visitor))
        }

        let value = if let Some(value) = members.get("m_value") {
          value
        } else {
          return Err(Error::invalid_type(Unexpected::Other(class.name), &visitor))
        };

        match (class.name, value) {
          ("System.Boolean", Object::Primitive(MemberPrimitiveUnTyped::Boolean(n))) => visitor.visit_bool((*n).into()),
          ("System.Byte", Object::Primitive(MemberPrimitiveUnTyped::Byte(n))) => visitor.visit_u8((*n).into()),
          ("System.SByte", Object::Primitive(MemberPrimitiveUnTyped::SByte(n))) => visitor.visit_i8((*n).into()),
          ("System.Char", Object::Primitive(MemberPrimitiveUnTyped::Char(c))) => visitor.visit_char((*c).into()),
          ("System.Decimal", Object::Primitive(MemberPrimitiveUnTyped::Decimal(_c))) => unimplemented!(),
          ("System.Double", Object::Primitive(MemberPrimitiveUnTyped::Double(n))) => visitor.visit_f64((*n).into()),
          ("System.Single", Object::Primitive(MemberPrimitiveUnTyped::Single(n))) => visitor.visit_f32((*n).into()),
          ("System.Int32", Object::Primitive(MemberPrimitiveUnTyped::Int32(n))) => visitor.visit_i32((*n).into()),
          ("System.UInt32", Object::Primitive(MemberPrimitiveUnTyped::UInt32(n))) => visitor.visit_u32((*n).into()),
          ("System.Int64", Object::Primitive(MemberPrimitiveUnTyped::Int64(n))) => visitor.visit_i64((*n).into()),
          ("System.UInt64", Object::Primitive(MemberPrimitiveUnTyped::UInt64(n))) => visitor.visit_u64((*n).into()),
          ("System.Int16", Object::Primitive(MemberPrimitiveUnTyped::Int16(n))) => visitor.visit_i16((*n).into()),
          ("System.UInt16", Object::Primitive(MemberPrimitiveUnTyped::UInt16(n))) => visitor.visit_u16((*n).into()),
          (name, _) => Err(Error::custom(format!("invalid system type: {}", name))),
        }
      },
      Object::Array(members) => {
        ObjectArrayDeserializer::new(self.objects, members.into_iter()).deserialize_any(visitor)
      },
      Object::Ref(id) => {
        if let Some(object) = self.objects.get(&id) {
          Self::new(self.objects, object).deserialize_any(visitor)
        } else {
          Err(Error::invalid_value(Unexpected::Other("unresolved object ID"), &visitor))
        }
      },
      Object::Primitive(primitive) => primitive.clone().deserialize_any(visitor),
      Object::String(s) => visitor.visit_borrowed_str(s),
      Object::Null(1) => visitor.visit_none(),
      Object::Null(_) => Err(Error::invalid_value(Unexpected::Other("unresolved null object"), &visitor)),
    }
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map struct enum identifier ignored_any
  }
}

#[cfg(feature = "serde")]
impl<'de> IntoDeserializer<'de, Error> for Object<'de> {
  type Deserializer = Self;

  fn into_deserializer(self) -> Self::Deserializer {
    self
  }
}

#[cfg(feature = "serde")]
impl<'de> Deserializer<'de> for Object<'de> {
  type Error = Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    ObjectDeserializer::new(&Default::default(), &self).deserialize_any(visitor)
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map struct enum identifier ignored_any
  }
}
