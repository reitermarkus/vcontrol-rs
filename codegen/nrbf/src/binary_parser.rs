use std::{collections::BTreeMap, ops::Index};

use nom::{
  branch::alt,
  combinator::{fail, map, map_opt, opt, verify},
  multi::many_m_n,
  IResult, ToUsize,
};
#[cfg(feature = "serde")]
use serde::{
  de::value::{Error, SeqDeserializer},
  de::{IntoDeserializer, Visitor},
  forward_to_deserialize_any, Deserializer,
};

use crate::{
  common::{AdditionalTypeInfo, MemberTypeInfo},
  data_type::{Int32, LengthPrefixedString},
  enumeration::{BinaryArrayType, BinaryType},
  grammar::{self, Class, MemberReferenceInner, NullObject},
  record::{
    self, ArraySingleObject, ArraySinglePrimitive, ArraySingleString, BinaryArray, BinaryLibrary, BinaryObjectString,
    ClassWithId, ClassWithMembers, ClassWithMembersAndTypes, MemberPrimitiveTyped, MemberPrimitiveUnTyped,
    MemberReference, SystemClassWithMembers, SystemClassWithMembersAndTypes,
  },
};

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectClass<'i> {
  name: &'i str,
  library: Option<&'i str>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object<'i> {
  Object { class: ObjectClass<'i>, members: BTreeMap<&'i str, Object<'i>> },
  Array(Array<'i>),
  Primitive(MemberPrimitiveUnTyped),
  String(&'i str),
  Null(usize),
  Ref(Int32),
}

#[cfg(feature = "serde")]
impl<'de> IntoDeserializer<'de, Error> for Object<'de> {
  type Deserializer = Self;

  fn into_deserializer(self) -> Self::Deserializer {
    self
  }
}

#[cfg(feature = "serde")]
impl<'de> Deserializer<'de> for Object<'de> {
  type Error = Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    match self {
      Self::Object { .. } => {
        unimplemented!()
      },
      Self::Array(array) => match array {
        Array::Object(objects) => SeqDeserializer::new(objects.into_iter()).deserialize_any(visitor),
        Array::Binary(_) => unimplemented!(),
      },
      Self::Primitive(primitive) => primitive.deserialize_any(visitor),
      Self::String(s) => visitor.visit_borrowed_str(s),
      Self::Null(1) => visitor.visit_none(),
      Self::Null(_) => {
        unimplemented!()
      },
      Self::Ref(_) => {
        unimplemented!()
      },
    }
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map struct enum identifier ignored_any
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Array<'i> {
  Object(Vec<Object<'i>>),
  Binary(BTreeMap<Int32, Object<'i>>),
}

#[derive(Debug, Default)]
pub struct BinaryParser<'i> {
  pub binary_libraries: BTreeMap<Int32, LengthPrefixedString<'i>>,
  pub classes: BTreeMap<Int32, Class<'i>>,
  pub objects: BTreeMap<Int32, Object<'i>>,
}

impl<'i> BinaryParser<'i> {
  pub fn parse_binary_library(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    let (input, binary_library) = opt(verify(BinaryLibrary::parse, |binary_library| {
      !self.binary_libraries.contains_key(&binary_library.library_id())
    }))(input)?;

    if let Some(binary_library) = binary_library {
      self.binary_libraries.insert(binary_library.library_id(), binary_library.library_name);
    }

    Ok((input, ()))
  }

  /// 2.7 Binary Record Grammar - `memberReference`
  pub fn parse_member_reference(
    &mut self,
    input: &'i [u8],
    type_enum_and_additional_type_info: Option<(BinaryType, Option<&AdditionalTypeInfo<'i>>)>,
  ) -> IResult<&'i [u8], Object<'i>> {
    let (input, ()) = self.parse_binary_library(input)?;

    let (input, (object_id, object)) =
      if let Some((type_enum, additional_type_info)) = type_enum_and_additional_type_info {
        match (type_enum, additional_type_info) {
          (BinaryType::Primitive, Some(AdditionalTypeInfo::Primitive(primitive_type))) => {
            map(|input| MemberPrimitiveUnTyped::parse(input, *primitive_type), |p| (None, Object::Primitive(p)))(input)?
          },
          (BinaryType::String, None) => {
            map(BinaryObjectString::parse, |s| (Some(s.object_id()), Object::String(s.as_str())))(input)?
          },
          (BinaryType::Object, None) => return self.parse_member_reference(input, None),
          (BinaryType::SystemClass, Some(class_name)) => unimplemented!("{class_name:?}"),
          (BinaryType::Class, Some(class_type_info)) => {
            unimplemented!("{class_type_info:?}")
          },
          (BinaryType::ObjectArray, None) => return self.parse_member_reference(input, None),
          (BinaryType::StringArray, None) => alt((
            map(BinaryObjectString::parse, |s| (Some(s.object_id()), Object::String(s.as_str()))),
            map(MemberReference::parse, |member_reference| (None, Object::Ref(member_reference.id_ref))),
            map(NullObject::parse, |n| (None, Object::Null(n.null_count()))),
          ))(input)?,
          (BinaryType::PrimitiveArray, Some(additional_type_info)) => unimplemented!("{additional_type_info:?}"),
          _ => unreachable!(),
        }
      } else {
        alt((
          map(MemberPrimitiveTyped::parse, |p| (None, Object::Primitive(p.into_untyped()))),
          map(MemberReference::parse, |member_reference| (None, Object::Ref(member_reference.id_ref))),
          map(BinaryObjectString::parse, |s| (Some(s.object_id()), Object::String(s.as_str()))),
          map(NullObject::parse, |n| (None, Object::Null(n.null_count()))),
          map(|input| self.parse_classes(input), |o| (None, o)),
        ))(input)?
      };

    if let Some(object_id) = object_id {
      self.objects.insert(object_id, object.clone());
    }

    Ok((input, object))
  }

  fn parse_members_with_type_info(
    &mut self,
    mut input: &'i [u8],
    member_type_info: &MemberTypeInfo<'i>,
  ) -> IResult<&'i [u8], Vec<Object<'i>>> {
    let mut member_references = vec![];

    for (binary_type_enum, additional_info) in
      member_type_info.binary_type_enums.iter().zip(member_type_info.additional_infos.iter())
    {
      let member;
      (input, member) = self.parse_member_reference(input, Some((*binary_type_enum, additional_info.as_ref())))?;
      member_references.push(member);
    }

    Ok((input, member_references))
  }

  pub fn parse_class(&mut self, input: &'i [u8]) -> IResult<&'i [u8], (Int32, Vec<Object<'i>>)> {
    let (input, (object_id, class)) = verify(
      alt((
        map_opt(ClassWithId::parse, |class| {
          let object_id = class.object_id();
          self.classes.get(&class.metadata_id()).map(|class| (object_id, class.clone()))
        }),
        map(verify(ClassWithMembers::parse, |class| self.binary_libraries.contains_key(&class.library_id)), |class| {
          (class.object_id(), Class::ClassWithMembers(class))
        }),
        map(
          verify(ClassWithMembersAndTypes::parse, |class| self.binary_libraries.contains_key(&class.library_id)),
          |class| (class.object_id(), Class::ClassWithMembersAndTypes(class)),
        ),
        map(SystemClassWithMembers::parse, |class| (class.object_id(), Class::SystemClassWithMembers(class))),
        map(SystemClassWithMembersAndTypes::parse, |class| {
          (class.object_id(), Class::SystemClassWithMembersAndTypes(class))
        }),
      )),
      |(object_id, _)| !self.classes.contains_key(&object_id),
    )(input)?;

    let (input, (object_class, member_references)) = match class {
      Class::ClassWithMembers(ref class) => {
        let object_class = ObjectClass {
          name: class.class_info().name.as_str(),
          library: Some(self.binary_libraries[&class.library_id].as_str()),
        };

        let member_count = class.class_info().member_names.len();
        let (input, member_references) =
          many_m_n(member_count, member_count, |input| self.parse_member_reference(input, None))(input)?;

        (input, (object_class, member_references))
      },
      Class::ClassWithMembersAndTypes(ref class) => {
        let object_class = ObjectClass {
          name: class.class_info().name.as_str(),
          library: Some(self.binary_libraries[&class.library_id].as_str()),
        };

        let (input, member_references) = self.parse_members_with_type_info(input, &class.member_type_info)?;

        (input, (object_class, member_references))
      },
      Class::SystemClassWithMembers(ref class) => {
        let object_class = ObjectClass { name: class.class_info().name.as_str(), library: None };

        let member_count = class.class_info().member_names.len();
        let (input, member_references) =
          many_m_n(member_count, member_count, |input| self.parse_member_reference(input, None))(input)?;

        (input, (object_class, member_references))
      },
      Class::SystemClassWithMembersAndTypes(ref class) => {
        let object_class = ObjectClass { name: class.class_info().name.as_str(), library: None };

        let (input, member_references) = self.parse_members_with_type_info(input, &class.member_type_info)?;

        (input, (object_class, member_references))
      },
    };

    let class_info = class.class_info();
    let object = Object::Object {
      class: object_class,
      members: BTreeMap::from_iter(
        class_info
          .member_names
          .iter()
          .zip(member_references.iter().cloned())
          .map(|(member_name, member_value)| (member_name.as_str(), member_value)),
      ),
    };

    self.classes.insert(object_id, class);
    self.objects.insert(object_id, object);

    Ok((input, (object_id, member_references)))
  }

  pub fn parse_classes(&mut self, input: &'i [u8]) -> IResult<&'i [u8], Object<'i>> {
    let (input, ()) = self.parse_binary_library(input)?;

    let (input, (object_id, _)) = self.parse_class(input)?;

    Ok((input, self.objects[&object_id].clone()))
  }

  fn parse_array_single_object(&mut self, input: &'i [u8]) -> IResult<&'i [u8], grammar::Array<'i>> {
    let (mut input, array) = ArraySingleObject::parse(input)?;

    let mut members = vec![];

    let mut len_remaining = array.array_info.len();
    while len_remaining > 0 {
      let member;
      (input, member) = self.parse_member_reference(input, None)?;

      let count = match member {
        Object::Null(count) => count,
        _ => 1,
      };

      members.push(member);
      len_remaining -= count;
    }

    let object_id = array.object_id();
    self.objects.insert(object_id, Object::Array(Array::Object(members.clone())));

    Ok((input, grammar::Array::ArraySingleObject(object_id, members)))
  }

  fn parse_array_single_primitive(&mut self, input: &'i [u8]) -> IResult<&'i [u8], grammar::Array<'i>> {
    let (mut input, array) = ArraySinglePrimitive::parse(input)?;

    let length = array.array_info.len();
    let (input, members) = many_m_n(
      length,
      length,
      map(|input| MemberPrimitiveUnTyped::parse(input, array.primitive_type), |primitive| Object::Primitive(primitive)),
    )(input)?;

    let object_id = array.object_id();
    self.objects.insert(object_id, Object::Array(Array::Object(members.clone())));

    Ok((input, grammar::Array::ArraySinglePrimitive(object_id, members)))
  }

  fn parse_array_single_string(&mut self, input: &'i [u8]) -> IResult<&'i [u8], grammar::Array<'i>> {
    let (mut input, array) = ArraySingleString::parse(input)?;

    let mut members = vec![];

    let mut len_remaining = array.array_info.len();
    while len_remaining > 0 {
      let member;
      (input, member) = self.parse_member_reference(input, Some((BinaryType::StringArray, None)))?;

      let count = match member {
        Object::Null(count) => count,
        _ => 1,
      };

      members.push(member);
      len_remaining -= count;
    }

    let object_id = array.object_id();
    self.objects.insert(object_id, Object::Array(Array::Object(members.clone())));

    Ok((input, grammar::Array::ArraySingleString(object_id, members)))
  }

  fn parse_binary_array(&mut self, input: &'i [u8]) -> IResult<&'i [u8], grammar::Array<'i>> {
    let (mut input, array) = BinaryArray::parse(input)?;

    let member_count = match array.binary_array_type_enum {
      BinaryArrayType::Single | BinaryArrayType::SingleOffset => array.lengths.first().map(|n| i32::from(*n) as u32),
      BinaryArrayType::Rectangular | BinaryArrayType::RectangularOffset => {
        array.lengths.iter().try_fold(1u32, |acc, n| acc.checked_mul(i32::from(*n) as u32))
      },
      BinaryArrayType::Jagged | BinaryArrayType::JaggedOffset => array.lengths.first().map(|n| i32::from(*n) as u32),
    };
    let member_count = match member_count {
      Some(member_count) => member_count.to_usize(),
      None => return fail(input),
    };
    let (input, members) = many_m_n(member_count, member_count, |input| {
      self.parse_member_reference(input, Some((array.type_enum, array.additional_type_info.as_ref())))
    })(input)?;

    let object_id = array.object_id();
    self.objects.insert(object_id, Object::Array(Array::Object(members.clone())));

    Ok((input, grammar::Array::BinaryArray(object_id, members)))
  }

  pub fn parse_array(&mut self, input: &'i [u8]) -> IResult<&'i [u8], grammar::Array<'i>> {
    if let Ok((input, array)) = self.parse_array_single_object(input) {
      return Ok((input, array))
    }

    if let Ok((input, array)) = self.parse_array_single_primitive(input) {
      return Ok((input, array))
    }

    if let Ok((input, array)) = self.parse_array_single_string(input) {
      return Ok((input, array))
    }

    self.parse_binary_array(input)
  }
}
