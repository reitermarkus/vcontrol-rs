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
  let output = RemotingMessage {
    root_object: Value::Ref(Int32(1)),
    objects: BTreeMap::from_iter([(
      Int32(1),
      Value::Object(Object {
        class: "System.Double",
        library: None,
        members: HashMap::from_iter([("m_value", Value::Double(-0.1067))]),
      }),
    )]),
    method_call_or_return: None,
  };

  assert_eq!(RemotingMessage::parse(INPUT), Ok(([].as_slice(), output)));
}

#[cfg(feature = "serde")]
#[test]
fn double_deserialize() {
  assert_eq!(nrbf::from_slice(INPUT), Ok(-0.1067));
}
