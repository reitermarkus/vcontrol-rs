use nom::{
  branch::alt,
  combinator::{cond, map},
  IResult, Parser,
};

use crate::{
  data_type::Int32,
  method_invocation::{AnyValueWithCode, ArrayOfValueWithCode, MessageFlags, StringValueWithCode, ValueWithCode},
  record::{ArraySingleObject, RecordType},
  BinaryParser,
};

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
  pub fn parse(input: &'i [u8], parser: &mut BinaryParser<'i>) -> IResult<&'i [u8], Self> {
    map(|input| ArraySingleObject::parse(input, parser), Self)(input)
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.0.object_id()
  }
}
