use nom::{branch::alt, combinator::map, IResult};

use crate::{
  binary_parser::Object,
  data_type::Int32,
  record::{ArraySingleObject, ArraySinglePrimitive, ArraySingleString, BinaryArray},
  BinaryParser,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Array<'i> {
  ArraySingleObject(Int32, Vec<Object<'i>>),
  ArraySinglePrimitive(Int32, Vec<Object<'i>>),
  ArraySingleString(Int32, Vec<Object<'i>>),
  BinaryArray(Int32, Vec<Object<'i>>),
}

impl<'i> Array<'i> {
  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    match self {
      Self::ArraySingleObject(id, _) => *id,
      Self::ArraySinglePrimitive(id, _) => *id,
      Self::ArraySingleString(id, _) => *id,
      Self::BinaryArray(id, _) => *id,
    }
  }
}

/// 2.7 Binary Record Grammar - `Arrays`
#[derive(Debug, Clone, PartialEq)]
pub struct Arrays<'i> {
  pub array: Array<'i>,
}

impl<'i> Arrays<'i> {
  pub fn parse(input: &'i [u8], parser: &mut BinaryParser<'i>) -> IResult<&'i [u8], Self> {
    let (input, ()) = parser.parse_binary_library(input)?;

    let (input, array) = parser.parse_array(input)?;

    Ok((input, Self { array }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.array.object_id()
  }
}
