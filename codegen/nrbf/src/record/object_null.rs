use nom::{IResult, Parser};

use crate::{
  error::{error_position, ErrorWithInput},
  record::RecordType,
};

/// 2.5.4 `ObjectNull`
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectNull;

impl ObjectNull {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, ErrorWithInput<'_>> {
    let (input, _) = RecordType::ObjectNull
      .parse(input)
      .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedObjectNull)))?;

    Ok((input, Self))
  }

  #[inline]
  pub(crate) fn null_count(&self) -> usize {
    1
  }
}
