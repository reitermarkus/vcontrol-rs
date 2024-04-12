use std::num::NonZeroU32;

use nom::IResult;

use crate::{combinator::object_id, data_type::LengthPrefixedString, error::Error, record::RecordType};

/// 2.5.7 `BinaryObjectString`
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryObjectString<'s> {
  pub object_id: NonZeroU32,
  pub value: LengthPrefixedString<'s>,
}

impl<'i> BinaryObjectString<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, Error<'i>> {
    let (input, _) = RecordType::BinaryObjectString.parse(input)?;

    let (input, object_id) = object_id(input)?;
    let (input, value) = LengthPrefixedString::parse(input)?;

    Ok((input, Self { object_id, value }))
  }

  pub fn as_str(&self) -> &'i str {
    self.value.as_str()
  }

  #[inline]
  pub(crate) fn object_id(&self) -> NonZeroU32 {
    self.object_id
  }
}
