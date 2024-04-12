use nom::{IResult, Parser};

use crate::{
  data_type::{Int32, LengthPrefixedString},
  error::{error_position, ErrorWithInput},
};

use super::RecordType;

/// 2.6.2 `BinaryLibrary`
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryLibrary<'i> {
  pub library_id: Int32,
  pub library_name: LengthPrefixedString<'i>,
}

impl<'i> BinaryLibrary<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, ErrorWithInput<'_>> {
    let (input, _) = RecordType::BinaryLibrary
      .parse(input)
      .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedBinaryLibrary)))?;

    let (input, library_id) =
      Int32::parse_positive(input).map_err(|err| err.map(|err| error_position!(err.input, ExpectedInt32)))?;
    let (input, library_name) = LengthPrefixedString::parse(input)
      .map_err(|err| err.map(|err| error_position!(err.input, ExpectedLengthPrefixedString)))?;

    Ok((input, Self { library_id, library_name }))
  }

  #[inline]
  pub(crate) fn library_id(&self) -> Int32 {
    self.library_id
  }
}
