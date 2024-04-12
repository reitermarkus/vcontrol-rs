//! 2.4.3 Record Definitions

use nom::{bytes::complete::tag, combinator::value, error::ParseError, Compare, IResult, InputTake, Parser};

use crate::error::{error_position, Error};

mod serialization_header;
pub use serialization_header::SerializationHeader;
mod class_with_id;
pub use class_with_id::ClassWithId;
mod system_class_with_members;
pub use system_class_with_members::SystemClassWithMembers;
mod class_with_members;
pub use class_with_members::ClassWithMembers;
mod system_class_with_members_and_types;
pub use system_class_with_members_and_types::SystemClassWithMembersAndTypes;
mod class_with_members_and_types;
pub use class_with_members_and_types::ClassWithMembersAndTypes;
mod binary_object_string;
pub use binary_object_string::BinaryObjectString;
mod binary_array;
pub use binary_array::BinaryArray;
mod member_primitive_typed;
pub use member_primitive_typed::MemberPrimitiveTyped;
mod member_primitive_untyped;
pub use member_primitive_untyped::MemberPrimitiveUnTyped;
mod member_reference;
pub use member_reference::MemberReference;
mod object_null;
pub use object_null::ObjectNull;
mod message_end;
pub use message_end::MessageEnd;
mod binary_library;
pub use binary_library::BinaryLibrary;
mod object_null_multiple_256;
pub use object_null_multiple_256::ObjectNullMultiple256;
mod object_null_multiple;
pub use object_null_multiple::ObjectNullMultiple;
mod array_single_primitive;
pub use array_single_primitive::ArraySinglePrimitive;
mod array_single_object;
pub use array_single_object::ArraySingleObject;
mod array_single_string;
pub use array_single_string::ArraySingleString;
mod binary_method_call;
pub use binary_method_call::BinaryMethodCall;
mod binary_method_return;
pub use binary_method_return::BinaryMethodReturn;
mod value_with_code;
pub use value_with_code::ValueWithCode;
mod string_value_with_code;
pub use string_value_with_code::StringValueWithCode;
mod message_flags;
pub use message_flags::MessageFlags;
mod array_of_value_with_code;
pub use array_of_value_with_code::ArrayOfValueWithCode;

/// 2.1.2.1 `RecordTypeEnumeration`
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum RecordType {
  SerializedStreamHeader         = 0,
  ClassWithId                    = 1,
  SystemClassWithMembers         = 2,
  ClassWithMembers               = 3,
  SystemClassWithMembersAndTypes = 4,
  ClassWithMembersAndTypes       = 5,
  BinaryObjectString             = 6,
  BinaryArray                    = 7,
  MemberPrimitiveTyped           = 8,
  MemberReference                = 9,
  ObjectNull                     = 10,
  MessageEnd                     = 11,
  BinaryLibrary                  = 12,
  ObjectNullMultiple256          = 13,
  ObjectNullMultiple             = 14,
  ArraySinglePrimitive           = 15,
  ArraySingleObject              = 16,
  ArraySingleString              = 17,
  MethodCall                     = 21,
  MethodReturn                   = 22,
}

impl RecordType {
  fn parse(self, input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    value(self, tag([self as u8]))(input)
      .map_err(|err| err.map(|err: nom::error::Error<&[u8]>| error_position!(err.input, ExpectedRecordType(self))))
  }

  pub(crate) fn description(&self) -> &'static str {
    match self {
      Self::SerializedStreamHeader => "a SerializedStreamHeader",
      Self::ClassWithId => "a ClassWithId",
      Self::SystemClassWithMembers => "a SystemClassWithMembers",
      Self::ClassWithMembers => "a ClassWithMembers",
      Self::SystemClassWithMembersAndTypes => "a SystemClassWithMembersAndTypes",
      Self::ClassWithMembersAndTypes => "a ClassWithMembersAndTypes",
      Self::BinaryObjectString => "a BinaryObjectString",
      Self::BinaryArray => "a BinaryArray",
      Self::MemberPrimitiveTyped => "a MemberPrimitiveTyped",
      Self::MemberReference => "a MemberReference",
      Self::ObjectNull => "an ObjectNull",
      Self::MessageEnd => "a MessageEnd",
      Self::BinaryLibrary => "a BinaryLibrary",
      Self::ObjectNullMultiple256 => "an ObjectNullMultiple256",
      Self::ObjectNullMultiple => "an ObjectNullMultiple",
      Self::ArraySinglePrimitive => "an ArraySinglePrimitive",
      Self::ArraySingleObject => "an ArraySingleObject",
      Self::ArraySingleString => "an ArraySingleString",
      Self::MethodCall => "a MethodCall",
      Self::MethodReturn => "a MethodReturn",
    }
  }
}

impl<I, E> Parser<I, Self, E> for RecordType
where
  I: InputTake + Compare<[u8; 1]>,
  E: ParseError<I>,
{
  fn parse(&mut self, input: I) -> IResult<I, Self, E> {
    value(*self, tag([*self as u8]))(input)
  }
}
