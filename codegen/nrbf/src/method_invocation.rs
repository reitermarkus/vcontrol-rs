//! 2.2 Method Invocation Records

use bitflags::bitflags;
use nom::{
  branch::alt,
  combinator::{cond, map, map_res, value},
  multi::many_m_n,
  number::complete::le_i32,
  sequence::preceded,
  IResult, Parser, ToUsize,
};

use super::{
  data_type::{
    Boolean, Byte, Char, DateTime, Decimal, Double, Int16, Int32, Int64, Int8, LengthPrefixedString, Single, TimeSpan,
    UInt16, UInt32, UInt64,
  },
  enumeration::{PrimitiveType, RecordType},
  ArraySingleObject,
};

bitflags! {
  /// 2.2.1.1 `MessageFlags`
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub struct MessageFlags: i32 {
    /// The record contains no arguments.
    /// It is in the Arg category.
    const NO_ARGS                   = 0x00000001;
    /// The Arguments Array is in the Args field of the Method record.
    /// It is in the Arg category.
    const ARGS_INLINE               = 0x00000002;
    /// Each argument is an item in a separate Call Array record.
    /// It is in the Arg category.
    const ARGS_IS_ARRAY             = 0x00000004;
    /// The Arguments Array is an item in a separate Call Array record.
    /// It is in the Arg category.
    const ARGS_IN_ARRAY             = 0x00000008;

    /// The record does not contain a Call Context value.
    /// It is in the Context category.
    const NO_CONTEXT                = 0x00000010;
    /// Call Context contains only a Logical Call ID value and is in
    /// the CallContext field of the Method record.
    /// It is in the Context category.
    const CONTEXT_INLINE            = 0x00000020;
    /// CallContext values are contained in an array that is contained in the Call Array record.
    /// It is in the Context category.
    const CONTEXT_IN_ARRAY          = 0x00000040;

    /// The Method Signature is contained in the Call Array record.
    /// It is in the Signature category.
    const METHOD_SIGNATURE_IN_ARRAY = 0x00000080;

    /// Message Properties is contained in the Call Array record.
    /// It is in the Property category.
    const PROPERTIES_IN_ARRAY       = 0x00000100;

    /// The Return Value is a Null object.
    /// It is in the Return category.
    const NO_RETURN_VALUE           = 0x00000200;
    /// The method has no Return Value.
    /// It is in the Return category.
    const RETURN_VALUE_VOID         = 0x00000400;
    /// The Return Value is in the ReturnValue field of the MethodReturnCallArray record.
    /// It is in the Return category.
    const RETURN_VALUE_INLINE       = 0x00000800;
    /// The Return Value is contained in the MethodReturnCallArray record.
    /// It is in the Return category.
    const RETURN_VALUE_IN_ARRAY     = 0x00001000;

    /// An Exception is contained in the MethodReturnCallArray record.
    /// It is in the Exception category.
    const EXCEPTION_IN_ARRAY        = 0x00002000;

    /// The Remote Method is generic and the actual Remoting Types
    /// for the Generic Arguments are contained in the Call Array.
    /// It is in the Generic category.
    const GENERIC_METHOD            = 0x00008000;
  }
}

#[rustfmt::skip]
impl MessageFlags {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map_res(map(Int32::parse, |n| Self::from_bits_retain(n.0)), |flags| {
      let args_flags = flags.intersection(

      Self::NO_ARGS.union(Self::ARGS_INLINE).union(

      Self::ARGS_IS_ARRAY).union(Self::ARGS_IN_ARRAY)
      );
      let context_flags = flags.intersection(Self::NO_CONTEXT.union(Self::CONTEXT_INLINE).union(Self::CONTEXT_IN_ARRAY));
      let return_flags = flags.intersection(Self::NO_RETURN_VALUE.union(
          Self::RETURN_VALUE_VOID).union(
          Self::RETURN_VALUE_IN_ARRAY).union(
          Self::RETURN_VALUE_IN_ARRAY));
      let signature_flags = flags.intersection(Self::METHOD_SIGNATURE_IN_ARRAY);
      let exception_flags = flags.intersection(Self::EXCEPTION_IN_ARRAY);

      // For each flags category given in the preceding table (Arg, Context, Signature, Return, Exception,
      // Property, and Generic), the value MUST NOT have more than one flag from the Category set.
      if args_flags.bits().count_ones() > 1 || context_flags.bits().count_ones() > 1 || return_flags.bits().count_ones() > 1 {
        return Err(())
      }

      // The Args and Exception flag categories are exclusive: if a flag from the Args category is set, the
      // value MUST NOT have any flag from the Exception category set, and vice versa.
      if !args_flags.is_empty() && !exception_flags.is_empty() {
        return Err(())
      }

      // The Return and Exception flag categories are exclusive: if a flag from the Return category is set,
      // the value MUST NOT have any flag from the Exception category set, and vice versa.
      if !return_flags.is_empty() && !exception_flags.is_empty() {
        return Err(())
      }

      // The Return and Signature categories are exclusive: if a flag from the Return category is set, the
      // value MUST NOT have any flag from the Signature category set, and vice versa.
      if !return_flags.is_empty() && !signature_flags.is_empty() {
        return Err(())
      }

      // The Exception and Signature categories are exclusive: if a flag from the Signature category is set,
      // the value MUST NOT have any flag from the Exception category set, and vice versa.
      if !exception_flags.is_empty() && !signature_flags.is_empty() {
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
pub enum AnyValueWithCode<'i> {
  Primitive(ValueWithCode),
  String(StringValueWithCode<'i>),
}

/// 2.2.2.2 `StringValueWithCode`
#[derive(Debug, Clone, PartialEq)]
pub struct StringValueWithCode<'i>(LengthPrefixedString<'i>);

impl<'i> StringValueWithCode<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    map(preceded(PrimitiveType::String, LengthPrefixedString::parse), Self)(input)
  }
}

impl<'s> From<LengthPrefixedString<'s>> for StringValueWithCode<'s> {
  fn from(s: LengthPrefixedString<'s>) -> Self {
    Self(s)
  }
}

