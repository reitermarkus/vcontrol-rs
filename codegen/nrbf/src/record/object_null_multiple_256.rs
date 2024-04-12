use nom::{IResult, Parser, ToUsize};

use crate::{
  data_type::Byte,
  error::{error_position, ErrorWithInput},
  record::RecordType,
};

/// 2.5.6 `ObjectNullMultiple256`
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectNullMultiple256 {
  pub null_count: Byte,
}

impl ObjectNullMultiple256 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, ErrorWithInput<'_>> {
    let (input, _) = RecordType::ObjectNullMultiple256.parse(input).map_err(|err| {
      err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedObjectNullMultiple256))
    })?;

    let (input, null_count) = Byte::parse_positive(input)?;

    Ok((input, Self { null_count }))
  }

  #[inline]
  pub(crate) fn null_count(&self) -> usize {
    u8::from(self.null_count).to_usize()
  }
}
