use nom::{IResult, Parser};

use crate::record::RecordType;

/// 2.5.4 `ObjectNull`
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectNull;

impl ObjectNull {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = RecordType::ObjectNull.parse(input)?;

    Ok((input, Self))
  }
}