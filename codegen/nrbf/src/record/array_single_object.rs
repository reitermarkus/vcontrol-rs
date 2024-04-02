use nom::{IResult, Parser};

use crate::{common::ArrayInfo, data_type::Int32, record::RecordType};

/// 2.4.3.2 `ArraySingleObject`
#[derive(Debug, Clone, PartialEq)]
pub struct ArraySingleObject {
  pub array_info: ArrayInfo,
}

impl ArraySingleObject {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = RecordType::ArraySingleObject.parse(input)?;

    let (input, array_info) = ArrayInfo::parse(input)?;

    Ok((input, Self { array_info }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.array_info.object_id()
  }
}
