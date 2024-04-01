use std::{collections::BTreeMap, ops::Index};

use nom::{
  branch::alt,
  combinator::{map, map_opt, opt, verify},
  multi::many_m_n,
  IResult,
};

use crate::{
  data_type::{Int32, LengthPrefixedString},
  grammar::{Class, Classes, MemberReference2},
  record::{
    BinaryLibrary, ClassWithId, ClassWithMembers, ClassWithMembersAndTypes, SystemClassWithMembers,
    SystemClassWithMembersAndTypes,
  },
};

#[derive(Debug)]
struct Object<'i> {
  class: &'i str,
  members: BTreeMap<&'i str, MemberReference2<'i>>,
}

#[derive(Debug, Default)]
pub struct BinaryParser<'i> {
  pub binary_libraries: BTreeMap<Int32, LengthPrefixedString<'i>>,
  pub classes: BTreeMap<Int32, Class<'i>>,
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

  pub fn parse_class(&mut self, input: &'i [u8]) -> IResult<&'i [u8], (Int32, Vec<MemberReference2<'i>>)> {
    let (input, (object_id, class)) = verify(
      alt((
        map_opt(ClassWithId::parse, |class| {
          let object_id = class.object_id();
          self.classes.get(&class.metadata_id()).map(|class| (object_id, class.clone()))
        }),
        map(ClassWithMembers::parse, |class| (class.object_id(), Class::ClassWithMembers(class))),
        map(ClassWithMembersAndTypes::parse, |class| (class.object_id(), Class::ClassWithMembersAndTypes(class))),
        map(SystemClassWithMembers::parse, |class| (class.object_id(), Class::SystemClassWithMembers(class))),
        map(SystemClassWithMembersAndTypes::parse, |class| {
          (class.object_id(), Class::SystemClassWithMembersAndTypes(class))
        }),
      )),
      |(object_id, _)| !self.classes.contains_key(&object_id),
    )(input)?;

    let (input, member_references) = match class {
      Class::ClassWithMembers(ref class) => {
        let member_count = class.class_info.member_names.len();
        many_m_n(member_count, member_count, |input| MemberReference2::parse(input, self))(input)?
      },
      Class::ClassWithMembersAndTypes(ref class) => {
        Classes::parse_member_references(input, &class.member_type_info, self)?
      },
      Class::SystemClassWithMembers(ref class) => {
        let member_count = class.class_info.member_names.len();
        many_m_n(member_count, member_count, |input| MemberReference2::parse(input, self))(input)?
      },
      Class::SystemClassWithMembersAndTypes(ref class) => {
        Classes::parse_member_references(input, &class.member_type_info, self)?
      },
    };

    let class_info = class.class_info();
    let object = Object {
      class: class_info.name.as_str(),
      members: BTreeMap::from_iter(
        class_info
          .member_names
          .iter()
          .zip(member_references.iter().cloned())
          .map(|(member_name, member_value)| (member_name.as_str(), member_value)),
      ),
    };

    self.classes.insert(object_id, class);

    Ok((input, (object_id, member_references)))
  }
}
