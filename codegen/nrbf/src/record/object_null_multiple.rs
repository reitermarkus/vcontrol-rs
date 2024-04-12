use nom::{IResult, ToUsize};

use crate::{
  data_type::Int32,
  error::{error_position, Error},
  record::RecordType,
};

/// 2.5.5 `ObjectNullMultiple`
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectNullMultiple {
  pub null_count: Int32,
}

impl ObjectNullMultiple {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    let (input, _) = RecordType::ObjectNullMultiple.parse(input)?;

    match Int32::parse(input) {
      Ok((input, null_count)) if null_count.0 > 0 => Ok((input, Self { null_count })),
      Ok(_) => Err(nom::Err::Failure(error_position!(input, InvalidNullCount))),
      Err(err) => Err(err),
    }
  }

  #[inline]
  pub(crate) fn null_count(&self) -> usize {
    (i32::from(self.null_count) as u32).to_usize()
  }
}
