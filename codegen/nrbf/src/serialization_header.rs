use nom::{bytes::complete::tag, number::complete::le_i32, IResult};

/// 2.6.1 `SerializationHeaderRecord`
#[derive(Debug, Clone, PartialEq)]
pub struct SerializationHeader {
  pub root_id: i32,
  pub header_id: i32,
  pub major_version: i32,
  pub minor_version: i32,
}

impl SerializationHeader {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = tag([0])(input)?;

    let (input, root_id) = le_i32(input)?;
    let (input, header_id) = le_i32(input)?;
    let (input, major_version) = le_i32(input)?;
    let (input, minor_version) = le_i32(input)?;

    Ok((input, Self { root_id, header_id, major_version, minor_version }))
  }
}
