use nom::{IResult, Parser};

use crate::{
  data_type::Int32,
  error::{error_position, ErrorWithInput},
  record::RecordType,
};

/// 2.6.1 `SerializationHeaderRecord`
#[derive(Debug, Clone, PartialEq)]
pub struct SerializationHeader {
  pub root_id: Int32,
  pub header_id: Int32,
  pub major_version: Int32,
  pub minor_version: Int32,
}

impl SerializationHeader {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, ErrorWithInput<'_>> {
    let (input, _) = RecordType::SerializedStreamHeader
      .parse(input)
      .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedHeader)))?;

    let (input, root_id) = Int32::parse(input)?;
    let (input, header_id) = Int32::parse(input)?;
    let (input, major_version) = Int32::parse(input)?;
    let (input, minor_version) = Int32::parse(input)?;

    Ok((input, Self { root_id, header_id, major_version, minor_version }))
  }
}
