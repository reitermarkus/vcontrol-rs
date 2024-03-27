use nom::{bytes::complete::tag, IResult, Parser};

use crate::data_type::Int32;

/// 2.6.1 `SerializationHeaderRecord`
#[derive(Debug, Clone, PartialEq)]
pub struct SerializationHeader {
  pub root_id: Int32,
  pub header_id: Int32,
  pub major_version: Int32,
  pub minor_version: Int32,
}

impl SerializationHeader {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = tag([0])(input)?;

    let (input, root_id) = Int32::parse(input)?;
    let (input, header_id) = Int32::parse(input)?;
    let (input, major_version) = Int32::parse(input)?;
    let (input, minor_version) = Int32::parse(input)?;

    Ok((input, Self { root_id, header_id, major_version, minor_version }))
  }
}
