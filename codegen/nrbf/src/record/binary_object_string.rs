use nom::{IResult, Parser};

use crate::{
  data_type::{Int32, LengthPrefixedString},
  record::RecordType,
};

/// 2.5.7 `BinaryObjectString`
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryObjectString<'s> {
  pub object_id: Int32,
  pub value: LengthPrefixedString<'s>,
}

impl<'i> BinaryObjectString<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = RecordType::BinaryObjectString.parse(input)?;

    let (input, object_id) = Int32::parse_positive(input)?;
    let (input, value) = LengthPrefixedString::parse(input)?;

    Ok((input, Self { object_id, value }))
  }
}
