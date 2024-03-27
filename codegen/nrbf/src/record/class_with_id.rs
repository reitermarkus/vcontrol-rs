use nom::{IResult, Parser};

use crate::{data_type::Int32, record::RecordType};

/// 2.3.2.5 `ClassWithId`
#[derive(Debug, Clone, PartialEq)]
pub struct ClassWithId {
  pub object_id: Int32,
  pub metadata_id: Int32,
}

impl ClassWithId {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = RecordType::ClassWithId.parse(input)?;

    let (input, object_id) = Int32::parse(input)?;
    let (input, metadata_id) = Int32::parse(input)?;

    Ok((input, Self { object_id, metadata_id }))
  }
}
