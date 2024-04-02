use std::collections::BTreeMap;

use const_str::concat_bytes;
use nrbf::{
  common::{AdditionalTypeInfo, ClassInfo, MemberTypeInfo},
  data_type::{Int32, LengthPrefixedString, Single},
  enumeration::{BinaryType, PrimitiveType},
  grammar::{Class, Classes, MemberReferenceInner, Referenceable, RemotingMessage},
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
    13, "System.Single",
    b"\x01\x00\x00\x00",
    7, "m_value",
    0,
    11,
    b"\xc3\xf5\x48\x40",
  11
);

#[test]
fn single() {
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
          name: LengthPrefixedString::from("System.Single"),
          member_names: vec![LengthPrefixedString::from("m_value")],
        },
        member_type_info: MemberTypeInfo {
          binary_type_enums: vec![BinaryType::Primitive],
          additional_infos: vec![Some(AdditionalTypeInfo::Primitive(PrimitiveType::Single))],
        },
      }),
    )]),
    pre_method_referenceables: vec![Referenceable::Classes(Classes {
      class_id: Int32(1),
      member_references: vec![MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Single(Single(
        3.14,
      )))],
    })],
    method_call_or_return: None,
    post_method_referenceables: vec![],
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(INPUT), Ok(([].as_slice(), output)));
}

#[cfg(feature = "serde")]
#[test]
fn single_deserialize() {
  assert_eq!(nrbf::from_stream(INPUT), Ok(3.14f32));
}
