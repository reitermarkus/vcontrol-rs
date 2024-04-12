use nom::{IResult, Parser};

use crate::{
  combinator::into_failure,
  common::ArrayInfo,
  data_type::Int32,
  enumeration::PrimitiveType,
  error::{error_position, ErrorWithInput},
  record::RecordType,
};

/// 2.4.3.3 `ArraySinglePrimitive`
#[derive(Debug, Clone, PartialEq)]
pub struct ArraySinglePrimitive {
  pub array_info: ArrayInfo,
  pub primitive_type: PrimitiveType,
}

impl ArraySinglePrimitive {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, ErrorWithInput<'_>> {
    let (input, _) = RecordType::ArraySinglePrimitive.parse(input).map_err(|err| {
      err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedArraySinglePrimitive))
    })?;

    let (input, array_info) = ArrayInfo::parse(input)
      .map_err(into_failure)
      .map_err(|err| err.map(|err| error_position!(err.input, ExpectedArrayInfo)))?;
    let (input, primitive_type) = PrimitiveType::parse(input)
      .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedPrimitiveType)))?;

    Ok((input, Self { array_info, primitive_type }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.array_info.object_id()
  }
}
