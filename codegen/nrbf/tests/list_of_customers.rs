use std::collections::BTreeMap;

use const_str::concat_bytes;
use nrbf::{
  common::{AdditionalTypeInfo, ArrayInfo, ClassInfo, MemberTypeInfo},
  data_type::{Byte, Int32, LengthPrefixedString},
  enumeration::{BinaryType, PrimitiveType},
  grammar::{
    Array, Arrays, Class, Classes, MemberReference2, MemberReferenceInner, NullObject, Referenceable, RemotingMessage,
  },
  record::{
    ArraySingleString, BinaryObjectString, MemberPrimitiveUnTyped, MemberReference, MessageEnd, ObjectNullMultiple256,
    SerializationHeader, SystemClassWithMembersAndTypes,
  },
};

#[test]
fn list_of_customers() {
  #[rustfmt::skip]
  let input = concat_bytes!(
    0,
      0x01, 0x00, 0x00, 0x00,
      0xFF, 0xFF, 0xFF, 0xFF,
      0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      4,
        0x01, 0x00, 0x00, 0x00,
        127, "System.Collections.Generic.List`1[[System.String, mscorlib, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]]",
        0x03, 0x00, 0x00, 0x00,
        6, "_items",
        5, "_size",
        8, "_version",
      6, 0, 0,
      8, 8,
      9,
        0x02, 0x00, 0x00, 0x00,
      0x02, 0x00, 0x00, 0x00,
      0x02, 0x00, 0x00, 0x00,
    17,
      0x02, 0x00, 0x00, 0x00,
      0x04, 0x00, 0x00, 0x00,
    6,
      0x03, 0x00, 0x00, 0x00,
      3, "Bob",
    6,
      0x04, 0x00, 0x00, 0x00,
      3, "Rob",
    13,
      2,
    11
  );

  let output = RemotingMessage {
    header: SerializationHeader {
      root_id: Int32(1),
      header_id: Int32(-1),
      major_version: Int32(1),
      minor_version: Int32(0),
    },
    binary_libraries: BTreeMap::new(),
    pre_method_referenceables: vec![
      Referenceable::Classes(Classes {
        class: Class::SystemClassWithMembersAndTypes(SystemClassWithMembersAndTypes {
          class_info: ClassInfo {
            object_id: Int32(1),
            name: LengthPrefixedString::from(
              "System.Collections.Generic.List`1[[System.String, mscorlib, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]]"
            ),
            member_names: vec![
              LengthPrefixedString::from("_items"),
              LengthPrefixedString::from("_size"),
              LengthPrefixedString::from("_version"),
            ],
          },
          member_type_info: MemberTypeInfo {
            binary_type_enums: vec![
              BinaryType::StringArray,
              BinaryType::Primitive,
              BinaryType::Primitive,
            ],
            additional_infos: vec![
              None,
              Some(AdditionalTypeInfo::Primitive(PrimitiveType::Int32)),
              Some(AdditionalTypeInfo::Primitive(PrimitiveType::Int32)),
            ],
          },
        }),
        member_references: vec![
          MemberReference2 {
            member_reference: MemberReferenceInner::MemberReference(MemberReference { id_ref: Int32(2) })
          },
          MemberReference2 {
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(2))),
          },
          MemberReference2 {
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(2))),
          },
        ],
      }),
      Referenceable::Arrays(Arrays {
        array: Array::ArraySingleString(ArraySingleString {
          array_info: ArrayInfo {
            object_id: Int32(2),
            length: Int32(4),
          },
          members: vec![
            MemberReferenceInner::BinaryObjectString(BinaryObjectString {
              object_id: Int32(3),
              value: LengthPrefixedString::from("Bob"),
            }),
            MemberReferenceInner::BinaryObjectString(BinaryObjectString {
              object_id: Int32(4),
              value: LengthPrefixedString::from("Rob"),
            }),
            MemberReferenceInner::NullObject(NullObject::ObjectNullMultiple256(
              ObjectNullMultiple256 {
                null_count: Byte(2),
              },
            )),
          ],
        }),
      }),
    ],
    method_call_or_return: None,
    post_method_referenceables: vec![],
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(input), Ok(([].as_slice(), output)));
}
