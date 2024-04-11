use nom::{combinator::map, number::complete::le_u16, IResult};

use super::impl_primitive;
use crate::combinator::into_failure;

/// 2.1.1 `UINT16`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UInt16(pub u16);

impl UInt16 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_u16, Self)(input).map_err(into_failure)
  }
}

impl_primitive!(UInt16, u16, visit_u16, deserialize_u16);
