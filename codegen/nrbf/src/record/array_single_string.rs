use nom::{branch::alt, combinator::map, IResult, Parser};

use crate::{
  common::ArrayInfo,
  grammar::{MemberReferenceInner, NullObject},
  record::{BinaryObjectString, MemberReference, RecordType},
};

/// 2.4.3.4 `ArraySingleString`
#[derive(Debug, Clone, PartialEq)]
pub struct ArraySingleString<'i> {
  pub array_info: ArrayInfo,
  pub members: Vec<MemberReferenceInner<'i>>,
}

impl<'i> ArraySingleString<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = RecordType::ArraySingleString.parse(input)?;

    let (mut input, array_info) = ArrayInfo::parse(input)?;

    let mut members = vec![];

    let mut len_remaining = array_info.len();
    while len_remaining > 0 {
      let (member, count);
      (input, (member, count)) = alt((
        map(BinaryObjectString::parse, |s| (MemberReferenceInner::BinaryObjectString(s), 1)),
        map(MemberReference::parse, |m| (MemberReferenceInner::MemberReference(m), 1)),
        map(NullObject::parse, |null_object| {
          let null_count = match null_object {
            NullObject::ObjectNull(_) => 1,
            NullObject::ObjectNullMultiple(ref n) => n.null_count(),
            NullObject::ObjectNullMultiple256(ref n) => n.null_count(),
          };

          (MemberReferenceInner::NullObject(null_object), null_count)
        }),
      ))(input)?;
      members.push(member);
      len_remaining -= count;
    }

    Ok((input, Self { array_info, members }))
  }
}
