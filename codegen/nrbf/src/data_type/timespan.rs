use nom::{combinator::map, number::complete::le_i64, IResult};
#[cfg(feature = "serde")]
use serde::{ser::SerializeTupleStruct, Deserialize, Serialize, Serializer};

/// 2.1.1.4 `TimeSpan`
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeSpan(pub i64);

impl TimeSpan {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_i64, Self)(input)
  }
}

impl From<i64> for TimeSpan {
  #[inline]
  fn from(v: i64) -> Self {
    Self(v)
  }
}

impl From<TimeSpan> for i64 {
  #[inline]
  fn from(val: TimeSpan) -> Self {
    val.0
  }
}

#[cfg(feature = "serde")]
impl Serialize for TimeSpan {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut ts = serializer.serialize_tuple_struct("TimeSpan", 1)?;
    ts.serialize_field(&self.0)?;
    ts.end()
  }
}
