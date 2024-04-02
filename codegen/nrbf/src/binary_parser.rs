use std::{collections::BTreeMap, ops::Index};

use nom::{
  branch::alt,
  combinator::{map, map_opt, opt, verify},
  multi::many_m_n,
  IResult,
};

use crate::{
  common::{AdditionalTypeInfo, MemberTypeInfo},
  data_type::{Int32, LengthPrefixedString},
  enumeration::BinaryType,
  grammar::{Class, MemberReferenceInner},
  record::{
    BinaryLibrary, BinaryObjectString, ClassWithId, ClassWithMembers, ClassWithMembersAndTypes, MemberPrimitiveUnTyped,
    SystemClassWithMembers, SystemClassWithMembersAndTypes,
  },
};

#[derive(Debug)]
struct ObjectClass<'i> {
  name: &'i str,
  library: Option<&'i str>,
}

#[derive(Debug)]
struct Object<'i> {
  class: ObjectClass<'i>,
  members: BTreeMap<&'i str, MemberReferenceInner<'i>>,
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
  ) -> IResult<&'i [u8], MemberReferenceInner<'i>> {
    let (input, ()) = self.parse_binary_library(input)?;

    if let Some((type_enum, additional_type_info)) = type_enum_and_additional_type_info {
      match (type_enum, additional_type_info) {
        (BinaryType::Primitive, Some(AdditionalTypeInfo::Primitive(primitive_type))) => map(
          |input| MemberPrimitiveUnTyped::parse(input, *primitive_type),
          |value| MemberReferenceInner::MemberPrimitiveUnTyped(value),
        )(input),
        (BinaryType::String, None) => {
          map(BinaryObjectString::parse, |value| MemberReferenceInner::BinaryObjectString(value))(input)
        },
        (BinaryType::Object, None) => MemberReferenceInner::parse(input, self),
        (BinaryType::SystemClass, Some(class_name)) => unimplemented!("{class_name:?}"),
        (BinaryType::Class, Some(class_type_info)) => {
          unimplemented!("{class_type_info:?}")
        },
        (BinaryType::ObjectArray, None) => MemberReferenceInner::parse(input, self),
        (BinaryType::StringArray, None) => MemberReferenceInner::parse(input, self),
        (BinaryType::PrimitiveArray, Some(additional_type_info)) => unimplemented!("{additional_type_info:?}"),
        _ => unreachable!(),
      }
    } else {
      MemberReferenceInner::parse(input, self)
    }
  }

  fn parse_members_with_type_info(
    &mut self,
    mut input: &'i [u8],
    member_type_info: &MemberTypeInfo<'i>,
  ) -> IResult<&'i [u8], Vec<MemberReferenceInner<'i>>> {
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

  pub fn parse_class(&mut self, input: &'i [u8]) -> IResult<&'i [u8], (Int32, Vec<MemberReferenceInner<'i>>)> {
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
          name: class.class_info.name.as_str(),
          library: Some(self.binary_libraries[&class.library_id].as_str()),
        };

        let member_count = class.class_info.member_names.len();
        let (input, member_references) =
          many_m_n(member_count, member_count, |input| self.parse_member_reference(input, None))(input)?;

        (input, (object_class, member_references))
      },
      Class::ClassWithMembersAndTypes(ref class) => {
        let object_class = ObjectClass {
          name: class.class_info.name.as_str(),
          library: Some(self.binary_libraries[&class.library_id].as_str()),
        };

        let (input, member_references) = self.parse_members_with_type_info(input, &class.member_type_info)?;

        (input, (object_class, member_references))
      },
      Class::SystemClassWithMembers(ref class) => {
        let object_class = ObjectClass { name: class.class_info.name.as_str(), library: None };

        let member_count = class.class_info.member_names.len();
        let (input, member_references) =
          many_m_n(member_count, member_count, |input| self.parse_member_reference(input, None))(input)?;

        (input, (object_class, member_references))
      },
      Class::SystemClassWithMembersAndTypes(ref class) => {
        let object_class = ObjectClass { name: class.class_info.name.as_str(), library: None };

        let (input, member_references) = self.parse_members_with_type_info(input, &class.member_type_info)?;

        (input, (object_class, member_references))
      },
    };

    let class_info = class.class_info();
    let object = Object {
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
}
