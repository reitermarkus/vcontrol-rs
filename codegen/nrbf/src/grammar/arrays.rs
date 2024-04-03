use nom::{
  branch::alt,
  combinator::{map, opt},
  IResult,
};

use crate::{
  data_type::Int32,
  record::{ArraySingleObject, ArraySinglePrimitive, ArraySingleString, BinaryArray, BinaryLibrary},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Array<'i> {
  ArraySingleObject(ArraySingleObject<'i>),
  ArraySinglePrimitive(ArraySinglePrimitive),
  ArraySingleString(ArraySingleString<'i>),
  BinaryArray(BinaryArray<'i>),
}

impl<'i> Array<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    alt((
      map(ArraySingleObject::parse, Self::ArraySingleObject),
      map(ArraySinglePrimitive::parse, Self::ArraySinglePrimitive),
      map(ArraySingleString::parse, Self::ArraySingleString),
      map(BinaryArray::parse, Self::BinaryArray),
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
  pub binary_library: Option<BinaryLibrary<'i>>,
  pub array: Array<'i>,
}

impl<'i> Arrays<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;
    let (input, array) = Array::parse(input)?;

    Ok((input, Self { binary_library, array }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.array.object_id()
  }
}
