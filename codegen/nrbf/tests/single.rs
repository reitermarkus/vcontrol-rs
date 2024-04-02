use std::collections::{BTreeMap, HashMap};

use const_str::concat_bytes;
use nrbf::{
  data_type::{Int32, Single},
  grammar::RemotingMessage,
  record::{MemberPrimitiveUnTyped, MessageEnd, SerializationHeader},
  value::Object,
  Value,
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
  let output = RemotingMessage {
    header: SerializationHeader {
      root_id: Int32(1),
      header_id: Int32(-1),
      major_version: Int32(1),
      minor_version: Int32(0),
    },
    objects: BTreeMap::from_iter([(
      Int32(1),
      Value::Object(Object {
        class: "System.Single",
        library: None,
        members: HashMap::from_iter([("m_value", Value::Primitive(MemberPrimitiveUnTyped::Single(Single(3.14))))]),
      }),
    )]),
    method_call_or_return: None,
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(INPUT), Ok(([].as_slice(), output)));
}

#[cfg(feature = "serde")]
#[test]
fn single_deserialize() {
  assert_eq!(nrbf::from_stream(INPUT), Ok(3.14f32));
}
