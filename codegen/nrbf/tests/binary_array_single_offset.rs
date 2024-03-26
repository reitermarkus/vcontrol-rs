use nrbf::{
  data_type::{Byte, Int32, Int64, LengthPrefixedString},
  enumeration::{BinaryType, PrimitiveType},
  method_invocation::{BinaryMethodCall, MessageFlags, MethodCallArray, StringValueWithCode},
  parse, AdditionalTypeInfo, Array, ArrayInfo, ArraySingleObject, ArraySinglePrimitive, ArraySingleString, Arrays,
  BinaryArray, BinaryArrayType, BinaryLibrary, BinaryObjectString, CallArray, Class, ClassInfo,
  ClassWithMembersAndTypes, Classes, MemberPrimitiveUnTyped, MemberReference, MemberReference2, MemberReference3,
  MemberTypeInfo, MethodCall, NullObject, ObjectNullMultiple256, Record, Referenceable, SystemClassWithMembersAndTypes,
};

#[test]
fn binary_array_single_offset() {
  #[rustfmt::skip]
  let input = [
    0,
      1, 0, 0, 0,
      255, 255, 255, 255,
      1, 0, 0, 0,
      0, 0, 0, 0,
    7,
      1, 0, 0, 0,
      3,
      1, 0, 0, 0,
      10, 0, 0, 0,
      208, 7, 0, 0,
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
          binary_array_type_enum: BinaryArrayType::SingleOffset,
          rank: Int32(1),
          lengths: vec![Int32(10)],
          lower_bounds: Some(vec![Int32(2000)]),
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
            }
          ]
        })
      }))]
    ))
  );
}
