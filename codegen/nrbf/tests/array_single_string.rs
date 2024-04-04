use const_str::concat_bytes;
use nrbf::{RemotingMessage, Value};

#[rustfmt::skip]
const INPUT: &[u8] = concat_bytes!(
  0,
    0x01, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF,
    0x01, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
  17,
    0x01, 0x00, 0x00, 0x00,
    0x02, 0x00, 0x00, 0x00,
  6,
    0x02, 0x00, 0x00, 0x00,
    3, "Bob",
  6,
    0x03, 0x00, 0x00, 0x00,
    3, "Rob",
  11,
);

#[test]
fn array_single_string() {
  let output = RemotingMessage::Value(Value::Array(vec![Value::String("Bob"), Value::String("Rob")]));

  assert_eq!(RemotingMessage::parse(INPUT), Ok(output));
}

#[cfg(feature = "serde")]
#[test]
fn array_single_string_deserialize() {
  assert_eq!(nrbf::from_slice(INPUT), Ok(["Bob", "Rob"]));
  assert_eq!(nrbf::from_slice(INPUT), Ok(vec![String::from("Bob"), String::from("Rob")]));
}
