use nom::{branch::alt, combinator::map, IResult, Parser, ToUsize};

use super::{
  data_type::{Byte, Int32},
  RecordType,
};

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct ObjectNull;

impl ObjectNull {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = RecordType::ObjectNull.parse(input)?;

    Ok((input, Self))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectNullMultiple {
  pub null_count: Int32,
}

impl ObjectNullMultiple {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = RecordType::ObjectNullMultiple.parse(input)?;

    let (input, null_count) = Int32::parse_positive(input)?;

    Ok((input, Self { null_count }))
  }

  pub(crate) fn null_count(&self) -> usize {
    (i32::from(self.null_count) as u32).to_usize()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectNullMultiple256 {
  pub null_count: Byte,
}

impl ObjectNullMultiple256 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = RecordType::ObjectNullMultiple256.parse(input)?;

    let (input, null_count) = Byte::parse(input)?;

    Ok((input, Self { null_count }))
  }

  pub(crate) fn null_count(&self) -> usize {
    u8::from(self.null_count).to_usize()
  }
}

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
