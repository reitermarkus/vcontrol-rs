use nrbf::{
  common::ArrayInfo,
  data_type::{Int32, Int64},
  grammar::{Array, Arrays, Referenceable},
  parse,
  record::{ArraySinglePrimitive, MemberPrimitiveUnTyped},
  Record,
};

#[test]
fn array_single_primitive() {
  #[rustfmt::skip]
  let input = [
    0,
      1, 0, 0, 0,
      255, 255, 255, 255,
      1, 0, 0, 0,
      0, 0, 0, 0,
    15,
      1, 0, 0, 0,
      2, 0, 0, 0,
      9,
      67, 0, 0, 0, 0, 0, 0, 0,
      42, 0, 0, 0, 0, 0, 0, 0,
    11,
  ];

  let output = vec![Record::Referenceable(Referenceable::Arrays(Arrays {
    binary_library: None,
    array: Array::ArraySinglePrimitive(ArraySinglePrimitive {
      array_info: ArrayInfo { object_id: Int32(1), length: Int32(2) },
      members: vec![MemberPrimitiveUnTyped::Int64(Int64(67)), MemberPrimitiveUnTyped::Int64(Int64(42))],
    }),
  }))];

  assert_eq!(parse(&input), Ok(([].as_slice(), output)));
}
