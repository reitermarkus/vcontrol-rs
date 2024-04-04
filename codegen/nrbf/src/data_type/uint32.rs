use nom::{combinator::map, number::complete::le_u32, IResult};

use super::impl_primitive;

/// 2.1.1 `UINT32`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UInt32(pub u32);

impl UInt32 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_u32, Self)(input)
  }
}

impl_primitive!(UInt32, u32, visit_u32, deserialize_u32);
