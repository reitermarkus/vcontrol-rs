use nom::{combinator::map_res, number::complete::u8, IResult};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Serializer};

/// 2.1.1 `BOOLEAN`
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Boolean(pub bool);

impl Boolean {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map_res(u8, |byte| {
      Ok(Self(match byte {
        0 => false,
        1 => true,
        _ => return Err(()),
      }))
    })(input)
  }
}

impl From<bool> for Boolean {
  #[inline]
  fn from(v: bool) -> Self {
    Self(v)
  }
}

impl From<Boolean> for bool {
  #[inline]
  fn from(val: Boolean) -> Self {
    val.0
  }
}

#[cfg(feature = "serde")]
impl Serialize for Boolean {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_bool(self.0)
  }
}
