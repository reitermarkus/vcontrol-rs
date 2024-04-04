use const_str::concat_bytes;
use nrbf::{RemotingMessage, Value};

#[rustfmt::skip]
const INPUT: &[u8] = concat_bytes!(
  0,
    b"\x01\x00\x00\x00",
    b"\xFF\xFF\xFF\xFF",
    b"\x01\x00\x00\x00",
    b"\x00\x00\x00\x00",
  15,
    b"\x01\x00\x00\x00",
    b"\x02\x00\x00\x00",
    9,
    b"\x43\x00\x00\x00\x00\x00\x00\x00",
    b"\x2a\x00\x00\x00\x00\x00\x00\x00",
  11,
);

#[test]
fn array_single_primitive() {
  let output = RemotingMessage::Value(Value::Array(vec![Value::Int64(67), Value::Int64(42)]));

  assert_eq!(RemotingMessage::parse(INPUT), Ok(output));
}

#[cfg(feature = "serde")]
#[test]
fn array_single_primitive_deserialize() {
  assert_eq!(nrbf::from_slice(INPUT), Ok(vec![67i64, 42i64]));
  assert_eq!(nrbf::from_slice(INPUT), Ok(vec![67i32, 42i32]));

  assert_eq!(
    nrbf::from_slice::<[i64; 1]>(INPUT).unwrap_err().to_string(),
    "invalid length 2, expected 1 element in array"
  );
  assert_eq!(nrbf::from_slice::<[i64; 2]>(INPUT), Ok([67, 42]));
  assert_eq!(
    nrbf::from_slice::<[i64; 3]>(INPUT).unwrap_err().to_string(),
    "invalid length 2, expected an array of length 3"
  );
}