/// 2.2.2.3 `ArrayOfValueWithCode`
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayOfValueWithCode(Vec<ValueWithCode>);

impl ArrayOfValueWithCode {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, length) = map_res(Int32::parse, usize::try_from)(input)?;
    map(many_m_n(length, length, ValueWithCode::parse), Self)(input)
  }
}

/// 2.2.3.1 `BinaryMethodCall`
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryMethodCall<'i> {
  pub message_enum: MessageFlags,
  pub method_name: StringValueWithCode<'i>,
  pub type_name: StringValueWithCode<'i>,
  pub call_context: Option<StringValueWithCode<'i>>,
  pub args: Option<ArrayOfValueWithCode>,
}

impl<'i> BinaryMethodCall<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = RecordType::MethodCall.parse(input)?;

    let (input, message_enum) = MessageFlags::parse(input)?;
    let (input, method_name) = StringValueWithCode::parse(input)?;
    let (input, type_name) = StringValueWithCode::parse(input)?;
    let (input, call_context) =
      cond(message_enum.intersects(MessageFlags::CONTEXT_INLINE), StringValueWithCode::parse)(input)?;
    let (input, args) = cond(message_enum.intersects(MessageFlags::ARGS_INLINE), ArrayOfValueWithCode::parse)(input)?;

    Ok((input, Self { message_enum, method_name, type_name, call_context, args }))
  }
}

/// 2.2.3.2 `MethodCallArray`
#[derive(Debug, Clone, PartialEq)]
pub struct MethodCallArray<'i>(pub ArraySingleObject<'i>);

impl<'i> MethodCallArray<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    map(ArraySingleObject::parse, Self)(input)
  }
}

/// 2.2.3.3 `BinaryMethodReturn`
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryMethodReturn<'i> {
  pub message_enum: MessageFlags,
  pub return_value: Option<AnyValueWithCode<'i>>,
  pub call_context: Option<StringValueWithCode<'i>>,
  pub args: Option<ArrayOfValueWithCode>,
}

impl<'i> BinaryMethodReturn<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = RecordType::MethodReturn.parse(input)?;

    let (input, message_enum) = MessageFlags::parse(input)?;
    let (input, return_value) = cond(
      message_enum.intersects(MessageFlags::RETURN_VALUE_INLINE),
      alt((
        map(ValueWithCode::parse, AnyValueWithCode::Primitive),
        map(StringValueWithCode::parse, AnyValueWithCode::String),
      )),
    )(input)?;
    let (input, call_context) =
      cond(message_enum.intersects(MessageFlags::CONTEXT_INLINE), StringValueWithCode::parse)(input)?;
    let (input, args) = cond(message_enum.intersects(MessageFlags::ARGS_INLINE), ArrayOfValueWithCode::parse)(input)?;

    Ok((input, Self { message_enum, return_value, call_context, args }))
  }
}

/// 2.2.3.4 `MethodReturnCallArray`
#[derive(Debug, Clone, PartialEq)]
pub struct MethodReturnCallArray<'i>(pub ArraySingleObject<'i>);

impl<'i> MethodReturnCallArray<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    map(ArraySingleObject::parse, Self)(input)
  }
}
