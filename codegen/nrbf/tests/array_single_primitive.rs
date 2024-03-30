use nrbf::{
  common::ArrayInfo,
  data_type::{Int32, Int64},
  grammar::{Array, Arrays, Referenceable, RemotingMessage},
  record::{ArraySinglePrimitive, MemberPrimitiveUnTyped, MessageEnd, SerializationHeader},
};

#[test]
fn array_single_primitive() {
  #[rustfmt::skip]
  let input = [
    0,
      0x01, 0x00, 0x00, 0x00,
      0xFF, 0xFF, 0xFF, 0xFF,
      0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    15,
      0x01, 0x00, 0x00, 0x00,
      0x02, 0x00, 0x00, 0x00,
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
    pre_method_referenceables: vec![Referenceable::Arrays(Arrays {
      binary_library: None,
      array: Array::ArraySinglePrimitive(ArraySinglePrimitive {
        array_info: ArrayInfo { object_id: Int32(1), length: Int32(2) },
        members: vec![MemberPrimitiveUnTyped::Int64(Int64(67)), MemberPrimitiveUnTyped::Int64(Int64(42))],
      }),
    })],
    method_call_or_return: None,
    post_method_referenceables: vec![],
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(&input), Ok(([].as_slice(), output)));
}
