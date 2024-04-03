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
  data_type::Int32,
  record::{MessageEnd, SerializationHeader},
  value::ValueDeserializer,
  BinaryParser, Value,
};

#[derive(Debug, Clone, PartialEq)]
pub struct MethodCall<'i> {
  pub method_name: &'i str,
  pub type_name: &'i str,
  pub call_context: Option<&'i str>,
  pub args: Option<Vec<Value<'i>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodReturn<'i> {
  pub return_value: Option<Value<'i>>,
  pub call_context: Option<&'i str>,
  pub args: Option<Vec<Value<'i>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MethodCallOrReturn<'i> {
  MethodCall(MethodCall<'i>),
  MethodReturn(MethodReturn<'i>),
}

impl<'i> MethodCallOrReturn<'i> {
  pub fn parse(input: &'i [u8], parser: &mut BinaryParser<'i>) -> IResult<&'i [u8], Self> {
    if let Ok(s) = map(|input| parser.parse_method_call(input), Self::MethodCall)(input) {
      return Ok(s)
    }

    if let Ok(s) = map(|input| parser.parse_method_return(input), Self::MethodReturn)(input) {
      return Ok(s)
    }

    Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Alt)))
  }
}

/// 2.7 Binary Record Grammar - `remotingMessage`
#[derive(Debug, Clone, PartialEq)]
pub struct RemotingMessage<'i> {
  pub root_object: Value<'i>,
  pub objects: BTreeMap<Int32, Value<'i>>,
  pub method_call_or_return: Option<MethodCallOrReturn<'i>>,
}

impl<'i> RemotingMessage<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let mut parser = BinaryParser::default();

    let (mut input, header) = SerializationHeader::parse(input)?;

    let root_object = Value::Ref(header.root_id);

    while let Ok((input2, _)) = parser.parse_referenceable(input) {
      input = input2;
    }

    let (mut input, method_call_or_return) = opt(|input| MethodCallOrReturn::parse(input, &mut parser))(input)?;

    while let Ok((input2, _)) = parser.parse_referenceable(input) {
      input = input2;
    }

    let (input, MessageEnd) = MessageEnd::parse(input)?;

    Ok((input, Self { root_object, objects: parser.objects, method_call_or_return }))
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

    ValueDeserializer::new(&objects, &self.root_object).deserialize_any(visitor)
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map struct enum identifier ignored_any
  }
}
