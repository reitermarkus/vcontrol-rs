use nom::{
  combinator::{fail, map},
  IResult,
};

use crate::{
  data_type::{
    Boolean, Byte, Char, DateTime, Decimal, Double, Int16, Int32, Int64, Int8, Single, TimeSpan, UInt16, UInt32, UInt64,
  },
  enumeration::PrimitiveType,
};

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
