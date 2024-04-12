use nom::IResult;

use crate::{error::Error, record::RecordType};

/// 2.6.3 `MessageEnd`
#[derive(Debug, Clone, PartialEq)]
pub struct MessageEnd;

impl MessageEnd {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    let (input, _) = RecordType::MessageEnd.parse(input)?;

    Ok((input, Self))
  }
}
