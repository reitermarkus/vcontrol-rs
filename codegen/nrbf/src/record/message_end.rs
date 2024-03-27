use nom::{bytes::complete::tag, IResult, Parser};

use crate::record::RecordType;

/// 2.6.3 `MessageEnd`
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct MessageEnd;

impl MessageEnd {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = RecordType::MessageEnd.parse(input)?;

    Ok((input, Self))
  }
}
