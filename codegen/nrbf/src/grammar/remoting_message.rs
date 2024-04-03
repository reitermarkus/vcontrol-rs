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
  grammar::{MethodCall, MethodReturn, Referenceable},
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
impl<'de, 'i> Deserializer<'de> for &'de RemotingMessage<'i> {
  type Error = Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    use serde::de::value::SeqDeserializer;

    use crate::{
      grammar::{Array, Arrays},
      record::{ArraySinglePrimitive, ArraySingleString},
    };

    let root_item = self.pre_method_referenceables.iter().find(|r| r.object_id() == self.header.root_id);

    match root_item {
      Some(Referenceable::Arrays(Arrays { binary_library: None, array })) => match array {
        Array::ArraySinglePrimitive(ArraySinglePrimitive { array_info, members }) => {
          let mut it = members.iter();
          let mut deserializer = SeqDeserializer::new(&mut it);
          let seq = visitor.visit_seq(&mut deserializer)?;
          deserializer.end()?;
          Ok(seq)
        },
        Array::ArraySingleString(ArraySingleString { array_info, members: _ }) => {
          let mut it = members.iter().filter_map(|member| match member {});

          unimplemented!()
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
