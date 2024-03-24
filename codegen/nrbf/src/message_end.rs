use nom::{bytes::complete::tag, IResult};

/// 2.6.3 `MessageEnd`
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct MessageEnd;

impl MessageEnd {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = tag([11])(input)?;

    Ok((input, Self))
  }
}
