use nom::{IResult, Parser};

use crate::{data_type::Int32, record::RecordType};

#[derive(Debug, Clone, PartialEq)]
pub struct MemberReference {
  pub id_ref: Int32,
}

impl MemberReference {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = RecordType::MemberReference.parse(input)?;

    let (input, id_ref) = Int32::parse_positive(input)?;

    Ok((input, Self { id_ref }))
  }
}
