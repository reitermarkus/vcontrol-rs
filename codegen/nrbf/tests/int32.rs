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
    12, "System.Int32",
    b"\x01\x00\x00\x00",
    7, "m_value",
    0,
    8,
    b"\xFF\xFF\xFF\xFF",
  11
);

#[test]
fn int32() {
  let output = RemotingMessage::Value(Value::Object(Object {
    class: "System.Int32",
    library: None,
    members: HashMap::from_iter([("m_value", Value::Int32(-1))]),
  }));

  assert_eq!(RemotingMessage::parse(INPUT), Ok(output));
}

#[cfg(feature = "serde")]
#[test]
fn int32_deserialize() {
  use serde::Deserialize;

  assert_eq!(nrbf::from_slice(INPUT), Ok(-1));

  #[derive(Deserialize)]
  struct Int32 {
    pub m_value: i32,
  }

  assert_eq!(nrbf::from_slice::<Int32>(INPUT).map(|v| v.m_value), Ok(-1));
}
