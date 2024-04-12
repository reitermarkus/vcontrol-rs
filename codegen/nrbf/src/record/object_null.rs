use nom::IResult;

use crate::{error::Error, record::RecordType};

/// 2.5.4 `ObjectNull`
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectNull;

impl ObjectNull {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    let (input, _) = RecordType::ObjectNull.parse(input)?;

    Ok((input, Self))
  }

  #[inline]
  pub(crate) fn null_count(&self) -> usize {
    1
  }
}
