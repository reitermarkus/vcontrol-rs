use nom::{IResult, Parser, ToUsize};

use crate::{data_type::Byte, record::RecordType};

/// 2.5.6 `ObjectNullMultiple256`
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectNullMultiple256 {
  pub null_count: Byte,
}

impl ObjectNullMultiple256 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = RecordType::ObjectNullMultiple256.parse(input)?;

    let (input, null_count) = Byte::parse(input)?;

    Ok((input, Self { null_count }))
  }

  pub(crate) fn null_count(&self) -> usize {
    u8::from(self.null_count).to_usize()
  }
}
