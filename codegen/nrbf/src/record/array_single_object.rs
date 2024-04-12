use nom::{IResult, Parser};

use crate::{
  common::ArrayInfo,
  data_type::Int32,
  error::{error_position, ErrorWithInput},
  record::RecordType,
};

/// 2.4.3.2 `ArraySingleObject`
#[derive(Debug, Clone, PartialEq)]
pub struct ArraySingleObject {
  pub array_info: ArrayInfo,
}

impl ArraySingleObject {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, ErrorWithInput<'_>> {
    let (input, _) = RecordType::ArraySingleObject
      .parse(input)
      .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedArraySingleObject)))?;

    let (input, array_info) = ArrayInfo::parse(input)?;

    Ok((input, Self { array_info }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.array_info.object_id()
  }
}
