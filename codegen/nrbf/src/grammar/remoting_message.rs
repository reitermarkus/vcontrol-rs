use std::collections::BTreeMap;

use nom::{
  branch::alt,
  combinator::{map, opt},
  multi::many0,
  IResult,
};
#[cfg(feature = "serde")]
use serde::{
  de::{value::Error, Deserializer, Visitor},
  forward_to_deserialize_any,
};

use crate::{
  grammar::{MemberReferenceInner, MethodCall, MethodReturn, Referenceable},
  record::{MessageEnd, SerializationHeader},
};

#[derive(Debug, Clone, PartialEq)]
pub enum MethodCallOrReturn<'i> {
  MethodCall(MethodCall<'i>),
  MethodReturn(MethodReturn<'i>),
}

impl<'i> MethodCallOrReturn<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    alt((map(MethodCall::parse, Self::MethodCall), map(MethodReturn::parse, Self::MethodReturn)))(input)
  }
}

/// 2.7 Binary Record Grammar - `remotingMessage`
#[derive(Debug, Clone, PartialEq)]
pub struct RemotingMessage<'i> {
  pub header: SerializationHeader,
  pub pre_method_referenceables: Vec<Referenceable<'i>>,
  pub method_call_or_return: Option<MethodCallOrReturn<'i>>,
  pub post_method_referenceables: Vec<Referenceable<'i>>,
  pub end: MessageEnd,
}

impl<'i> RemotingMessage<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, header) = SerializationHeader::parse(input)?;
    let (input, pre_method_referenceables) = many0(Referenceable::parse)(input)?;
    let (input, method_call_or_return) = opt(MethodCallOrReturn::parse)(input)?;
    let (input, post_method_referenceables) = many0(Referenceable::parse)(input)?;
    let (input, end) = MessageEnd::parse(input)?;

    Ok((input, Self { header, pre_method_referenceables, method_call_or_return, post_method_referenceables, end }))
  }
}

#[cfg(feature = "serde")]
impl<'de> Deserializer<'de> for RemotingMessage<'de> {
  type Error = Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    use serde::de::{value::SeqDeserializer, Error, Unexpected};

    use crate::{
      grammar::{Array, Arrays},
      record::{ArraySinglePrimitive, ArraySingleString},
    };

    match self.method_call_or_return {
      Some(MethodCallOrReturn::MethodCall(_)) => {
        return Err(Error::invalid_type(Unexpected::Other("method call"), &visitor))
      },
      Some(MethodCallOrReturn::MethodReturn(_)) => {
        return Err(Error::invalid_type(Unexpected::Other("method return"), &visitor))
      },
      None => (),
    }

    let mut referenceables =
      BTreeMap::from_iter(self.pre_method_referenceables.into_iter().map(|r| (i32::from(r.object_id()), r)));

    let root_item = referenceables.remove(&(self.header.root_id.into()));

    match root_item {
      Some(Referenceable::Arrays(Arrays { binary_library: None, array })) => match array {
        Array::ArraySinglePrimitive(ArraySinglePrimitive { array_info: _, members }) => {
          let it = members.into_iter();
          SeqDeserializer::new(it).deserialize_any(visitor)
        },
        Array::ArraySingleString(ArraySingleString { array_info: _, members }) => {
          let it = members.into_iter().map(|member| match member {
            MemberReferenceInner::BinaryObjectString(s) => s,
            MemberReferenceInner::MemberReference(member) => match referenceables.remove(&(member.id_ref.into())) {
              Some(Referenceable::BinaryObjectString(s)) => s,
              _ => unimplemented!(),
            },
            MemberReferenceInner::NullObject(_) => unimplemented!(),
            _ => unreachable!(),
          });
          SeqDeserializer::new(it).deserialize_any(visitor)
        },
        _ => unimplemented!(),
      },
      _ => unimplemented!(),
    }
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map struct enum identifier ignored_any
  }
}
