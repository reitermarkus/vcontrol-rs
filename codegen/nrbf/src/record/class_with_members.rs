use nom::{branch::alt, combinator::map, IResult, Parser, ToUsize};

use crate::{
  data_type::{Byte, Int32},
  record::RecordType,
  ClassInfo,
};

/// 2.3.2.2 `ClassWithMembers`
#[derive(Debug, Clone, PartialEq)]
pub struct ClassWithMembers<'i> {
  pub class_info: ClassInfo<'i>,
  pub library_id: Int32,
}

impl<'i> ClassWithMembers<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = RecordType::ClassWithMembers.parse(input)?;

    let (input, class_info) = ClassInfo::parse(input)?;
    let (input, library_id) = Int32::parse_positive(input)?;

    Ok((input, Self { class_info, library_id }))
  }
}
