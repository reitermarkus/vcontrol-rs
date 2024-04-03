use nom::{
  combinator::{map, map_res},
  multi::many_m_n,
  IResult,
};

use crate::{data_type::Int32, record::ValueWithCode, Value};

/// 2.2.2.3 `ArrayOfValueWithCode`
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayOfValueWithCode<'i>(Vec<ValueWithCode<'i>>);

impl<'i> ArrayOfValueWithCode<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, length) = map_res(Int32::parse, usize::try_from)(input)?;
    map(many_m_n(length, length, ValueWithCode::parse), Self)(input)
  }

  #[inline]
  pub(crate) fn into_values(self) -> Vec<Value<'i>> {
    self.0.into_iter().map(|v| v.into_value()).collect()
  }
}
