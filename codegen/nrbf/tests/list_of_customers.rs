use std::collections::BTreeMap;

use const_str::concat_bytes;
use nrbf::{
  binary_parser::{Object, ObjectClass},
  data_type::Int32,
  grammar::RemotingMessage,
  record::{MemberPrimitiveUnTyped, MessageEnd, SerializationHeader},
};

#[test]
fn list_of_customers() {
  #[rustfmt::skip]
  let input = concat_bytes!(
    0,
      0x01, 0x00, 0x00, 0x00,
      0xFF, 0xFF, 0xFF, 0xFF,
      0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      4,
        0x01, 0x00, 0x00, 0x00,
        127, "System.Collections.Generic.List`1[[System.String, mscorlib, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]]",
        0x03, 0x00, 0x00, 0x00,
        6, "_items",
        5, "_size",
        8, "_version",
      6, 0, 0,
      8, 8,
      9,
        0x02, 0x00, 0x00, 0x00,
      0x02, 0x00, 0x00, 0x00,
      0x02, 0x00, 0x00, 0x00,
    17,
      0x02, 0x00, 0x00, 0x00,
      0x04, 0x00, 0x00, 0x00,
    6,
      0x03, 0x00, 0x00, 0x00,
      3, "Bob",
    6,
      0x04, 0x00, 0x00, 0x00,
      3, "Rob",
    13,
      2,
    11
  );

  let output = RemotingMessage {
    header: SerializationHeader {
      root_id: Int32(1),
      header_id: Int32(-1),
      major_version: Int32(1),
      minor_version: Int32(0),
    },
    objects: BTreeMap::from_iter([
      (
        Int32(1),
        Object::Object {
          class: ObjectClass { name: "System.Collections.Generic.List`1[[System.String, mscorlib, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]]", library: None },
          members: BTreeMap::from_iter([
            ("_items", Object::Ref(Int32(2))),
            ("_size", Object::Primitive(MemberPrimitiveUnTyped::Int32(Int32(2)))),
            ("_version", Object::Primitive(MemberPrimitiveUnTyped::Int32(Int32(2)))),
          ]),
        },
      ),
      (
        Int32(2),
        Object::Array(vec![
          Object::String("Bob"),
          Object::String("Rob"),
          Object::Null(2),
        ]),
      ),
      (Int32(3), Object::String("Bob")),
      (Int32(4), Object::String("Rob")),
    ]),
    method_call_or_return: None,
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(input), Ok(([].as_slice(), output)));
}
