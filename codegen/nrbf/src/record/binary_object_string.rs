use nom::{IResult, Parser};
#[cfg(feature = "serde")]
use serde::{
  de::value::Error,
  de::{IntoDeserializer, Visitor},
  forward_to_deserialize_any, Deserializer,
};

use crate::{
  data_type::{Int32, LengthPrefixedString},
  record::RecordType,
};

/// 2.5.7 `BinaryObjectString`
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryObjectString<'s> {
  pub object_id: Int32,
  pub value: LengthPrefixedString<'s>,
}

impl<'i> BinaryObjectString<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = RecordType::BinaryObjectString.parse(input)?;

    let (input, object_id) = Int32::parse_positive(input)?;
    let (input, value) = LengthPrefixedString::parse(input)?;

    Ok((input, Self { object_id, value }))
  }

  pub fn as_str(&self) -> &'i str {
    self.value.as_str()
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.object_id
  }
}

#[cfg(feature = "serde")]
impl<'de> IntoDeserializer<'de, Error> for BinaryObjectString<'de> {
  type Deserializer = Self;

  fn into_deserializer(self) -> Self::Deserializer {
    self
  }
}

#[cfg(feature = "serde")]
impl<'de> Deserializer<'de> for BinaryObjectString<'de> {
  type Error = Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_borrowed_str(self.as_str())
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map struct enum identifier ignored_any
  }
}
