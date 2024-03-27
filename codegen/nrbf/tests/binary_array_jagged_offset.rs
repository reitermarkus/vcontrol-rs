use nrbf::{
  common::{AdditionalTypeInfo, ArrayInfo},
  data_type::Int32,
  enumeration::{BinaryArrayType, BinaryType, PrimitiveType},
  grammar::{Array, Arrays, MemberReference2, MemberReferenceInner, Referenceable},
  parse,
  record::{ArraySinglePrimitive, BinaryArray, MemberPrimitiveUnTyped, MemberReference},
  Record,
};

#[test]
fn binary_array_jagged_offset() {
  #[rustfmt::skip]
  let input = [
    0,
      1, 0, 0, 0,
      255, 255, 255, 255,
      1, 0, 0, 0,
      0, 0, 0, 0,
    7,
      1, 0, 0, 0,
      4,
      1, 0, 0, 0,
      3, 0, 0, 0,
      208, 7, 0, 0,
      7,
      8,
      9,
        2, 0, 0, 0,
      9,
        3, 0, 0, 0,
      9,
        4, 0, 0, 0,
    15,
      2, 0, 0, 0,
      1, 0, 0, 0,
      8,
      1, 0, 0, 0,
    15,
      3, 0, 0, 0,
      2, 0, 0, 0,
      8,
      2, 0, 0, 0,
      3, 0, 0, 0,
    15,
      4, 0, 0, 0,
      3, 0, 0, 0,
      8,
      4, 0, 0, 0,
      5, 0, 0, 0,
      6, 0, 0, 0,
    11,
  ];

  assert_eq!(
    dbg!(parse(&input)),
    Ok((
      [].as_slice(),
      vec![
        Record::Referenceable(Referenceable::Arrays(Arrays {
          binary_library: None,
          array: Array::BinaryArray(BinaryArray {
            object_id: Int32(1),
            binary_array_type_enum: BinaryArrayType::JaggedOffset,
            rank: Int32(1),
            lengths: vec![Int32(3)],
            lower_bounds: Some(vec![Int32(2000)]),
            type_enum: BinaryType::PrimitiveArray,
            additional_type_info: Some(AdditionalTypeInfo::Primitive(PrimitiveType::Int32)),
            members: vec![
              MemberReference2 {
                binary_library: None,
                member_reference: MemberReferenceInner::MemberReference(MemberReference { id_ref: Int32(2) }),
              },
              MemberReference2 {
                binary_library: None,
                member_reference: MemberReferenceInner::MemberReference(MemberReference { id_ref: Int32(3) }),
              },
              MemberReference2 {
                binary_library: None,
                member_reference: MemberReferenceInner::MemberReference(MemberReference { id_ref: Int32(4) }),
              },
            ],
          }),
        })),
        Record::Referenceable(Referenceable::Arrays(Arrays {
          binary_library: None,
          array: Array::ArraySinglePrimitive(ArraySinglePrimitive {
            array_info: ArrayInfo { object_id: Int32(2), length: Int32(1) },
            members: vec![MemberPrimitiveUnTyped::Int32(Int32(1))]
          }),
        })),
        Record::Referenceable(Referenceable::Arrays(Arrays {
          binary_library: None,
          array: Array::ArraySinglePrimitive(ArraySinglePrimitive {
            array_info: ArrayInfo { object_id: Int32(3), length: Int32(2) },
            members: vec![MemberPrimitiveUnTyped::Int32(Int32(2)), MemberPrimitiveUnTyped::Int32(Int32(3))],
          }),
        })),
        Record::Referenceable(Referenceable::Arrays(Arrays {
          binary_library: None,
          array: Array::ArraySinglePrimitive(ArraySinglePrimitive {
            array_info: ArrayInfo { object_id: Int32(4), length: Int32(3) },
            members: vec![
              MemberPrimitiveUnTyped::Int32(Int32(4)),
              MemberPrimitiveUnTyped::Int32(Int32(5)),
              MemberPrimitiveUnTyped::Int32(Int32(6)),
            ],
          }),
        })),
      ]
    ))
  );
}
