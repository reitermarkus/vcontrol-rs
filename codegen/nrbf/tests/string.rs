use std::collections::BTreeMap;

use const_str::concat_bytes;
use nrbf::{
  binary_parser::Object,
  data_type::Int32,
  grammar::RemotingMessage,
  record::{MessageEnd, SerializationHeader},
};

#[test]
fn string_empty() {
  #[rustfmt::skip]
  let input = concat_bytes!(
    0,
      1i32.to_le_bytes(),
      b"\xFF\xFF\xFF\xFF",
      b"\x01\x00\x00\x00",
      b"\x00\x00\x00\x00",
    6,
      b"\x01\x00\x00\x00",
      0,
        "",
    11,
  );

  let output = RemotingMessage {
    header: SerializationHeader {
      root_id: Int32(1),
      header_id: Int32(-1),
      major_version: Int32(1),
      minor_version: Int32(0),
    },
    objects: BTreeMap::from_iter([(Int32(1), Object::String(""))]),
    method_call_or_return: None,
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(input), Ok(([].as_slice(), output)));
}

#[rustfmt::skip]
const INPUT: &[u8] = concat_bytes!(
  0,
    b"\x01\x00\x00\x00",
    b"\xFF\xFF\xFF\xFF",
    b"\x01\x00\x00\x00",
    b"\x00\x00\x00\x00",
  6,
    b"\x01\x00\x00\x00",
    17,
      "This is a string.",
  11,
);

#[test]
fn string() {
  let output = RemotingMessage {
    header: SerializationHeader {
      root_id: Int32(1),
      header_id: Int32(-1),
      major_version: Int32(1),
      minor_version: Int32(0),
    },
    objects: BTreeMap::from_iter([(Int32(1), Object::String("This is a string."))]),
    method_call_or_return: None,
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(INPUT), Ok(([].as_slice(), output)));
}

#[cfg(feature = "serde")]
#[test]
fn string_deserialize() {
  assert_eq!(nrbf::from_stream(INPUT), Ok("This is a string."));
  assert_eq!(nrbf::from_stream(INPUT), Ok(String::from("This is a string.")));
}
