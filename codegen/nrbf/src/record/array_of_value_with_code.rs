use nom::{combinator::map, multi::length_count, IResult};

use crate::{combinator::length, error::Error, record::ValueWithCode, Value};

/// 2.2.2.3 `ArrayOfValueWithCode`
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayOfValueWithCode<'i>(Vec<ValueWithCode<'i>>);

impl<'i> ArrayOfValueWithCode<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, Error<'i>> {
    map(length_count(length, ValueWithCode::parse), Self)(input)
  }

  #[inline]
  pub(crate) fn into_values(self) -> Vec<Value<'i>> {
    self.0.into_iter().map(|v| v.into_value()).collect()
  }
}
