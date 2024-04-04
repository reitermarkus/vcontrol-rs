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
    13, "System.Double",
    b"\x01\x00\x00\x00",
    7, "m_value",
    0,
    6,
    b"\xFF\xB2{\xF2\xB0P\xBB\xBF",
  11
);

#[test]
fn double() {
  let output = RemotingMessage::Value(Value::Object(Object {
    class: "System.Double",
    library: None,
    members: HashMap::from_iter([("m_value", Value::Double(-0.1067))]),
  }));

  assert_eq!(RemotingMessage::parse(INPUT), Ok(output));
}

#[cfg(feature = "serde")]
#[test]
fn double_deserialize() {
  use serde::Deserialize;

  assert_eq!(nrbf::from_slice(INPUT), Ok(-0.1067));

  #[derive(Deserialize)]
  struct Double {
    pub m_value: f64,
  }

  assert_eq!(nrbf::from_slice::<Double>(INPUT).map(|v| v.m_value), Ok(-0.1067));
}
