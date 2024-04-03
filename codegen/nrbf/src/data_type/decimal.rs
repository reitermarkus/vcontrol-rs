use std::str::FromStr;

use nom::{combinator::map_res, IResult};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Serializer};

use crate::data_type::LengthPrefixedString;

/// 2.1.1.7 `Decimal`
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Decimal(pub rust_decimal::Decimal);

impl Decimal {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map_res(LengthPrefixedString::parse, |s| rust_decimal::Decimal::from_str(s.as_str()).map(Self))(input)
  }
}

#[cfg(feature = "serde")]
impl Serialize for Decimal {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    Serialize::serialize(&self.0, serializer)
  }
}
