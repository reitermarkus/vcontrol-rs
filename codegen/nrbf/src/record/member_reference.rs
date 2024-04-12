use std::num::NonZeroU32;

use nom::IResult;

use crate::{combinator::object_id, error::Error, record::RecordType};

/// 2.5.3 `MemberReference`
#[derive(Debug, Clone, PartialEq)]
pub struct MemberReference {
  pub id_ref: NonZeroU32,
}

impl MemberReference {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    let (input, _) = RecordType::MemberReference.parse(input)?;

    let (input, id_ref) = object_id(input)?;

    Ok((input, Self { id_ref }))
  }
}
