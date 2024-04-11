use nom::{IResult, Parser};

use crate::{
  common::ArrayInfo, data_type::Int32, enumeration::PrimitiveType, record::RecordType,
};

/// 2.4.3.3 `ArraySinglePrimitive`
#[derive(Debug, Clone, PartialEq)]
pub struct ArraySinglePrimitive {
  pub array_info: ArrayInfo,
  pub primitive_type: PrimitiveType,
}

impl ArraySinglePrimitive {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = RecordType::ArraySinglePrimitive.parse(input)?;

    let (input, array_info) = ArrayInfo::parse(input)?;
    let (input, primitive_type) = PrimitiveType::parse(input)?;

    Ok((input, Self { array_info, primitive_type }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.array_info.object_id()
  }
}
