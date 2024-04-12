use nom::{combinator::cond, IResult, Parser};

use crate::{
  error::{error_position, ErrorWithInput},
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
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, ErrorWithInput<'i>> {
    let (input, _) = RecordType::MethodCall
      .parse(input)
      .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedBinaryMethodCall)))?;

    let (input, message_enum) =
      MessageFlags::parse(input).map_err(|err| err.map(|err| error_position!(err.input, ExpectedMessageFlags)))?;
    let (input, method_name) = StringValueWithCode::parse(input)?;
    let (input, type_name) = StringValueWithCode::parse(input)?;
    let (input, call_context) =
      cond(message_enum.intersects(MessageFlags::CONTEXT_INLINE), StringValueWithCode::parse)(input)?;
    let (input, args) = cond(message_enum.intersects(MessageFlags::ARGS_INLINE), ArrayOfValueWithCode::parse)(input)
      .map_err(|err| err.map(|err| error_position!(err.input, ExpectedArrayOfValueWithCode)))?;

    Ok((input, Self { message_enum, method_name, type_name, call_context, args }))
  }
}
