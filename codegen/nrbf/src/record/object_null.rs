use nom::{branch::alt, combinator::map, IResult, Parser, ToUsize};

use crate::{
  data_type::{Byte, Int32},
  record::RecordType,
};

/// 2.5.4 `ObjectNull`
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct ObjectNull;

impl ObjectNull {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = RecordType::ObjectNull.parse(input)?;

    Ok((input, Self))
  }
}
