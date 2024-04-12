use nom::{IResult, Parser};

use crate::{
  data_type::Int32,
  error::{error_position, ErrorWithInput},
  record::RecordType,
};

/// 2.5.3 `MemberReference`
#[derive(Debug, Clone, PartialEq)]
pub struct MemberReference {
  pub id_ref: Int32,
}

impl MemberReference {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, ErrorWithInput<'_>> {
    let (input, _) = RecordType::MemberReference
      .parse(input)
      .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedMemberReference)))?;

    let (input, id_ref) = Int32::parse_positive(input)?;

    Ok((input, Self { id_ref }))
  }
}
