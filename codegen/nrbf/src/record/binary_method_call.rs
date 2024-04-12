use nom::{combinator::cond, IResult};

use crate::{
  error::Error,
  record::{ArrayOfValueWithCode, MessageFlags, RecordType, StringValueWithCode},
};

/// 2.2.3.1 `BinaryMethodCall`
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryMethodCall<'i> {
  pub message_enum: MessageFlags,
  pub method_name: StringValueWithCode<'i>,
  pub type_name: StringValueWithCode<'i>,
  pub call_context: Option<StringValueWithCode<'i>>,
  pub args: Option<ArrayOfValueWithCode<'i>>,
}

impl<'i> BinaryMethodCall<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, Error<'i>> {
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
