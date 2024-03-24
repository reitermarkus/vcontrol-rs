use nom::{bytes::complete::tag, number::complete::le_i32, IResult};

use super::LengthPrefixedString;

/// 2.6.2 `BinaryLibrary`
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryLibrary<'i> {
  pub library_id: i32,
  pub library_name: LengthPrefixedString<'i>,
}

impl<'i> BinaryLibrary<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = tag([12])(input)?;

    let (input, library_id) = le_i32(input)?;
    let (input, library_name) = LengthPrefixedString::parse(input)?;

    Ok((input, Self { library_id, library_name }))
  }
}
