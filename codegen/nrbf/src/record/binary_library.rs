use std::num::NonZeroU32;

use nom::IResult;

use crate::{combinator::library_id, data_type::LengthPrefixedString, error::Error};

use super::RecordType;

/// 2.6.2 `BinaryLibrary`
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryLibrary<'i> {
  pub library_id: NonZeroU32,
  pub library_name: LengthPrefixedString<'i>,
}

impl<'i> BinaryLibrary<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, Error<'_>> {
    let (input, _) = RecordType::BinaryLibrary.parse(input)?;

    let (input, library_id) = library_id(input)?;
    let (input, library_name) = LengthPrefixedString::parse(input)?;

    Ok((input, Self { library_id, library_name }))
  }

  #[inline]
  pub(crate) fn library_id(&self) -> NonZeroU32 {
    self.library_id
  }
}
