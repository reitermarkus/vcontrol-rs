use nom::{multi::many_m_n, IResult};

use crate::{common::ClassInfo, enumeration::BinaryType, AdditionalTypeInfo};

#[derive(Debug, Clone, PartialEq)]
pub struct MemberTypeInfo<'i> {
  pub binary_type_enums: Vec<BinaryType>,
  pub additional_infos: Vec<Option<AdditionalTypeInfo<'i>>>,
}

impl<'i> MemberTypeInfo<'i> {
  pub fn parse(input: &'i [u8], class_info: &ClassInfo<'_>) -> IResult<&'i [u8], Self> {
    let count = class_info.member_names.len();

    let (input, binary_type_enums) = many_m_n(count, count, BinaryType::parse)(input)?;
    let (input, additional_infos) = AdditionalTypeInfo::parse_many(input, &binary_type_enums)?;

    Ok((input, Self { binary_type_enums, additional_infos }))
  }
}
