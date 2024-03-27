use nom::{multi::many_m_n, IResult, Parser};

use crate::{
  common::ArrayInfo,
  enumeration::PrimitiveType,
  record::{MemberPrimitiveUnTyped, RecordType},
};

/// 2.4.3.3 `ArraySinglePrimitive`
#[derive(Debug, Clone, PartialEq)]
pub struct ArraySinglePrimitive {
  pub array_info: ArrayInfo,
  pub members: Vec<MemberPrimitiveUnTyped>,
}

impl ArraySinglePrimitive {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = RecordType::ArraySinglePrimitive.parse(input)?;

    let (input, array_info) = ArrayInfo::parse(input)?;
    let (input, primitive_type) = PrimitiveType::parse(input)?;
    let length = array_info.len();
    let (input, members) =
      many_m_n(length, length, |input| MemberPrimitiveUnTyped::parse(input, primitive_type))(input)?;

    Ok((input, Self { array_info, members }))
  }
}
