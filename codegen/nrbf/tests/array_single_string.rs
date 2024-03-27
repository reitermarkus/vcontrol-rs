use nrbf::{
  common::ArrayInfo,
  data_type::{Int32, LengthPrefixedString},
  grammar::{Array, Arrays, MemberReferenceInner, Referenceable},
  parse,
  record::{ArraySingleString, BinaryObjectString},
  Record,
};

#[test]
fn array_single_string() {
  #[rustfmt::skip]
  let input = [
    0,
      1, 0, 0, 0,
      255, 255, 255, 255,
      1, 0, 0, 0,
      0, 0, 0, 0,
    17,
      1, 0, 0, 0,
      2, 0, 0, 0,
      6,
        2, 0, 0, 0,
        3, 66, 111, 98,
      6,
        3, 0, 0, 0,
        3, 82, 111, 98,
    11,
  ];

  assert_eq!(
    parse(&input),
    Ok((
      [].as_slice(),
      vec![Record::Referenceable(Referenceable::Arrays(Arrays {
        binary_library: None,
        array: Array::ArraySingleString(ArraySingleString {
          array_info: ArrayInfo { object_id: Int32(1), length: Int32(2) },
          members: vec![
            MemberReferenceInner::BinaryObjectString(BinaryObjectString {
              object_id: Int32(2),
              value: LengthPrefixedString::from("Bob"),
            }),
            MemberReferenceInner::BinaryObjectString(BinaryObjectString {
              object_id: Int32(3),
              value: LengthPrefixedString::from("Rob"),
            }),
          ],
        }),
      }))],
    )),
  );
}
