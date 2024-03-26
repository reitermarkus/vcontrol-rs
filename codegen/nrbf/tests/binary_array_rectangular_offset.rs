use nrbf::{
  data_type::Int32,
  enumeration::{BinaryType, PrimitiveType},
  parse, AdditionalTypeInfo, Array, Arrays, BinaryArray, BinaryArrayType, MemberPrimitiveUnTyped, MemberReference2,
  MemberReference3, Record, Referenceable,
};

#[test]
fn binary_array_rectangular_offset() {
  #[rustfmt::skip]
  let input = [
    0,
      1, 0, 0, 0,
      255, 255, 255, 255,
      1, 0, 0, 0,
      0, 0, 0, 0,
    7,
      1, 0, 0, 0,
      5,
      2, 0, 0, 0,
      10, 0, 0, 0,
      4, 0, 0, 0,
      208, 7, 0, 0,
      1, 0, 0, 0,
      0,
      8,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
      0, 0, 0, 0,
    11
  ];

  assert_eq!(
    parse(&input),
    Ok((
      [].as_slice(),
      vec![Record::Referenceable(Referenceable::Arrays(Arrays {
        binary_library: None,
        array: Array::BinaryArray(BinaryArray {
          object_id: Int32(1),
          binary_array_type_enum: BinaryArrayType::RectangularOffset,
          rank: Int32(2),
          lengths: vec![Int32(10), Int32(4)],
          lower_bounds: Some(vec![Int32(2000), Int32(1)]),
          type_enum: BinaryType::Primitive,
          additional_type_info: Some(AdditionalTypeInfo::Primitive(PrimitiveType::Int32)),
          members: vec![
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            },
            MemberReference2 {
              binary_library: None,
              member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(0)))
            }
          ]
        })
      }))]
    ))
  );
}
