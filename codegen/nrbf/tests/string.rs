use const_str::concat_bytes;
use nrbf::{RemotingMessage, Value};

#[test]
fn string_empty() {
  #[rustfmt::skip]
  let input = concat_bytes!(
    0,
      1i32.to_le_bytes(),
      b"\xFF\xFF\xFF\xFF",
      b"\x01\x00\x00\x00",
      b"\x00\x00\x00\x00",
    6,
      b"\x01\x00\x00\x00",
      0, "",
    11,
  );

  let output = RemotingMessage::Value(Value::String(""));

  assert_eq!(RemotingMessage::parse(input), Ok(output));
}

#[rustfmt::skip]
const INPUT: &[u8] = concat_bytes!(
  0,
    b"\x01\x00\x00\x00",
    b"\xFF\xFF\xFF\xFF",
    b"\x01\x00\x00\x00",
    b"\x00\x00\x00\x00",
  6,
    b"\x01\x00\x00\x00",
    17, "This is a string.",
  11,
);

#[test]
fn string() {
  let output = RemotingMessage::Value(Value::String("This is a string."));

  assert_eq!(RemotingMessage::parse(INPUT), Ok(output));
}

#[cfg(feature = "serde")]
#[test]
fn string_deserialize() {
  assert_eq!(nrbf::from_slice(INPUT), Ok("This is a string."));
  assert_eq!(nrbf::from_slice(INPUT), Ok(String::from("This is a string.")));
}
