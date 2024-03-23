//! 2.2 Method Invocation Records

use nom::{
  branch::alt,
  bytes::complete::tag,
  combinator::{map, map_res, value},
  number::complete::le_i32,
  sequence::preceded,
  IResult,
};

use super::{
  data_type::{
    Boolean, Byte, Char, DateTime, Decimal, Double, Int16, Int32, Int64, Int8, LengthPrefixedString, Single, TimeSpan,
    UInt16, UInt32, UInt64,
  },
  enumeration::PrimitiveType,
};

/// 2.2.1.1 `MessageFlags`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MessageFlags(pub Int32);

#[rustfmt::skip]
impl MessageFlags {
  pub const NO_ARGS:                   Int32 = Int32(0x00000001);
  pub const ARGS_INLINE:               Int32 = Int32(0x00000002);
  pub const ARGS_IS_ARRAY:             Int32 = Int32(0x00000004);
  pub const ARGS_IN_ARRAY:             Int32 = Int32(0x00000008);

  pub const NO_CONTEXT:                Int32 = Int32(0x00000010);
  pub const CONTEXT_INLINE:            Int32 = Int32(0x00000020);
  pub const CONTEXT_IN_ARRAY:          Int32 = Int32(0x00000040);

  pub const METHOD_SIGNATURE_IN_ARRAY: Int32 = Int32(0x00000080);

  pub const PROPERTIES_IN_ARRAY:       Int32 = Int32(0x00000100);

  pub const NO_RETURN_VALUE:           Int32 = Int32(0x00000200);
  pub const RETURN_VALUE_VOID:         Int32 = Int32(0x00000400);
  pub const RETURN_VALUE_INLINE:       Int32 = Int32(0x00000800);
  pub const RETURN_VALUE_IN_ARRAY:     Int32 = Int32(0x00001000);

  pub const EXCEPTION_IN_ARRAY:        Int32 = Int32(0x00002000);

  pub const GENERIC_METHOD:            Int32 = Int32(0x00008000);

  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map_res(map(Int32::parse, Self), |flags| {
      let args_flags =
        (flags.0).0 & (Self::NO_ARGS.0 | Self::ARGS_INLINE.0 | Self::ARGS_IS_ARRAY.0 | Self::ARGS_IN_ARRAY.0);
      let context_flags = (flags.0).0 & (Self::NO_CONTEXT.0 | Self::CONTEXT_INLINE.0 | Self::CONTEXT_IN_ARRAY.0);
      let return_flags = (flags.0).0
        & (Self::NO_RETURN_VALUE.0
          | Self::RETURN_VALUE_VOID.0
          | Self::RETURN_VALUE_IN_ARRAY.0
          | Self::RETURN_VALUE_IN_ARRAY.0);
      let signature_flags = (flags.0).0 & Self::METHOD_SIGNATURE_IN_ARRAY.0;
      let exception_flags = (flags.0).0 & Self::EXCEPTION_IN_ARRAY.0;

      // For each flags category given in the preceding table (Arg, Context, Signature, Return, Exception,
      // Property, and Generic), the value MUST NOT have more than one flag from the Category set.
      if args_flags.count_ones() > 1 || context_flags.count_ones() > 1 || return_flags.count_ones() > 1 {
        return Err(())
      }

      // The Args and Exception flag categories are exclusive: if a flag from the Args category is set, the
      // value MUST NOT have any flag from the Exception category set, and vice versa.
      if args_flags != 0 && exception_flags != 0 {
        return Err(())
      }

      // The Return and Exception flag categories are exclusive: if a flag from the Return category is set,
      // the value MUST NOT have any flag from the Exception category set, and vice versa.
      if return_flags != 0 && exception_flags != 0 {
        return Err(())
      }

      // The Return and Signature categories are exclusive: if a flag from the Return category is set, the
      // value MUST NOT have any flag from the Signature category set, and vice versa.
      if return_flags != 0 && signature_flags != 0 {
        return Err(())
      }

      // The Exception and Signature categories are exclusive: if a flag from the Signature category is set,
      // the value MUST NOT have any flag from the Exception category set, and vice versa.
      if exception_flags != 0 && signature_flags != 0 {
        return Err(())
      }

      Ok(flags)
    })(input)
  }
}

/// 2.2.2.1 `ValueWithCode`
#[derive(Debug, Clone, PartialEq)]
pub enum ValueWithCode {
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
  Null,
}

impl ValueWithCode {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    alt((
      map(preceded(PrimitiveType::Boolean, Boolean::parse), Self::Boolean),
      map(preceded(PrimitiveType::Byte, Byte::parse), Self::Byte),
      map(preceded(PrimitiveType::Char, Char::parse), Self::Char),
      map(preceded(PrimitiveType::Decimal, Decimal::parse), Self::Decimal),
      map(preceded(PrimitiveType::Double, Double::parse), Self::Double),
      map(preceded(PrimitiveType::Int16, Int16::parse), Self::Int16),
      map(preceded(PrimitiveType::Int32, Int32::parse), Self::Int32),
      map(preceded(PrimitiveType::Int64, Int64::parse), Self::Int64),
      map(preceded(PrimitiveType::SByte, Int8::parse), Self::SByte),
      map(preceded(PrimitiveType::Single, Single::parse), Self::Single),
      map(preceded(PrimitiveType::TimeSpan, TimeSpan::parse), Self::TimeSpan),
      map(preceded(PrimitiveType::DateTime, DateTime::parse), Self::DateTime),
      map(preceded(PrimitiveType::UInt16, UInt16::parse), Self::UInt16),
      map(preceded(PrimitiveType::UInt32, UInt32::parse), Self::UInt32),
      map(preceded(PrimitiveType::UInt64, UInt64::parse), Self::UInt64),
      value(Self::Null, PrimitiveType::Null),
    ))(input)
  }
}

/// 2.2.2.2 `StringValueWithCode`
#[derive(Debug, Clone, PartialEq)]
pub struct StringValueWithCode<'i>(LengthPrefixedString<'i>);

impl<'i> StringValueWithCode<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    map(preceded(tag([18]), LengthPrefixedString::parse), Self)(input)
  }
}

impl<'s> From<LengthPrefixedString<'s>> for StringValueWithCode<'s> {
  fn from(s: LengthPrefixedString<'s>) -> Self {
    Self(s)
  }
}
