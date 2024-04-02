use std::collections::{BTreeMap, HashMap};

use const_str::concat_bytes;
use nrbf::{
  data_type::Int32,
  value::Object,
  RemotingMessage, Value,
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
    root_object: Value::Ref(Int32(1)),
    objects: BTreeMap::from_iter([
      (
        Int32(1),
        Value::Object(Object {
          class: "System.Collections.Generic.List`1[[System.String, mscorlib, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]]", library: None,
          members: HashMap::from_iter([
            ("_items", Value::Ref(Int32(2))),
            ("_size", Value::Int32(2)),
            ("_version", Value::Int32(2)),
          ]),
        }),
      ),
      (
        Int32(2),
        Value::Array(vec![
          Value::Ref(Int32(3)),
          Value::Ref(Int32(4)),
          Value::Null(2),
        ]),
      ),
      (Int32(3), Value::String("Bob")),
      (Int32(4), Value::String("Rob")),
    ]),
    method_call_or_return: None,
  };

  assert_eq!(RemotingMessage::parse(input), Ok(([].as_slice(), output)));
}
