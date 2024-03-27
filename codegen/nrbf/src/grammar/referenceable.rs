use nom::{branch::alt, combinator::map, IResult};

use crate::{
  grammar::{Arrays, Classes},
  record::BinaryObjectString,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Referenceable<'i> {
  Classes(Classes<'i>),
  Arrays(Arrays<'i>),
  BinaryObjectString(BinaryObjectString<'i>),
}

impl<'i> Referenceable<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    alt((
      map(Classes::parse, Self::Classes),
      map(Arrays::parse, Self::Arrays),
      map(BinaryObjectString::parse, Self::BinaryObjectString),
    ))(input)
  }
}
