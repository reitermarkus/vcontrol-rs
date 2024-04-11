use nom::{combinator::map_res, number::complete::u8, IResult};

use super::impl_primitive;
use crate::{combinator::into_failure};

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
    .map_err(into_failure)
  }
}

impl_primitive!(Boolean, bool, visit_bool, deserialize_bool);
