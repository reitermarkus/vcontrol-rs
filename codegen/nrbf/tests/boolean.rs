use std::collections::HashMap;

use const_str::concat_bytes;
use nrbf::{value::Object, RemotingMessage, Value};

#[rustfmt::skip]
const INPUT: &[u8] = concat_bytes!(
  0,
    b"\x01\x00\x00\x00",
    b"\xFF\xFF\xFF\xFF",
    b"\x01\x00\x00\x00",
    b"\x00\x00\x00\x00",
  4,
    b"\x01\x00\x00\x00",
    14, "System.Boolean",
    b"\x01\x00\x00\x00",
    7, "m_value",
    0,
    1,
    0x01,
  11
);

#[test]
fn boolean() {
  let output = RemotingMessage::Value(Value::Object(Object {
    class: "System.Boolean",
    library: None,
    members: HashMap::from_iter([("m_value", Value::Boolean(true))]),
  }));

  assert_eq!(RemotingMessage::parse(INPUT), Ok(output));
}

#[cfg(feature = "serde")]
#[test]
fn int32_deserialize() {
  assert_eq!(nrbf::from_slice(INPUT), Ok(true));
}
