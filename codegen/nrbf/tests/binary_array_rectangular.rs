use nrbf::{
  common::AdditionalTypeInfo,
  data_type::{Int32, Int64},
  enumeration::{BinaryArrayType, BinaryType, PrimitiveType},
  grammar::{Array, Arrays, MemberReference2, MemberReferenceInner, Referenceable},
  parse,
  record::{BinaryArray, MemberPrimitiveUnTyped},
  Record,
};

#[test]
fn binary_array_rectangular() {
  #[rustfmt::skip]
  let input = [
    0,
      1, 0, 0, 0,
      255, 255, 255, 255,
      1, 0, 0, 0,
      0, 0, 0, 0,
    7,
      1, 0, 0, 0,
      2,
      2, 0, 0, 0,
      1, 0, 0, 0,
      2, 0, 0, 0,
      0,
      9,
      67, 0, 0, 0, 0, 0, 0, 0,
      42, 0, 0, 0, 0, 0, 0, 0,
    11,
  ];

  assert_eq!(
    parse(&input),
    Ok((
      [].as_slice(),
      vec![Record::Referenceable(Referenceable::Arrays(Arrays {
        binary_library: None,
        array: Array::BinaryArray(BinaryArray {
          object_id: Int32(1),
          binary_array_type_enum: BinaryArrayType::Rectangular,
          rank: Int32(2),
          lengths: vec![Int32(1), Int32(2)],
          lower_bounds: None,
          type_enum: BinaryType::Primitive,
          additional_type_info: Some(AdditionalTypeInfo::Primitive(PrimitiveType::Int64)),
          members: vec![
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int64(Int64(67))),
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int64(Int64(42)))
            },
          ],
        }),
      })),]
    ))
  );
}
