use nom::{
  combinator::{map, map_res},
  multi::many_m_n,
  IResult,
};

use crate::{combinator::into_failure, data_type::Int32, record::ValueWithCode, Value};

/// 2.2.2.3 `ArrayOfValueWithCode`
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayOfValueWithCode<'i>(Vec<ValueWithCode<'i>>);

impl<'i> ArrayOfValueWithCode<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    map_res(Int32::parse, usize::try_from)(input)
      .and_then(|(input, len)| map(many_m_n(len, len, ValueWithCode::parse), Self)(input))
      .map_err(into_failure)
  }

  #[inline]
  pub(crate) fn into_values(self) -> Vec<Value<'i>> {
    self.0.into_iter().map(|v| v.into_value()).collect()
  }
}