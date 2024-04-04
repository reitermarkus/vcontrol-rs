use std::collections::HashMap;

use const_str::concat_bytes;
use nrbf::{
  value::{Object, Value},
  RemotingMessage,
};

#[rustfmt::skip]
const INPUT: &[u8] = concat_bytes!(
  0,
    b"\x01\x00\x00\x00",
    b"\xFF\xFF\xFF\xFF",
    b"\x01\x00\x00\x00",
    b"\x00\x00\x00\x00",
  4,
    b"\x01\x00\x00\x00",
    13, "System.UInt64",
    b"\x01\x00\x00\x00",
    7, "m_value",
    0,
    16,
    b"\x70\xff\x00\x00\x00\x00\x00\x00",
  11
);

#[test]
fn uint32() {
  let output = RemotingMessage::Value(Value::Object(Object {
    class: "System.UInt64",
    library: None,
    members: HashMap::from_iter([("m_value", Value::UInt64(65392))]),
  }));

  assert_eq!(RemotingMessage::parse(INPUT), Ok(output));
}

#[cfg(feature = "serde")]
#[test]
fn uint32_deserialize() {
  use serde::Deserialize;

  assert_eq!(nrbf::from_slice(INPUT), Ok(65392));

  #[derive(Deserialize)]
  struct UInt64 {
    pub m_value: u64,
  }

  assert_eq!(nrbf::from_slice::<UInt64>(INPUT).map(|v| v.m_value), Ok(65392));
}
