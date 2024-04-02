use std::collections::{BTreeMap, HashMap};

use const_str::concat_bytes;
use nrbf::{
  data_type::Int32,
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
        class: "System.Int32",
        library: None,
        members: HashMap::from_iter([("m_value", Value::Primitive(MemberPrimitiveUnTyped::Int32(Int32(-1))))]),
      }),
    )]),
    method_call_or_return: None,
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(INPUT), Ok(([].as_slice(), output)));
}

#[cfg(feature = "serde")]
#[test]
fn int32_deserialize() {
  assert_eq!(nrbf::from_stream(INPUT), Ok(-1));
}
