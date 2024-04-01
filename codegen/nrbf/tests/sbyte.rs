use std::collections::BTreeMap;

use const_str::concat_bytes;
use nrbf::{
  common::{AdditionalTypeInfo, ClassInfo, MemberTypeInfo},
  data_type::{Int32, Int8, LengthPrefixedString},
  enumeration::{BinaryType, PrimitiveType},
  grammar::{Class, Classes, MemberReference2, MemberReferenceInner, Referenceable, RemotingMessage},
  record::{MemberPrimitiveUnTyped, MessageEnd, SerializationHeader, SystemClassWithMembersAndTypes},
};

#[rustfmt::skip]
const INPUT: &[u8] = concat_bytes!(
  0,
    b"\x01\x00\x00\x00",
    b"\xFF\xFF\xFF\xFF",
    b"\x01\x00\x00\x00",
    b"\x00\x00\x00\x00",
  4,
    b"\x01\x00\x00\x00",
    12, "System.SByte",
    b"\x01\x00\x00\x00",
    7, "m_value",
    0,
    10,
    0x81,
  11
);

#[test]
fn int16() {
  let output = RemotingMessage {
    header: SerializationHeader {
      root_id: Int32(1),
      header_id: Int32(-1),
      major_version: Int32(1),
      minor_version: Int32(0),
    },
    binary_libraries: BTreeMap::new(),
    classes: BTreeMap::from_iter([(
      Int32(1),
      Class::SystemClassWithMembersAndTypes(SystemClassWithMembersAndTypes {
        class_info: ClassInfo {
          object_id: Int32(1),
          name: LengthPrefixedString::from("System.SByte"),
          member_names: vec![LengthPrefixedString::from("m_value")],
        },
        member_type_info: MemberTypeInfo {
          binary_type_enums: vec![BinaryType::Primitive],
          additional_infos: vec![Some(AdditionalTypeInfo::Primitive(PrimitiveType::SByte))],
        },
      }),
    )]),
    pre_method_referenceables: vec![Referenceable::Classes(Classes {
      class: Class::SystemClassWithMembersAndTypes(SystemClassWithMembersAndTypes {
        class_info: ClassInfo {
          object_id: Int32(1),
          name: LengthPrefixedString::from("System.SByte"),
          member_names: vec![LengthPrefixedString::from("m_value")],
        },
        member_type_info: MemberTypeInfo {
          binary_type_enums: vec![BinaryType::Primitive],
          additional_infos: vec![Some(AdditionalTypeInfo::Primitive(PrimitiveType::SByte))],
        },
      }),
      member_references: vec![MemberReference2 {
        member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::SByte(Int8(-127))),
      }],
    })],
    method_call_or_return: None,
    post_method_referenceables: vec![],
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(INPUT), Ok(([].as_slice(), output)));
}

#[cfg(feature = "serde")]
#[test]
fn int16_deserialize() {
  assert_eq!(nrbf::from_stream(INPUT), Ok(-127));
}
