use nom::{branch::alt, combinator::map, IResult};

use crate::{
  data_type::Int32,
  grammar::{Arrays, Classes},
  record::BinaryObjectString,
};

/// 2.7 Binary Record Grammar - `referenceable`
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

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    match self {
      Self::Classes(classes) => classes.object_id(),
      Self::Arrays(arrays) => arrays.object_id(),
      Self::BinaryObjectString(s) => s.object_id(),
    }
  }
}
