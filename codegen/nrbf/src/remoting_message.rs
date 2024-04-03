use std::collections::BTreeMap;

use nom::IResult;
#[cfg(feature = "serde")]
use serde::{
  de::{value::Error, Deserializer, Visitor},
  forward_to_deserialize_any,
};

#[cfg(feature = "serde")]
use crate::value::ValueDeserializer;
use crate::{data_type::Int32, BinaryParser, Value};

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

#[derive(Debug, Clone, PartialEq)]
pub enum RemotingMessage<'i> {
  MethodCall(BTreeMap<Int32, Value<'i>>, MethodCall<'i>),
  MethodReturn(BTreeMap<Int32, Value<'i>>, MethodReturn<'i>),
  Value(BTreeMap<Int32, Value<'i>>, Value<'i>),
}

impl<'i> RemotingMessage<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let parser = BinaryParser::default();
    parser.deserialize(input)
  }

  #[cfg(feature = "serde")]
  fn into_deserializer<V: Visitor<'i>>(&self, visitor: &V) -> Result<ValueDeserializer<'i, '_>, Error> {
    use serde::de::{Error, Unexpected};

    match self {
      Self::MethodCall(..) => Err(Error::invalid_type(Unexpected::Other("method call"), visitor)),
      Self::MethodReturn(..) => Err(Error::invalid_type(Unexpected::Other("method return"), visitor)),
      Self::Value(objects, root_object) => Ok(ValueDeserializer::new(objects, root_object)),
    }
  }
}

#[cfg(feature = "serde")]
impl<'de> Deserializer<'de> for RemotingMessage<'de> {
  type Error = Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    self.into_deserializer(&visitor)?.deserialize_any(visitor)
  }

  fn deserialize_struct<V>(
    self,
    name: &'static str,
    fields: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    self.into_deserializer(&visitor)?.deserialize_struct(name, fields, visitor)
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map enum identifier ignored_any
  }
}
