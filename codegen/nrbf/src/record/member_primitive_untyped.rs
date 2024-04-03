use nom::{
  combinator::{fail, map},
  IResult,
};
#[cfg(feature = "serde")]
use serde::{
  de::value::Error,
  de::{IntoDeserializer, Visitor},
  forward_to_deserialize_any,
  ser::{Serialize, Serializer},
  Deserialize, Deserializer,
};

use crate::{
  data_type::{
    Boolean, Byte, Char, DateTime, Decimal, Double, Int16, Int32, Int64, Int8, Single, TimeSpan, UInt16, UInt32, UInt64,
  },
  enumeration::PrimitiveType,
};

/// 2.5.2 `MemberPrimitiveUnTyped`
#[cfg_attr(feature = "serde", derive(Deserialize), serde(untagged))]
#[derive(Debug, Clone, PartialEq)]
pub enum MemberPrimitiveUnTyped {
  Boolean(Boolean),
  Byte(Byte),
  Char(Char),
  Decimal(Decimal),
  Double(Double),
  Int16(Int16),
  Int32(Int32),
  Int64(Int64),
  SByte(Int8),
  Single(Single),
  TimeSpan(TimeSpan),
  DateTime(DateTime),
  UInt16(UInt16),
  UInt32(UInt32),
  UInt64(UInt64),
}

impl MemberPrimitiveUnTyped {
  pub fn parse(input: &[u8], primitive_type: PrimitiveType) -> IResult<&[u8], Self> {
    match primitive_type {
      PrimitiveType::Boolean => map(Boolean::parse, Self::Boolean)(input),
      PrimitiveType::Byte => map(Byte::parse, Self::Byte)(input),
      PrimitiveType::Char => map(Char::parse, Self::Char)(input),
      PrimitiveType::Decimal => map(Decimal::parse, Self::Decimal)(input),
      PrimitiveType::Double => map(Double::parse, Self::Double)(input),
      PrimitiveType::Int16 => map(Int16::parse, Self::Int16)(input),
      PrimitiveType::Int32 => map(Int32::parse, Self::Int32)(input),
      PrimitiveType::Int64 => map(Int64::parse, Self::Int64)(input),
      PrimitiveType::SByte => map(Int8::parse, Self::SByte)(input),
      PrimitiveType::Single => map(Single::parse, Self::Single)(input),
      PrimitiveType::TimeSpan => map(TimeSpan::parse, Self::TimeSpan)(input),
      PrimitiveType::DateTime => map(DateTime::parse, Self::DateTime)(input),
      PrimitiveType::UInt16 => map(UInt16::parse, Self::UInt16)(input),
      PrimitiveType::UInt32 => map(UInt32::parse, Self::UInt32)(input),
      PrimitiveType::UInt64 => map(UInt64::parse, Self::UInt64)(input),
      PrimitiveType::Null => fail(input),
      PrimitiveType::String => fail(input),
    }
  }
}

#[cfg(feature = "serde")]
impl Serialize for MemberPrimitiveUnTyped {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      Self::Boolean(v) => v.serialize(serializer),
      Self::Byte(v) => v.serialize(serializer),
      Self::Char(v) => v.serialize(serializer),
      Self::Decimal(v) => v.serialize(serializer),
      Self::Double(v) => v.serialize(serializer),
      Self::Int16(v) => v.serialize(serializer),
      Self::Int32(v) => v.serialize(serializer),
      Self::Int64(v) => v.serialize(serializer),
      Self::SByte(v) => v.serialize(serializer),
      Self::Single(v) => v.serialize(serializer),
      Self::TimeSpan(v) => v.serialize(serializer),
      Self::DateTime(v) => v.serialize(serializer),
      Self::UInt16(v) => v.serialize(serializer),
      Self::UInt32(v) => v.serialize(serializer),
      Self::UInt64(v) => v.serialize(serializer),
    }
  }
}

#[cfg(feature = "serde")]
impl<'de> IntoDeserializer<'de, Error> for &'de MemberPrimitiveUnTyped {
  type Deserializer = Self;

  fn into_deserializer(self) -> Self::Deserializer {
    self
  }
}

#[cfg(feature = "serde")]
impl<'de> Deserializer<'de> for &'de MemberPrimitiveUnTyped {
  type Error = Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    match self {
      MemberPrimitiveUnTyped::Boolean(v) => visitor.visit_bool((*v).into()),
      MemberPrimitiveUnTyped::SByte(v) => visitor.visit_i8((*v).into()),
      MemberPrimitiveUnTyped::Int16(v) => visitor.visit_i16((*v).into()),
      MemberPrimitiveUnTyped::Int32(v) => visitor.visit_i32((*v).into()),
      MemberPrimitiveUnTyped::Int64(v) => visitor.visit_i64((*v).into()),
      MemberPrimitiveUnTyped::Byte(v) => visitor.visit_u8((*v).into()),
      MemberPrimitiveUnTyped::UInt16(v) => visitor.visit_u16((*v).into()),
      MemberPrimitiveUnTyped::UInt32(v) => visitor.visit_u32((*v).into()),
      MemberPrimitiveUnTyped::UInt64(v) => visitor.visit_u64((*v).into()),
      MemberPrimitiveUnTyped::Single(v) => visitor.visit_f32((*v).into()),
      MemberPrimitiveUnTyped::Double(v) => visitor.visit_f64((*v).into()),
      MemberPrimitiveUnTyped::Char(v) => visitor.visit_char((*v).into()),
      MemberPrimitiveUnTyped::Decimal(v) => visitor.visit_string(v.0.to_string()),
      MemberPrimitiveUnTyped::TimeSpan(v) => visitor.visit_i64((*v).into()),
      MemberPrimitiveUnTyped::DateTime(v) => visitor.visit_i64((*v).into()),
    }
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map struct enum identifier ignored_any
  }
}
