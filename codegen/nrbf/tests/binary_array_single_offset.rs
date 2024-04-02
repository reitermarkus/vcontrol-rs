use std::collections::BTreeMap;

use nrbf::{
  data_type::Int32,
  grammar::RemotingMessage,
  record::{MessageEnd, SerializationHeader},
  Value,
};

#[test]
fn binary_array_single_offset() {
  #[rustfmt::skip]
  let input = [
    0,
      0x01, 0x00, 0x00, 0x00,
      0xFF, 0xFF, 0xFF, 0xFF,
      0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    7,
      0x01, 0x00, 0x00, 0x00,
      3,
      0x01, 0x00, 0x00, 0x00,
      0x0A, 0x00, 0x00, 0x00,
      0xD0, 0x07, 0x00, 0x00,
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
    11
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
        Value::Int32(0),
        Value::Int32(0),
        Value::Int32(0),
        Value::Int32(0),
        Value::Int32(0),
        Value::Int32(0),
        Value::Int32(0),
        Value::Int32(0),
        Value::Int32(0),
        Value::Int32(0),
      ]),
    )]),
    method_call_or_return: None,
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(&input), Ok(([].as_slice(), output)));
}
