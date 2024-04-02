use nom::{
  branch::alt,
  combinator::{map},
  IResult,
};

use crate::{
  data_type::Int32,
  record::{ArraySingleObject, ArraySinglePrimitive, ArraySingleString, BinaryArray},
  BinaryParser,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Array<'i> {
  ArraySingleObject(ArraySingleObject<'i>),
  ArraySinglePrimitive(ArraySinglePrimitive),
  ArraySingleString(ArraySingleString<'i>),
  BinaryArray(BinaryArray<'i>),
}

impl<'i> Array<'i> {
  pub fn parse(input: &'i [u8], parser: &mut BinaryParser<'i>) -> IResult<&'i [u8], Self> {
    if let Ok(s) = map(|input| ArraySingleObject::parse(input, parser), Self::ArraySingleObject)(input) {
      return Ok(s)
    }

    alt((
      map(ArraySinglePrimitive::parse, Self::ArraySinglePrimitive),
      map(ArraySingleString::parse, Self::ArraySingleString),
      map(|input| BinaryArray::parse(input, parser), Self::BinaryArray),
    ))(input)
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    match self {
      Self::ArraySingleObject(array) => array.object_id(),
      Self::ArraySinglePrimitive(array) => array.object_id(),
      Self::ArraySingleString(array) => array.object_id(),
      Self::BinaryArray(array) => array.object_id(),
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

    let (input, array) = Array::parse(input, parser)?;

    Ok((input, Self { array }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.array.object_id()
  }
}
