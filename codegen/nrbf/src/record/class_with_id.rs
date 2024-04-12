use nom::{combinator::verify, IResult, Parser};

use crate::{
  combinator::into_failure,
  data_type::Int32,
  error::{error_position, ErrorWithInput},
  record::RecordType,
};

/// 2.3.2.5 `ClassWithId`
#[derive(Debug, Clone, PartialEq)]
pub struct ClassWithId {
  pub object_id: Int32,
  pub metadata_id: Int32,
}

impl ClassWithId {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, ErrorWithInput<'_>> {
    let (input, _) = RecordType::ClassWithId
      .parse(input)
      .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedClassWithId)))?;

    let (input, object_id) = Int32::parse_positive(input)?;
    let (input, metadata_id) =
      verify(Int32::parse_positive, |&metadata_id| metadata_id != object_id)(input).map_err(into_failure)?;

    Ok((input, Self { object_id, metadata_id }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.object_id
  }

  #[inline]
  pub(crate) fn metadata_id(&self) -> Int32 {
    self.metadata_id
  }
}
