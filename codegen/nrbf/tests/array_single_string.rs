use const_str::concat_bytes;
use nrbf::{
  common::ArrayInfo,
  data_type::{Int32, LengthPrefixedString},
  grammar::{Array, Arrays, MemberReferenceInner, Referenceable, RemotingMessage},
  record::{ArraySingleString, BinaryObjectString, MessageEnd, SerializationHeader},
};

#[test]
fn array_single_string() {
  #[rustfmt::skip]
  let input = concat_bytes!(
    0,
      0x01, 0x00, 0x00, 0x00,
      0xFF, 0xFF, 0xFF, 0xFF,
      0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    17,
      0x01, 0x00, 0x00, 0x00,
      0x02, 0x00, 0x00, 0x00,
      6,
        0x02, 0x00, 0x00, 0x00,
        3, "Bob",
      6,
        0x03, 0x00, 0x00, 0x00,
        3, "Rob",
    11,
  );

  let output = RemotingMessage {
    header: SerializationHeader {
      root_id: Int32(1),
      header_id: Int32(-1),
      major_version: Int32(1),
      minor_version: Int32(0),
    },
    pre_method_referenceables: vec![Referenceable::Arrays(Arrays {
      binary_library: None,
      array: Array::ArraySingleString(ArraySingleString {
        array_info: ArrayInfo { object_id: Int32(1), length: Int32(2) },
        members: vec![
          MemberReferenceInner::BinaryObjectString(BinaryObjectString {
            object_id: Int32(2),
            value: LengthPrefixedString::from("Bob"),
          }),
          MemberReferenceInner::BinaryObjectString(BinaryObjectString {
            object_id: Int32(3),
            value: LengthPrefixedString::from("Rob"),
          }),
        ],
      }),
    })],
    method_call_or_return: None,
    post_method_referenceables: vec![],
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(input), Ok(([].as_slice(), output)));
}
