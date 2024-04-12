use nom::{IResult, Parser};

use crate::{
  error::{error_position, ErrorWithInput},
  record::RecordType,
};

/// 2.6.3 `MessageEnd`
#[derive(Debug, Clone, PartialEq)]
pub struct MessageEnd;

impl MessageEnd {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, ErrorWithInput<'_>> {
    let (input, _) = RecordType::MessageEnd
      .parse(input)
      .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedMessageEnd)))?;

    Ok((input, Self))
  }
}
