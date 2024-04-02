use std::collections::{BTreeMap};

use const_str::concat_bytes;
use nrbf::{
  data_type::Int32,
  grammar::RemotingMessage,
  record::{MessageEnd, SerializationHeader},
  Value,
};

#[rustfmt::skip]
const INPUT: &[u8] = concat_bytes!(
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

#[test]
fn array_single_string() {
  let output = RemotingMessage {
    header: SerializationHeader {
      root_id: Int32(1),
      header_id: Int32(-1),
      major_version: Int32(1),
      minor_version: Int32(0),
    },
    objects: BTreeMap::from_iter([
      (Int32(1), Value::Array(vec![Value::Ref(Int32(2)), Value::Ref(Int32(3))])),
      (Int32(2), Value::String("Bob")),
      (Int32(3), Value::String("Rob")),
    ]),
    method_call_or_return: None,
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(INPUT), Ok(([].as_slice(), output)));
}

#[cfg(feature = "serde")]
#[test]
fn array_single_string_deserialize() {
  assert_eq!(nrbf::from_stream(INPUT), Ok(["Bob", "Rob"]));
  assert_eq!(nrbf::from_stream(INPUT), Ok(vec![String::from("Bob"), String::from("Rob")]));
}
