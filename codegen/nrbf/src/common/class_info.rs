use nom::{multi::many_m_n, IResult, ToUsize};

use crate::data_type::{Int32, LengthPrefixedString};

/// 2.3.1.1 `ClassInfo`
#[derive(Debug, Clone, PartialEq)]
pub struct ClassInfo<'i> {
  pub object_id: Int32,
  pub name: LengthPrefixedString<'i>,
  pub member_names: Vec<LengthPrefixedString<'i>>,
}

impl<'i> ClassInfo<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, object_id) = Int32::parse_positive(input)?;
    let (input, name) = LengthPrefixedString::parse(input)?;
    let (input, member_count) = Int32::parse_positive_or_zero(input)?;

    let member_count = (i32::from(member_count) as u32).to_usize();

    let (input, member_names) = many_m_n(member_count, member_count, LengthPrefixedString::parse)(input)?;

    Ok((input, Self { object_id, name, member_names }))
  }
}
