use std::collections::BTreeMap;

use nrbf::{
  data_type::Int32,
  grammar::RemotingMessage,
  record::{MemberPrimitiveUnTyped, MessageEnd, SerializationHeader},
  Value,
};

#[test]
fn binary_array_jagged() {
  #[rustfmt::skip]
  let input = [
    0,
      0x01, 0x00, 0x00, 0x00,
      0xFF, 0xFF, 0xFF, 0xFF,
      0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    7,
      0x01, 0x00, 0x00, 0x00,
      1,
      0x01, 0x00, 0x00, 0x00,
      0x03, 0x00, 0x00, 0x00,
      7,
      8,
      9,
        0x02, 0x00, 0x00, 0x00,
      9,
        0x03, 0x00, 0x00, 0x00,
      9,
        0x04, 0x00, 0x00, 0x00,
    15,
      0x02, 0x00, 0x00, 0x00,
      0x02, 0x00, 0x00, 0x00,
      8,
      0x01, 0x00, 0x00, 0x00,
      0x02, 0x00, 0x00, 0x00,
    15,
      0x03, 0x00, 0x00, 0x00,
      0x03, 0x00, 0x00, 0x00,
      8,
      0x03, 0x00, 0x00, 0x00,
      0x04, 0x00, 0x00, 0x00,
      0x05, 0x00, 0x00, 0x00,
    15,
      0x04, 0x00, 0x00, 0x00,
      0x04, 0x00, 0x00, 0x00,
      8,
      0x06, 0x00, 0x00, 0x00,
      0x07, 0x00, 0x00, 0x00,
      0x08, 0x00, 0x00, 0x00,
      0x09, 0x00, 0x00, 0x00,
    11,
  ];

  let output = RemotingMessage {
    header: SerializationHeader {
      root_id: Int32(1),
      header_id: Int32(-1),
      major_version: Int32(1),
      minor_version: Int32(0),
    },
    objects: BTreeMap::from_iter([
      (Int32(1), Value::Array(vec![Value::Ref(Int32(2)), Value::Ref(Int32(3)), Value::Ref(Int32(4))])),
      (
        Int32(2),
        Value::Array(vec![
          Value::Primitive(MemberPrimitiveUnTyped::Int32(Int32(1))),
          Value::Primitive(MemberPrimitiveUnTyped::Int32(Int32(2))),
        ]),
      ),
      (
        Int32(3),
        Value::Array(vec![
          Value::Primitive(MemberPrimitiveUnTyped::Int32(Int32(3))),
          Value::Primitive(MemberPrimitiveUnTyped::Int32(Int32(4))),
          Value::Primitive(MemberPrimitiveUnTyped::Int32(Int32(5))),
        ]),
      ),
      (
        Int32(4),
        Value::Array(vec![
          Value::Primitive(MemberPrimitiveUnTyped::Int32(Int32(6))),
          Value::Primitive(MemberPrimitiveUnTyped::Int32(Int32(7))),
          Value::Primitive(MemberPrimitiveUnTyped::Int32(Int32(8))),
          Value::Primitive(MemberPrimitiveUnTyped::Int32(Int32(9))),
        ]),
      ),
    ]),
    method_call_or_return: None,
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(&input), Ok(([].as_slice(), output)));
}
