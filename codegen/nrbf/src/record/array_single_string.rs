use nom::{branch::alt, combinator::map, IResult, Parser};

use crate::{
  binary_parser::Object,
  common::ArrayInfo,
  data_type::Int32,
  grammar::{MemberReferenceInner, NullObject},
  record::{BinaryObjectString, MemberReference, RecordType},
};

/// 2.4.3.4 `ArraySingleString`
#[derive(Debug, Clone, PartialEq)]
pub struct ArraySingleString {
  pub array_info: ArrayInfo,
}

impl ArraySingleString {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = RecordType::ArraySingleString.parse(input)?;

    let (mut input, array_info) = ArrayInfo::parse(input)?;

    Ok((input, Self { array_info }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.array_info.object_id()
  }
}
