use std::collections::{BTreeMap, HashMap};

use const_str::concat_bytes;
use nrbf::{data_type::Int32, value::Object, RemotingMessage, Value};

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
fn int16() {
  let output = RemotingMessage::Value(
    BTreeMap::from_iter([(
      Int32(1),
      Value::Object(Object {
        class: "System.SByte",
        library: None,
        members: HashMap::from_iter([("m_value", Value::SByte(-127))]),
      }),
    )]),
    Value::Ref(Int32(1)),
  );

  assert_eq!(RemotingMessage::parse(INPUT), Ok(([].as_slice(), output)));
}

#[cfg(feature = "serde")]
#[test]
fn int16_deserialize() {
  assert_eq!(nrbf::from_slice(INPUT), Ok(-127));
}
