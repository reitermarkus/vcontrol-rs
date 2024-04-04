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
    12, "System.SByte",
    b"\x01\x00\x00\x00",
    7, "m_value",
    0,
    10,
    0x81,
  11
);

#[test]
fn sbyte() {
  let output = RemotingMessage::Value(Value::Object(Object {
    class: "System.SByte",
    library: None,
    members: HashMap::from_iter([("m_value", Value::SByte(-127))]),
  }));

  assert_eq!(RemotingMessage::parse(INPUT), Ok(output));
}

#[cfg(feature = "serde")]
#[test]
fn sbyte_deserialize() {
  use serde::Deserialize;

  assert_eq!(nrbf::from_slice(INPUT), Ok(-127));

  #[derive(Deserialize)]
  struct SByte {
    pub m_value: i8,
  }

  assert_eq!(nrbf::from_slice::<SByte>(INPUT).map(|v| v.m_value), Ok(-127));
}
