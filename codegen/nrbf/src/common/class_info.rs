use std::num::NonZeroU32;

use nom::{multi::length_count, IResult};

use crate::{
  combinator::{length, object_id},
  data_type::LengthPrefixedString,
  error::Error,
};

/// 2.3.1.1 `ClassInfo`
#[derive(Debug, Clone, PartialEq)]
pub struct ClassInfo<'i> {
  pub object_id: NonZeroU32,
  pub name: LengthPrefixedString<'i>,
  pub member_names: Vec<LengthPrefixedString<'i>>,
}

impl<'i> ClassInfo<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, Error<'i>> {
    let (input, object_id) = object_id(input)?;
    let (input, name) = LengthPrefixedString::parse(input)?;
    let (input, member_names) = length_count(length, LengthPrefixedString::parse)(input)?;

    Ok((input, Self { object_id, name, member_names }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> NonZeroU32 {
    self.object_id
  }
}
