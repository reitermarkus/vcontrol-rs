use std::collections::BTreeMap;

use nom::{
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
  data_type::{Int32, LengthPrefixedString},
  grammar::{Class, MethodCall, MethodReturn, Referenceable},
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
  pub binary_libraries: BTreeMap<Int32, LengthPrefixedString<'i>>,
  pub classes: BTreeMap<Int32, Class<'i>>,
  pub pre_method_referenceables: Vec<Referenceable<'i>>,
  pub method_call_or_return: Option<MethodCallOrReturn<'i>>,
  pub post_method_referenceables: Vec<Referenceable<'i>>,
  pub end: MessageEnd,
}

impl<'i> RemotingMessage<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let mut parser = BinaryParser::default();

    let (input, header) = SerializationHeader::parse(input)?;
    let (input, pre_method_referenceables) = many0(|input| Referenceable::parse(input, &mut parser))(input)?;
    let (input, method_call_or_return) = opt(|input| MethodCallOrReturn::parse(input, &mut parser))(input)?;
    let (input, post_method_referenceables) = many0(|input| Referenceable::parse(input, &mut parser))(input)?;
    let (input, end) = MessageEnd::parse(input)?;

    Ok((
      input,
      Self {
        header,
        binary_libraries: parser.binary_libraries,
        classes: parser.classes,
        pre_method_referenceables,
        method_call_or_return,
        post_method_referenceables,
        end,
      },
    ))
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
      grammar::{Array, Arrays, Class, Classes, MemberReferenceInner},
      record::{ArraySinglePrimitive, ArraySingleString, MemberPrimitiveUnTyped},
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

    let root_object = referenceables.remove(&(self.header.root_id.into()));
    match root_object {
      Some(Referenceable::Classes(Classes { class_id, member_references })) => {
        match self.classes.get(&class_id).unwrap() {
          Class::ClassWithMembers(class) => {
            Err(Error::invalid_type(Unexpected::Other(class.class_info.name.as_str()), &visitor))
          },
          Class::ClassWithMembersAndTypes(class) => {
            Err(Error::invalid_type(Unexpected::Other(class.class_info.name.as_str()), &visitor))
          },
          Class::SystemClassWithMembers(class) => {
            Err(Error::invalid_type(Unexpected::Other(class.class_info.name.as_str()), &visitor))
          },
          Class::SystemClassWithMembersAndTypes(class) => {
            match (
              class.class_info.name.as_str(),
              class.class_info.member_names.as_slice(),
              member_references.as_slice(),
            ) {
              (
                "System.Boolean",
                [LengthPrefixedString("m_value")],
                [MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Boolean(n))],
              ) => visitor.visit_bool((*n).into()),
              (
                "System.Byte",
                [LengthPrefixedString("m_value")],
                [MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Byte(n))],
              ) => visitor.visit_u8((*n).into()),
              (
                "System.SByte",
                [LengthPrefixedString("m_value")],
                [MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::SByte(n))],
              ) => visitor.visit_i8((*n).into()),
              (
                "System.Char",
                [LengthPrefixedString("m_value")],
                [MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Char(c))],
              ) => visitor.visit_char((*c).into()),
              (
                "System.Decimal",
                [LengthPrefixedString("m_value")],
                [MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Decimal(_c))],
              ) => unimplemented!(),
              (
                "System.Double",
                [LengthPrefixedString("m_value")],
                [MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Double(n))],
              ) => visitor.visit_f64((*n).into()),
              (
                "System.Single",
                [LengthPrefixedString("m_value")],
                [MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Single(n))],
              ) => visitor.visit_f32((*n).into()),
              (
                "System.Int32",
                [LengthPrefixedString("m_value")],
                [MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(n))],
              ) => visitor.visit_i32((*n).into()),
              (
                "System.UInt32",
                [LengthPrefixedString("m_value")],
                [MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::UInt32(n))],
              ) => visitor.visit_u32((*n).into()),
              (
                "System.Int64",
                [LengthPrefixedString("m_value")],
                [MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int64(n))],
              ) => visitor.visit_i64((*n).into()),
              (
                "System.UInt64",
                [LengthPrefixedString("m_value")],
                [MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::UInt64(n))],
              ) => visitor.visit_u64((*n).into()),
              (
                "System.Int16",
                [LengthPrefixedString("m_value")],
                [MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int16(n))],
              ) => visitor.visit_i16((*n).into()),
              (
                "System.UInt16",
                [LengthPrefixedString("m_value")],
                [MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::UInt16(n))],
              ) => visitor.visit_u16((*n).into()),
              (name, _, _) => Err(Error::custom(format!("invalid system type: {}", name))),
            }
          },
        }
      },
      Some(Referenceable::Arrays(Arrays { array })) => match array {
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
      Some(Referenceable::BinaryObjectString(s)) => s.deserialize_any(visitor),
      None => Err(Error::custom("root object not found")),
    }
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map struct enum identifier ignored_any
  }
}
