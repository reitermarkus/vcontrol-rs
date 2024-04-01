use nom::{IResult, Parser};

use crate::data_type::{Int32, LengthPrefixedString};

use super::RecordType;

/// 2.6.2 `BinaryLibrary`
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryLibrary<'i> {
  pub library_id: Int32,
  pub library_name: LengthPrefixedString<'i>,
}

impl<'i> BinaryLibrary<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = RecordType::BinaryLibrary.parse(input)?;

    let (input, library_id) = Int32::parse_positive(input)?;
    let (input, library_name) = LengthPrefixedString::parse(input)?;

    Ok((input, Self { library_id, library_name }))
  }

  #[inline]
  pub(crate) fn library_id(&self) -> Int32 {
    self.library_id
  }
}
