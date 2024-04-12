use nom::{IResult, Parser};

use crate::{
  data_type::{Int32, LengthPrefixedString},
  error::{error_position, ErrorWithInput},
  record::RecordType,
};

/// 2.5.7 `BinaryObjectString`
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryObjectString<'s> {
  pub object_id: Int32,
  pub value: LengthPrefixedString<'s>,
}

impl<'i> BinaryObjectString<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, ErrorWithInput<'i>> {
    let (input, _) = RecordType::BinaryObjectString
      .parse(input)
      .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedBinaryObjectString)))?;

    let (input, object_id) =
      Int32::parse_positive(input).map_err(|err| err.map(|err| error_position!(err.input, ExpectedInt32)))?;
    let (input, value) = LengthPrefixedString::parse(input)
      .map_err(|err| err.map(|err| error_position!(err.input, ExpectedLengthPrefixedString)))?;

    Ok((input, Self { object_id, value }))
  }

  pub fn as_str(&self) -> &'i str {
    self.value.as_str()
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.object_id
  }
}
