use nom::{combinator::map, sequence::preceded, IResult};

use crate::{combinator::into_failure, data_type::LengthPrefixedString, enumeration::PrimitiveType};

/// 2.2.2.2 `StringValueWithCode`
#[derive(Debug, Clone, PartialEq)]
pub struct StringValueWithCode<'i>(LengthPrefixedString<'i>);

impl<'i> StringValueWithCode<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    map(preceded(PrimitiveType::String, LengthPrefixedString::parse), Self)(input).map_err(into_failure)
  }

  #[inline]
  pub fn as_str(&self) -> &'i str {
    self.0.as_str()
  }
}

impl<'s> From<LengthPrefixedString<'s>> for StringValueWithCode<'s> {
  fn from(s: LengthPrefixedString<'s>) -> Self {
    Self(s)
  }
}
