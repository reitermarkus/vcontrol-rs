use nom::{
  multi::count,
  combinator::{map},
  IResult, ToUsize,
};

use crate::{
  data_type::Int32,
  error::{ErrorWithInput},
  record::ValueWithCode,
  Value,
};

/// 2.2.2.3 `ArrayOfValueWithCode`
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayOfValueWithCode<'i>(Vec<ValueWithCode<'i>>);

impl<'i> ArrayOfValueWithCode<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, ErrorWithInput<'i>> {
    let (input, len) = Int32::parse_positive_or_zero(input)?;

    map(count(ValueWithCode::parse, (i32::from(len) as u32).to_usize()), Self)(input)
  }

  #[inline]
  pub(crate) fn into_values(self) -> Vec<Value<'i>> {
    self.0.into_iter().map(|v| v.into_value()).collect()
  }
}
