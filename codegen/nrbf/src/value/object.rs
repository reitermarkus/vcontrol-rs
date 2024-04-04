use std::collections::HashMap;
#[cfg(feature = "serde")]
use std::{collections::BTreeMap, fmt};

#[cfg(feature = "serde")]
use serde::{
  de::Expected,
  de::{self, value::Error, Visitor},
  forward_to_deserialize_any,
};

use super::Value;
#[cfg(feature = "serde")]
use super::{resolve_object, ArrayDeserializer, ValueDeserializer};

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
    use serde::{
      de::{value::MapDeserializer, Error, Unexpected},
      Deserialize,
    };

    let Object { class, library, members } = self.object;

    let map_deserializer =
      MapDeserializer::new(members.iter().map(|(key, value)| (*key, ValueDeserializer::new(self.objects, value))));

    if library.is_some() {
      return map_deserializer.deserialize_map(visitor)
    }

    let class_name = class.split_once('`').map(|(s, _)| s).unwrap_or(*class);

    match class_name {
      "System.Boolean" => {
        #[derive(Deserialize)]
        struct Boolean {
          m_value: bool,
        }

        let v = Boolean::deserialize(map_deserializer).map(|v| v.m_value)?;
        return visitor.visit_bool(v)
      },
      "System.Byte" => {
        #[derive(Deserialize)]
        struct Byte {
          m_value: u8,
        }

        let v = Byte::deserialize(map_deserializer).map(|v| v.m_value)?;
        return visitor.visit_u8(v)
      },
      "System.SByte" => {
        #[derive(Deserialize)]
        struct SByte {
          m_value: i8,
        }

        let v = SByte::deserialize(map_deserializer).map(|v| v.m_value)?;
        return visitor.visit_i8(v)
      },
      "System.Char" => {
        #[derive(Deserialize)]
        struct Char {
          m_value: char,
        }

        let v = Char::deserialize(map_deserializer).map(|v| v.m_value)?;
        return visitor.visit_char(v)
      },
      "System.Decimal" => {
        if members.len() == 1 {
          if let Some(Value::Decimal(_c)) = members.get("m_value") {
            unimplemented!()
          }
        }
      },
      "System.Double" => {
        #[derive(Deserialize)]
        struct Double {
          m_value: f64,
        }

        let v = Double::deserialize(map_deserializer).map(|v| v.m_value)?;
        return visitor.visit_f64(v)
      },
      "System.Single" => {
        #[derive(Deserialize)]
        struct Single {
          m_value: f32,
        }

        let v = Single::deserialize(map_deserializer).map(|v| v.m_value)?;
        return visitor.visit_f32(v)
      },
      "System.Int32" => {
        #[derive(Deserialize)]
        struct Int32 {
          m_value: i32,
        }

        let v = Int32::deserialize(map_deserializer).map(|v| v.m_value)?;
        return visitor.visit_i32(v)
      },
      "System.UInt32" => {
        #[derive(Deserialize)]
        struct UInt32 {
          m_value: u32,
        }

        let v = UInt32::deserialize(map_deserializer).map(|v| v.m_value)?;
        return visitor.visit_u32(v)
      },
      "System.Int64" => {
        #[derive(Deserialize)]
        struct Int64 {
          m_value: i64,
        }

        let v = Int64::deserialize(map_deserializer).map(|v| v.m_value)?;
        return visitor.visit_i64(v)
      },
      "System.UInt64" => {
        #[derive(Deserialize)]
        struct UInt64 {
          m_value: u64,
        }

        let v = UInt64::deserialize(map_deserializer).map(|v| v.m_value)?;
        return visitor.visit_u64(v)
      },
      "System.Int16" => {
        #[derive(Deserialize)]
        struct Int16 {
          m_value: i16,
        }

        let v = Int16::deserialize(map_deserializer).map(|v| v.m_value)?;
        return visitor.visit_i16(v)
      },
      "System.UInt16" => {
        #[derive(Deserialize)]
        struct UInt16 {
          m_value: u16,
        }

        let v = UInt16::deserialize(map_deserializer).map(|v| v.m_value)?;
        return visitor.visit_u16(v)
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
              return ListDeserializer::new(self.objects, items.iter(), (*size) as usize).deserialize_any(visitor)
            }
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

    MapDeserializer::new(members.iter().map(|(key, value)| (*key, ValueDeserializer::new(self.objects, value))))
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
pub(crate) struct ListDeserializer<'de, 'o, I> {
  array_deserializer: ArrayDeserializer<'de, 'o, I>,
  count: usize,
  size: usize,
}

#[cfg(feature = "serde")]
impl<'de, 'o, I> ListDeserializer<'de, 'o, I>
where
  I: Iterator,
{
  pub fn new(objects: &'o BTreeMap<i32, Value<'de>>, iter: I, size: usize) -> Self {
    Self { array_deserializer: ArrayDeserializer::new(objects, iter), count: 0, size }
  }
}

#[cfg(feature = "serde")]
impl<'de, 'o, I> ListDeserializer<'de, 'o, I>
where
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
impl<'de, 'o, I> de::Deserializer<'de> for ListDeserializer<'de, 'o, I>
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
impl<'de, 'o, I> de::SeqAccess<'de> for ListDeserializer<'de, 'o, I>
where
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
