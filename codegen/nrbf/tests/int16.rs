use std::collections::BTreeMap;

use const_str::concat_bytes;
use nrbf::{
  binary_parser::{Object, ObjectClass},
  data_type::{Int16, Int32},
  grammar::RemotingMessage,
  record::{MemberPrimitiveUnTyped, MessageEnd, SerializationHeader},
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
    12, "System.Int16",
    b"\x01\x00\x00\x00",
    7, "m_value",
    0,
    b"\x07\x70\xff",
  11
);

#[test]
fn int16() {
  let output = RemotingMessage {
    header: SerializationHeader {
      root_id: Int32(1),
      header_id: Int32(-1),
      major_version: Int32(1),
      minor_version: Int32(0),
    },
    objects: BTreeMap::from_iter([(
      Int32(1),
      Object::Object {
        class: ObjectClass { name: "System.Int16", library: None },
        members: BTreeMap::from_iter([("m_value", Object::Primitive(MemberPrimitiveUnTyped::Int16(Int16(-144))))]),
      },
    )]),
    method_call_or_return: None,
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(INPUT), Ok(([].as_slice(), output)));
}

#[cfg(feature = "serde")]
#[test]
fn int16_deserialize() {
  assert_eq!(nrbf::from_stream(INPUT), Ok(-144));
}
