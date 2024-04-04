use std::collections::HashMap;

use const_str::concat_bytes;
use nrbf::{value::Object, RemotingMessage, Value};

#[rustfmt::skip]
const INPUT: &[u8] = concat_bytes!(
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

#[test]
fn list_of_customers() {
  let output = RemotingMessage::Value(
    Value::Object(Object {
      class: "System.Collections.Generic.List`1[[System.String, mscorlib, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]]", library: None,
      members: HashMap::from_iter([
        (
          "_items",
          Value::Array(vec![
            Value::String("Bob"),
            Value::String("Rob"),
            Value::Null,
            Value::Null,
          ]),
        ),
        ("_size", Value::Int32(2)),
        ("_version", Value::Int32(2)),
      ]),
    }),
  );

  assert_eq!(RemotingMessage::parse(INPUT), Ok(output));
}

#[cfg(feature = "serde")]
#[test]
fn list_of_customers_deserialize() {
  use serde::Deserialize;

  assert_eq!(nrbf::from_slice(INPUT), Ok(["Bob", "Rob"]));
  assert_eq!(nrbf::from_slice(INPUT), Ok(vec!["Bob", "Rob"]));

  #[derive(Debug, Deserialize, PartialEq)]
  struct List {
    pub _items: Vec<Option<String>>,
    pub _size: i32,
    pub _version: i32,
  }

  assert_eq!(
    nrbf::from_slice(INPUT),
    Ok(List { _items: vec![Some("Bob".into()), Some("Rob".into()), None, None], _size: 2, _version: 2 })
  );
}
