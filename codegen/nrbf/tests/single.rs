#![allow(clippy::approx_constant)]

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
    13, "System.Single",
    b"\x01\x00\x00\x00",
    7, "m_value",
    0,
    11,
    b"\xc3\xf5\x48\x40",
  11
);

#[test]
fn single() {
  let output = RemotingMessage::Value(Value::Object(Object {
    class: "System.Single",
    library: None,
    members: HashMap::from_iter([("m_value", Value::Single(3.14))]),
  }));

  assert_eq!(RemotingMessage::parse(INPUT), Ok(output));
}

#[cfg(feature = "serde")]
#[test]
fn single_deserialize() {
  assert_eq!(nrbf::from_slice(INPUT), Ok(3.14f32));
}
