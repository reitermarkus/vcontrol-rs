use nom::{IResult, Parser, ToUsize};

use crate::{
  data_type::Int32,
  error::{error_position, ErrorWithInput},
  record::RecordType,
};

/// 2.5.5 `ObjectNullMultiple`
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectNullMultiple {
  pub null_count: Int32,
}

impl ObjectNullMultiple {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, ErrorWithInput<'_>> {
    let (input, _) = RecordType::ObjectNullMultiple
      .parse(input)
      .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedObjectNullMultiple)))?;

    let (input, null_count) = Int32::parse_positive(input)?;

    Ok((input, Self { null_count }))
  }

  #[inline]
  pub(crate) fn null_count(&self) -> usize {
    (i32::from(self.null_count) as u32).to_usize()
  }
}
