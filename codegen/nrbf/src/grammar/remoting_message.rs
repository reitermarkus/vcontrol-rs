use std::collections::BTreeMap;

use nom::{
  combinator::{map, opt},
  IResult,
};
#[cfg(feature = "serde")]
use serde::{
  de::{value::Error, Deserializer, Visitor},
  forward_to_deserialize_any,
};

use crate::{
  binary_parser::{Object, ObjectDeserializer},
  data_type::Int32,
  record::{BinaryMethodCall, BinaryMethodReturn, MessageEnd, SerializationHeader},
  BinaryParser,
};

#[derive(Debug, Clone, PartialEq)]
pub enum MethodCallOrReturn<'i> {
  MethodCall(BinaryMethodCall<'i>),
  MethodReturn(BinaryMethodReturn<'i>),
}

impl<'i> MethodCallOrReturn<'i> {
  pub fn parse(input: &'i [u8], parser: &mut BinaryParser<'i>) -> IResult<&'i [u8], Self> {
    if let Ok(s) = map(|input| parser.parse_method_call(input), Self::MethodCall)(input) {
      return Ok(s)
    }

    if let Ok(s) = map(|input| parser.parse_method_return(input), |mr| Self::MethodReturn(mr))(input) {
      return Ok(s)
    }

    Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Alt)))
  }
}

/// 2.7 Binary Record Grammar - `remotingMessage`
#[derive(Debug, Clone, PartialEq)]
pub struct RemotingMessage<'i> {
  pub header: SerializationHeader,
  pub objects: BTreeMap<Int32, Object<'i>>,
  pub method_call_or_return: Option<MethodCallOrReturn<'i>>,
  pub end: MessageEnd,
}

impl<'i> RemotingMessage<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let mut parser = BinaryParser::default();

    let (mut input, header) = SerializationHeader::parse(input)?;

    while let Ok((input2, _)) = parser.parse_referenceable(input) {
      input = input2;
    }

    let (mut input, method_call_or_return) = opt(|input| MethodCallOrReturn::parse(input, &mut parser))(input)?;

    while let Ok((input2, _)) = parser.parse_referenceable(input) {
      input = input2;
    }

    let (input, end) = MessageEnd::parse(input)?;

    Ok((input, Self { header, objects: parser.objects, method_call_or_return, end }))
  }
}

#[cfg(feature = "serde")]
impl<'de> Deserializer<'de> for RemotingMessage<'de> {
  type Error = Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    use serde::de::{Error, Unexpected};

    match self.method_call_or_return {
      Some(MethodCallOrReturn::MethodCall(_)) => {
        return Err(Error::invalid_type(Unexpected::Other("method call"), &visitor))
      },
      Some(MethodCallOrReturn::MethodReturn(_)) => {
        return Err(Error::invalid_type(Unexpected::Other("method return"), &visitor))
      },
      None => (),
    }

    let objects = self.objects;

    let root_object = objects
      .get(&self.header.root_id)
      .ok_or_else(|| Error::invalid_type(Unexpected::Other("no root object"), &visitor))?;

    ObjectDeserializer::new(&objects, &root_object).deserialize_any(visitor)
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map struct enum identifier ignored_any
  }
}
