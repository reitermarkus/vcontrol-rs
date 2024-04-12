use nom::{combinator::cond, IResult, Parser};

use crate::{
  error::{error_position, ErrorWithInput},
  record::{ArrayOfValueWithCode, MessageFlags, RecordType, StringValueWithCode, ValueWithCode},
};

/// 2.2.3.3 `BinaryMethodReturn`
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryMethodReturn<'i> {
  pub message_enum: MessageFlags,
  pub return_value: Option<ValueWithCode<'i>>,
  pub call_context: Option<StringValueWithCode<'i>>,
  pub args: Option<ArrayOfValueWithCode<'i>>,
}

impl<'i> BinaryMethodReturn<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, ErrorWithInput<'i>> {
    let (input, _) = RecordType::MethodReturn
      .parse(input)
      .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedBinaryMethodReturn)))?;

    let (input, message_enum) =
      MessageFlags::parse(input).map_err(|err| err.map(|err| error_position!(err.input, ExpectedMessageFlags)))?;
    let (input, return_value) =
      cond(message_enum.intersects(MessageFlags::RETURN_VALUE_INLINE), ValueWithCode::parse)(input)
        .map_err(|err| err.map(|err| error_position!(err.input, ExpectedValueWithCode)))?;
    let (input, call_context) =
      cond(message_enum.intersects(MessageFlags::CONTEXT_INLINE), StringValueWithCode::parse)(input)?;
    let (input, args) = cond(message_enum.intersects(MessageFlags::ARGS_INLINE), ArrayOfValueWithCode::parse)(input)
      .map_err(|err| err.map(|err| error_position!(err.input, ExpectedArrayOfValueWithCode)))?;

    Ok((input, Self { message_enum, return_value, call_context, args }))
  }
}
