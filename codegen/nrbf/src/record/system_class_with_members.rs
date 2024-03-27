use nom::{branch::alt, combinator::map, IResult, Parser, ToUsize};

use crate::{
  data_type::{Byte, Int32},
  record::RecordType,
  ClassInfo,
};

/// 2.3.2.4 `SystemClassWithMembers`
#[derive(Debug, Clone, PartialEq)]
pub struct SystemClassWithMembers<'i> {
  pub class_info: ClassInfo<'i>,
}

impl<'i> SystemClassWithMembers<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = RecordType::SystemClassWithMembers.parse(input)?;

    let (input, class_info) = ClassInfo::parse(input)?;

    Ok((input, Self { class_info }))
  }
}
