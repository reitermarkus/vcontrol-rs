use const_str::concat_bytes;
use nrbf::{
  common::{AdditionalTypeInfo, ClassInfo, MemberTypeInfo},
  data_type::{Int32, LengthPrefixedString},
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
    12, "System.Int32",
    b"\x01\x00\x00\x00",
    7, "m_value",
    0,
    8,
    b"\xFF\xFF\xFF\xFF",
  11
);

#[test]
fn int32() {
  let output = RemotingMessage {
    header: SerializationHeader {
      root_id: Int32(1),
      header_id: Int32(-1),
      major_version: Int32(1),
      minor_version: Int32(0),
    },
    pre_method_referenceables: vec![Referenceable::Classes(Classes {
      binary_library: None,
      class: Class::SystemClassWithMembersAndTypes(SystemClassWithMembersAndTypes {
        class_info: ClassInfo {
          object_id: Int32(1),
          name: LengthPrefixedString::from("System.Int32"),
          member_names: vec![LengthPrefixedString::from("m_value")],
        },
        member_type_info: MemberTypeInfo {
          binary_type_enums: vec![BinaryType::Primitive],
          additional_infos: vec![Some(AdditionalTypeInfo::Primitive(PrimitiveType::Int32))],
        },
      }),
      member_references: vec![MemberReference2 {
        binary_library: None,
        member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(-1))),
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
fn int32_deserialize() {
  assert_eq!(nrbf::from_stream(INPUT), Ok(-1));
}