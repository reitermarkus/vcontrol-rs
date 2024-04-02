use std::collections::BTreeMap;

use nrbf::{
  common::{AdditionalTypeInfo, ArrayInfo},
  data_type::Int32,
  enumeration::{BinaryArrayType, BinaryType, PrimitiveType},
  grammar::{Array, Arrays, MemberReferenceInner, Referenceable, RemotingMessage},
  record::{
    ArraySinglePrimitive, BinaryArray, MemberPrimitiveUnTyped, MemberReference, MessageEnd, SerializationHeader,
  },
};

#[test]
fn binary_array_jagged_offset() {
  #[rustfmt::skip]
  let input = [
    0,
      0x01, 0x00, 0x00, 0x00,
      0xFF, 0xFF, 0xFF, 0xFF,
      0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    7,
      0x01, 0x00, 0x00, 0x00,
      4,
      0x01, 0x00, 0x00, 0x00,
      0x03, 0x00, 0x00, 0x00,
      208, 7, 0, 0,
      7,
      8,
      9,
        0x02, 0x00, 0x00, 0x00,
      9,
        0x03, 0x00, 0x00, 0x00,
      9,
        0x04, 0x00, 0x00, 0x00,
    15,
      0x02, 0x00, 0x00, 0x00,
      0x01, 0x00, 0x00, 0x00,
      8,
      0x01, 0x00, 0x00, 0x00,
    15,
      0x03, 0x00, 0x00, 0x00,
      0x02, 0x00, 0x00, 0x00,
      8,
      0x02, 0x00, 0x00, 0x00,
      0x03, 0x00, 0x00, 0x00,
    15,
      0x04, 0x00, 0x00, 0x00,
      0x03, 0x00, 0x00, 0x00,
      8,
      0x04, 0x00, 0x00, 0x00,
      0x05, 0x00, 0x00, 0x00,
      0x06, 0x00, 0x00, 0x00,
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
    pre_method_referenceables: vec![
      Referenceable::Arrays(Arrays {
        array: Array::BinaryArray(BinaryArray {
          object_id: Int32(1),
          binary_array_type_enum: BinaryArrayType::JaggedOffset,
          rank: Int32(1),
          lengths: vec![Int32(3)],
          lower_bounds: Some(vec![Int32(2000)]),
          type_enum: BinaryType::PrimitiveArray,
          additional_type_info: Some(AdditionalTypeInfo::Primitive(PrimitiveType::Int32)),
          members: vec![
            MemberReferenceInner::MemberReference(MemberReference { id_ref: Int32(2) }),
            MemberReferenceInner::MemberReference(MemberReference { id_ref: Int32(3) }),
            MemberReferenceInner::MemberReference(MemberReference { id_ref: Int32(4) }),
          ],
        }),
      }),
      Referenceable::Arrays(Arrays {
        array: Array::ArraySinglePrimitive(ArraySinglePrimitive {
          array_info: ArrayInfo { object_id: Int32(2), length: Int32(1) },
          members: vec![MemberPrimitiveUnTyped::Int32(Int32(1))],
        }),
      }),
      Referenceable::Arrays(Arrays {
        array: Array::ArraySinglePrimitive(ArraySinglePrimitive {
          array_info: ArrayInfo { object_id: Int32(3), length: Int32(2) },
          members: vec![MemberPrimitiveUnTyped::Int32(Int32(2)), MemberPrimitiveUnTyped::Int32(Int32(3))],
        }),
      }),
      Referenceable::Arrays(Arrays {
        array: Array::ArraySinglePrimitive(ArraySinglePrimitive {
          array_info: ArrayInfo { object_id: Int32(4), length: Int32(3) },
          members: vec![
            MemberPrimitiveUnTyped::Int32(Int32(4)),
            MemberPrimitiveUnTyped::Int32(Int32(5)),
            MemberPrimitiveUnTyped::Int32(Int32(6)),
          ],
        }),
      }),
    ],
    method_call_or_return: None,
    post_method_referenceables: vec![],
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(&input), Ok(([].as_slice(), output)));
}
