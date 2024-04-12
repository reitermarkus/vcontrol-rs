use nom::{IResult, Parser};

use crate::{
  combinator::into_failure,
  common::ArrayInfo,
  data_type::Int32,
  error::{error_position, ErrorWithInput},
  record::RecordType,
};

/// 2.4.3.4 `ArraySingleString`
#[derive(Debug, Clone, PartialEq)]
pub struct ArraySingleString {
  pub array_info: ArrayInfo,
}

impl ArraySingleString {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, ErrorWithInput<'_>> {
    let (input, _) = RecordType::ArraySingleString
      .parse(input)
      .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedArraySingleString)))?;

    let (input, array_info) = ArrayInfo::parse(input)
      .map_err(into_failure)
      .map_err(|err| err.map(|err| error_position!(err.input, ExpectedArrayInfo)))?;

    Ok((input, Self { array_info }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.array_info.object_id()
  }
}
