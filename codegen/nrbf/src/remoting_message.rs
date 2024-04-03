use std::collections::BTreeMap;

use nom::IResult;
#[cfg(feature = "serde")]
use serde::{
  de::{value::Error, Deserializer, Visitor},
  forward_to_deserialize_any,
};

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

/// 2.7 Binary Record Grammar - `remotingMessage`
#[derive(Debug, Clone, PartialEq)]
pub struct RemotingMessage<'i> {
  pub root_object: Value<'i>,
  pub objects: BTreeMap<Int32, Value<'i>>,
  pub method_call_or_return: Option<MethodCallOrReturn<'i>>,
}

impl<'i> RemotingMessage<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let parser = BinaryParser::default();
    parser.deserialize(input)
  }

  fn check_deserializable<V: Visitor<'i>>(&self, visitor: &V) -> Result<(), Error> {
    use serde::de::{Error, Unexpected};

    match self.method_call_or_return {
      Some(MethodCallOrReturn::MethodCall(_)) => Err(Error::invalid_type(Unexpected::Other("method call"), visitor)),
      Some(MethodCallOrReturn::MethodReturn(_)) => {
        Err(Error::invalid_type(Unexpected::Other("method return"), visitor))
      },
      None => Ok(()),
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
    use crate::value::ValueDeserializer;

    self.check_deserializable(&visitor)?;

    let objects = self.objects;

    ValueDeserializer::new(&objects, &self.root_object).deserialize_any(visitor)
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
    use crate::value::ValueDeserializer;

    let objects = self.objects;

    ValueDeserializer::new(&objects, &self.root_object).deserialize_struct(name, fields, visitor)
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map enum identifier ignored_any
  }
}
