use nom::{multi::count, IResult};

use crate::{
  common::{AdditionalTypeInfo, ClassInfo},
  enumeration::BinaryType,
  error::Error,
};

/// 2.3.1.2 `MemberTypeInfo`
#[derive(Debug, Clone, PartialEq)]
pub struct MemberTypeInfo<'i> {
  pub binary_type_enums: Vec<BinaryType>,
  pub additional_infos: Vec<Option<AdditionalTypeInfo<'i>>>,
}

impl<'i> MemberTypeInfo<'i> {
  pub fn parse(input: &'i [u8], class_info: &ClassInfo<'_>) -> IResult<&'i [u8], Self, Error<'i>> {
    let (mut input, binary_type_enums) = count(BinaryType::parse, class_info.member_names.len())(input)?;

    let mut additional_infos = vec![];
    for &binary_type_enum in binary_type_enums.iter() {
      let additional_info;
      (input, additional_info) = AdditionalTypeInfo::parse(input, binary_type_enum)?;
      additional_infos.push(additional_info);
    }

    Ok((input, Self { binary_type_enums, additional_infos }))
  }
}
