use nom::{branch::alt, combinator::map, IResult};

use crate::{
  data_type::Int32,
  grammar::{Arrays, Classes},
  record::BinaryObjectString,
  BinaryParser,
};

/// 2.7 Binary Record Grammar - `referenceable`
#[derive(Debug, Clone, PartialEq)]
pub enum Referenceable<'i> {
  Classes(Classes<'i>),
  Arrays(Arrays<'i>),
  BinaryObjectString(BinaryObjectString<'i>),
}

impl<'i> Referenceable<'i> {
  pub fn parse(input: &'i [u8], parser: &mut BinaryParser<'i>) -> IResult<&'i [u8], Self> {
    if let Ok(s) = map(|input| Classes::parse(input, parser), Self::Classes)(input) {
      return Ok(s)
    }

    alt((
      map(|input| Arrays::parse(input, parser), Self::Arrays),
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
