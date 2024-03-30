use nrbf::{
  common::AdditionalTypeInfo,
  data_type::Int32,
  enumeration::{BinaryArrayType, BinaryType, PrimitiveType},
  grammar::{Array, Arrays, MemberReference2, MemberReferenceInner, Referenceable, RemotingMessage},
  record::{BinaryArray, MemberPrimitiveUnTyped, MessageEnd, SerializationHeader},
};

#[test]
fn binary_array_rectangular_offset() {
  #[rustfmt::skip]
  let input = [
    0,
      0x01, 0x00, 0x00, 0x00,
      0xFF, 0xFF, 0xFF, 0xFF,
      0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    7,
      0x01, 0x00, 0x00, 0x00,
      5,
      0x02, 0x00, 0x00, 0x00,
      0x0A, 0x00, 0x00, 0x00,
      0x04, 0x00, 0x00, 0x00,
      208, 7, 0, 0,
      0x01, 0x00, 0x00, 0x00,
      0,
      8,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    11
  ];

  let output = RemotingMessage {
    header: SerializationHeader {
      root_id: Int32(1),
      header_id: Int32(-1),
      major_version: Int32(1),
      minor_version: Int32(0),
    },
    pre_method_referenceables: vec![Referenceable::Arrays(Arrays {
      binary_library: None,
      array: Array::BinaryArray(BinaryArray {
        object_id: Int32(1),
        binary_array_type_enum: BinaryArrayType::RectangularOffset,
        rank: Int32(2),
        lengths: vec![Int32(10), Int32(4)],
        lower_bounds: Some(vec![Int32(2000), Int32(1)]),
        type_enum: BinaryType::Primitive,
        additional_type_info: Some(AdditionalTypeInfo::Primitive(PrimitiveType::Int32)),
        members: vec![
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
          },
          MemberReference2 {
            binary_library: None,
            member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0))),
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
