use nom::{multi::count, IResult, ToUsize};

use crate::{

  data_type::{Int32, LengthPrefixedString},
  error::ErrorWithInput,
};

/// 2.3.1.1 `ClassInfo`
#[derive(Debug, Clone, PartialEq)]
pub struct ClassInfo<'i> {
  pub object_id: Int32,
  pub name: LengthPrefixedString<'i>,
  pub member_names: Vec<LengthPrefixedString<'i>>,
}

impl<'i> ClassInfo<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, ErrorWithInput<'i>> {
    let (input, object_id) = Int32::parse_positive(input)?;
    let (input, name) = LengthPrefixedString::parse(input)?;
    let (input, member_count) = Int32::parse_positive_or_zero(input)?;

    let (input, member_names) = count(LengthPrefixedString::parse, (i32::from(member_count) as u32).to_usize())(input)?;

    Ok((input, Self { object_id, name, member_names }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.object_id
  }
}
