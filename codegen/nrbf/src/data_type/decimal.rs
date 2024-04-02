use std::str::FromStr;

use nom::{combinator::map_res, IResult};

use crate::data_type::LengthPrefixedString;

/// 2.1.1.7 `Decimal`
#[derive(Debug, Clone, PartialEq)]
pub struct Decimal(pub rust_decimal::Decimal);

impl Decimal {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map_res(LengthPrefixedString::parse, |s| rust_decimal::Decimal::from_str(s.as_str()).map(Self))(input)
  }
}
