use crate::{
  binary_parser::Object,
  common::ClassInfo,
  record::{ClassWithMembers, ClassWithMembersAndTypes, SystemClassWithMembers, SystemClassWithMembersAndTypes},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Class<'i> {
  ClassWithMembers(ClassWithMembers<'i>),
  ClassWithMembersAndTypes(ClassWithMembersAndTypes<'i>),
  SystemClassWithMembers(SystemClassWithMembers<'i>),
  SystemClassWithMembersAndTypes(SystemClassWithMembersAndTypes<'i>),
}

impl<'i> Class<'i> {
  pub fn class_info(&self) -> &ClassInfo<'i> {
    match self {
      Self::ClassWithMembers(class) => &class.class_info,
      Self::ClassWithMembersAndTypes(class) => &class.class_info,
      Self::SystemClassWithMembers(class) => &class.class_info,
      Self::SystemClassWithMembersAndTypes(class) => &class.class_info,
    }
  }
}

/// 2.7 Binary Record Grammar - `Classes`
#[derive(Debug, Clone, PartialEq)]
pub struct Classes<'i> {
  pub class: Object<'i>,
}
