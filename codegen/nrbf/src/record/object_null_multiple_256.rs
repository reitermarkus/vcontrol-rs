use nom::{IResult, ToUsize};

use crate::{
  data_type::Byte,
  error::{error_position, Error},
  record::RecordType,
};

/// 2.5.6 `ObjectNullMultiple256`
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectNullMultiple256 {
  pub null_count: Byte,
}

impl ObjectNullMultiple256 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    let (input, _) = RecordType::ObjectNullMultiple256.parse(input)?;

    match Byte::parse(input) {
      Ok((input, null_count)) if null_count.0 > 0 => Ok((input, Self { null_count })),
      Ok(_) => Err(nom::Err::Failure(error_position!(input, InvalidNullCount))),
      Err(err) => Err(err),
    }
  }

  #[inline]
  pub(crate) fn null_count(&self) -> usize {
    u8::from(self.null_count).to_usize()
  }
}
