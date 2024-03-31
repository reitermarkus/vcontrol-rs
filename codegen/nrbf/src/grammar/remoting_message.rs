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
      data_type::LengthPrefixedString,
      grammar::{Array, Arrays, Class, Classes, MemberReference2, MemberReferenceInner},
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
      Some(Referenceable::Classes(Classes { binary_library: None, class, member_references })) => match class {
        Class::ClassWithId(class) => unimplemented!(),
        Class::ClassWithMembers(class) => unimplemented!(),
        Class::ClassWithMembersAndTypes(class) => unimplemented!(),
        Class::SystemClassWithMembers(class) => unimplemented!(),
        Class::SystemClassWithMembersAndTypes(class) => {
          match (class.class_info.name.as_str(), class.class_info.member_names.as_slice(), member_references.as_slice())
          {
            (
              "System.Boolean",
              [LengthPrefixedString("m_value")],
              [MemberReference2 {
                binary_library: None,
                member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Boolean(n)),
              }],
            ) => visitor.visit_bool((*n).into()),
            (
              "System.Byte",
              [LengthPrefixedString("m_value")],
              [MemberReference2 {
                binary_library: None,
                member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Byte(n)),
              }],
            ) => visitor.visit_u8((*n).into()),
            (
              "System.SByte",
              [LengthPrefixedString("m_value")],
              [MemberReference2 {
                binary_library: None,
                member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::SByte(n)),
              }],
            ) => visitor.visit_i8((*n).into()),
            (
              "System.Char",
              [LengthPrefixedString("m_value")],
              [MemberReference2 {
                binary_library: None,
                member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Char(c)),
              }],
            ) => visitor.visit_char((*c).into()),
            (
              "System.Decimal",
              [LengthPrefixedString("m_value")],
              [MemberReference2 {
                binary_library: None,
                member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Decimal(c)),
              }],
            ) => unimplemented!(),
            (
              "System.Double",
              [LengthPrefixedString("m_value")],
              [MemberReference2 {
                binary_library: None,
                member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Double(n)),
              }],
            ) => visitor.visit_f64((*n).into()),
            (
              "System.Single",
              [LengthPrefixedString("m_value")],
              [MemberReference2 {
                binary_library: None,
                member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Single(n)),
              }],
            ) => visitor.visit_f32((*n).into()),
            (
              "System.Int32",
              [LengthPrefixedString("m_value")],
              [MemberReference2 {
                binary_library: None,
                member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(n)),
              }],
            ) => visitor.visit_i32((*n).into()),
            (
              "System.UInt32",
              [LengthPrefixedString("m_value")],
              [MemberReference2 {
                binary_library: None,
                member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::UInt32(n)),
              }],
            ) => visitor.visit_u32((*n).into()),
            (
              "System.Int64",
              [LengthPrefixedString("m_value")],
              [MemberReference2 {
                binary_library: None,
                member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int64(n)),
              }],
            ) => visitor.visit_i64((*n).into()),
            (
              "System.UInt64",
              [LengthPrefixedString("m_value")],
              [MemberReference2 {
                binary_library: None,
                member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::UInt64(n)),
              }],
            ) => visitor.visit_u64((*n).into()),
            (
              "System.Int16",
              [LengthPrefixedString("m_value")],
              [MemberReference2 {
                binary_library: None,
                member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int16(n)),
              }],
            ) => visitor.visit_i16((*n).into()),
            (
              "System.UInt16",
              [LengthPrefixedString("m_value")],
              [MemberReference2 {
                binary_library: None,
                member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::UInt16(n)),
              }],
            ) => visitor.visit_u16((*n).into()),
            (name, _, _) => Err(Error::custom(format!("invalid system type: {}", name))),
          }
        },
      },
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
      Some(Referenceable::BinaryObjectString(s)) => s.deserialize_any(visitor),
      Some(Referenceable::Classes(Classes { binary_library: Some(_), .. }))
      | Some(Referenceable::Arrays(Arrays { binary_library: Some(_), .. })) => {
        Err(Error::custom("deserializing a binary library is not supported"))
      },
      None => Err(Error::custom("root object not found")),
    }
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map struct enum identifier ignored_any
  }
}
