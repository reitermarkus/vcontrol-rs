use std::collections::BTreeMap;

use nrbf::{
  common::AdditionalTypeInfo,
  data_type::{Int32, Int64},
  enumeration::{BinaryArrayType, BinaryType, PrimitiveType},
  grammar::{Array, Arrays, MemberReference2, MemberReferenceInner, Referenceable, RemotingMessage},
  record::{BinaryArray, MemberPrimitiveUnTyped, MessageEnd, SerializationHeader},
};

#[test]
fn binary_array_rectangular() {
  #[rustfmt::skip]
  let input = [
    0,
      0x01, 0x00, 0x00, 0x00,
      0xFF, 0xFF, 0xFF, 0xFF,
      0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    7,
      0x01, 0x00, 0x00, 0x00,
      2,
      0x02, 0x00, 0x00, 0x00,
      0x01, 0x00, 0x00, 0x00,
      0x02, 0x00, 0x00, 0x00,
      0,
      9,
      67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    11,
  ];

  let output = RemotingMessage {
    header: SerializationHeader {
      root_id: Int32(1),
      header_id: Int32(-1),
      major_version: Int32(1),
      minor_version: Int32(0),
    },
    binary_libraries: BTreeMap::new(),
    classes: BTreeMap::new(),
    pre_method_referenceables: vec![Referenceable::Arrays(Arrays {
      array: Array::BinaryArray(BinaryArray {
        object_id: Int32(1),
        binary_array_type_enum: BinaryArrayType::Rectangular,
        rank: Int32(2),
        lengths: vec![Int32(1), Int32(2)],
        lower_bounds: None,
        type_enum: BinaryType::Primitive,
        additional_type_info: Some(AdditionalTypeInfo::Primitive(PrimitiveType::Int64)),
        members: vec![
          MemberReference2 {
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int64(Int64(67))),
          },
          MemberReference2 {
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int64(Int64(42))),
          },
        ],
      }),
    })],
    method_call_or_return: None,
    post_method_referenceables: vec![],
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(&input), Ok(([].as_slice(), output)));
}
