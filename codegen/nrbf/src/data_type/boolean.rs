use nom::{combinator::map_res, number::complete::u8, IResult};

/// 2.1.1 `BOOLEAN`
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
