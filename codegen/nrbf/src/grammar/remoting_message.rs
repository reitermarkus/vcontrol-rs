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
  binary_parser::Object,
  data_type::{Int32, LengthPrefixedString},
  grammar::{MethodCall, MethodReturn},
  record::{MessageEnd, SerializationHeader},
  BinaryParser,
};

#[derive(Debug, Clone, PartialEq)]
pub enum MethodCallOrReturn<'i> {
  MethodCall(MethodCall<'i>),
  MethodReturn(MethodReturn<'i>),
}

impl<'i> MethodCallOrReturn<'i> {
  pub fn parse(input: &'i [u8], parser: &mut BinaryParser<'i>) -> IResult<&'i [u8], Self> {
    if let Ok(s) = map(|input| MethodCall::parse(input, parser), Self::MethodCall)(input) {
      return Ok(s)
    }

    if let Ok(s) = map(|input| MethodReturn::parse(input, parser), |mr| Self::MethodReturn(mr))(input) {
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

    use crate::record::MemberPrimitiveUnTyped;

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

    let root_object = objects.get(&self.header.root_id).cloned();
    match root_object {
      Some(Object::Object { class, members }) => {
        if class.library.is_some() {
          return Err(Error::invalid_type(Unexpected::Other(class.name), &visitor))
        }

        let value = if let Some(value) = members.get("m_value") {
          value
        } else {
          return Err(Error::invalid_type(Unexpected::Other(class.name), &visitor))
        };

        match (class.name, value) {
          ("System.Boolean", Object::Primitive(MemberPrimitiveUnTyped::Boolean(n))) => visitor.visit_bool((*n).into()),
          ("System.Byte", Object::Primitive(MemberPrimitiveUnTyped::Byte(n))) => visitor.visit_u8((*n).into()),
          ("System.SByte", Object::Primitive(MemberPrimitiveUnTyped::SByte(n))) => visitor.visit_i8((*n).into()),
          ("System.Char", Object::Primitive(MemberPrimitiveUnTyped::Char(c))) => visitor.visit_char((*c).into()),
          ("System.Decimal", Object::Primitive(MemberPrimitiveUnTyped::Decimal(_c))) => unimplemented!(),
          ("System.Double", Object::Primitive(MemberPrimitiveUnTyped::Double(n))) => visitor.visit_f64((*n).into()),
          ("System.Single", Object::Primitive(MemberPrimitiveUnTyped::Single(n))) => visitor.visit_f32((*n).into()),
          ("System.Int32", Object::Primitive(MemberPrimitiveUnTyped::Int32(n))) => visitor.visit_i32((*n).into()),
          ("System.UInt32", Object::Primitive(MemberPrimitiveUnTyped::UInt32(n))) => visitor.visit_u32((*n).into()),
          ("System.Int64", Object::Primitive(MemberPrimitiveUnTyped::Int64(n))) => visitor.visit_i64((*n).into()),
          ("System.UInt64", Object::Primitive(MemberPrimitiveUnTyped::UInt64(n))) => visitor.visit_u64((*n).into()),
          ("System.Int16", Object::Primitive(MemberPrimitiveUnTyped::Int16(n))) => visitor.visit_i16((*n).into()),
          ("System.UInt16", Object::Primitive(MemberPrimitiveUnTyped::UInt16(n))) => visitor.visit_u16((*n).into()),
          (name, _) => Err(Error::custom(format!("invalid system type: {}", name))),
        }
      },
      Some(object) => object.deserialize_any(visitor),
      None => Err(Error::custom("root object not found")),
    }
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map struct enum identifier ignored_any
  }
}
