use nom::{
  branch::alt,
  combinator::{map, map_opt},
  number::complete::{le_u16, le_u24, le_u32, u8},
  IResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Serializer};

/// 2.1.1.1 `Char`
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Char(pub char);

impl Char {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(
      alt((
        map_opt(u8, |n| char::from_u32(n as u32)),
        map_opt(le_u16, |n| char::from_u32(n as u32)),
        map_opt(le_u24, char::from_u32),
        map_opt(le_u32, char::from_u32),
      )),
      Self,
    )(input)
  }
}

impl From<char> for Char {
  #[inline]
  fn from(v: char) -> Self {
    Self(v)
  }
}

impl From<Char> for char {
  #[inline]
  fn from(val: Char) -> Self {
    val.0
  }
}

#[cfg(feature = "serde")]
impl Serialize for Char {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_char(self.0)
  }
}