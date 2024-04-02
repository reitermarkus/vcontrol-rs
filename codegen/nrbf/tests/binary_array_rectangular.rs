use std::collections::{BTreeMap};

use nrbf::{
  data_type::{Int32, Int64},
  grammar::RemotingMessage,
  record::{MemberPrimitiveUnTyped, MessageEnd, SerializationHeader},
  Value,
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
    objects: BTreeMap::from_iter([(
      Int32(1),
      Value::Array(vec![
        Value::Primitive(MemberPrimitiveUnTyped::Int64(Int64(67))),
        Value::Primitive(MemberPrimitiveUnTyped::Int64(Int64(42))),
      ]),
    )]),
    method_call_or_return: None,
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(&input), Ok(([].as_slice(), output)));
}
