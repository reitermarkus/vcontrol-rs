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
    11, "System.Byte",
    b"\x01\x00\x00\x00",
    7, "m_value",
    0,
    2,
    0x81,
  11
);

#[test]
fn byte() {
  let output = RemotingMessage::Value(Value::Object(Object {
    class: "System.Byte",
    library: None,
    members: HashMap::from_iter([("m_value", Value::Byte(129))]),
  }));

  assert_eq!(RemotingMessage::parse(INPUT), Ok(output));
}

#[cfg(feature = "serde")]
#[test]
fn byte_deserialize() {
  use serde::Deserialize;

  assert_eq!(nrbf::from_slice(INPUT), Ok(129));

  #[derive(Deserialize)]
  struct Byte {
    pub m_value: u8,
  }

  assert_eq!(nrbf::from_slice::<Byte>(INPUT).map(|v| v.m_value), Ok(129));
}
