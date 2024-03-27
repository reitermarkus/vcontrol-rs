use nom::{branch::alt, combinator::map, IResult, Parser, ToUsize};

use crate::{
  data_type::{Byte, Int32},
  record::{ObjectNull, ObjectNullMultiple, ObjectNullMultiple256, RecordType},
};

#[derive(Debug, Clone, PartialEq)]
pub enum NullObject {
  ObjectNull(ObjectNull),
  ObjectNullMultiple(ObjectNullMultiple),
  ObjectNullMultiple256(ObjectNullMultiple256),
}

impl NullObject {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    alt((
      map(ObjectNull::parse, Self::ObjectNull),
      map(ObjectNullMultiple::parse, Self::ObjectNullMultiple),
      map(ObjectNullMultiple256::parse, Self::ObjectNullMultiple256),
    ))(input)
  }
}
