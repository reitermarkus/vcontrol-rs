use nom::{
  branch::alt,
  combinator::{cond, fail, map},
  multi::many_m_n,
  IResult, Parser, ToUsize,
};

use crate::{
  data_type::{Byte, Int32},
  record::{BinaryObjectString, MemberReference, RecordType},
  AdditionalTypeInfo, ArrayInfo, BinaryArrayType, BinaryType, ClassInfo, MemberPrimitiveUnTyped, MemberReference2,
  MemberReference3, MemberTypeInfo, PrimitiveType,
};

/// 2.4.3.2 `ArraySingleObject`
#[derive(Debug, Clone, PartialEq)]
pub struct ArraySingleObject<'i> {
  pub array_info: ArrayInfo,
  pub member_references: Vec<MemberReference2<'i>>,
}

impl<'i> ArraySingleObject<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = RecordType::ArraySingleObject.parse(input)?;

    let (input, array_info) = ArrayInfo::parse(input)?;
    let length = array_info.len();
    let (input, member_references) = many_m_n(length, length, MemberReference2::parse)(input)?;

    Ok((input, Self { array_info, member_references }))
  }
}
