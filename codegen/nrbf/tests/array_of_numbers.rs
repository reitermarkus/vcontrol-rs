use nrbf::{
  data_type::{Byte, Int32, Int64, LengthPrefixedString},
  enumeration::{BinaryType, PrimitiveType},
  method_invocation::{BinaryMethodCall, MessageFlags, MethodCallArray, StringValueWithCode},
  parse, AdditionalTypeInfo, Array, ArrayInfo, ArraySingleObject, ArraySinglePrimitive, ArraySingleString, Arrays,
  BinaryLibrary, BinaryObjectString, CallArray, Class, ClassInfo, ClassWithMembersAndTypes, Classes,
  MemberPrimitiveUnTyped, MemberReference, MemberReference2, MemberReference3, MemberTypeInfo, MethodCall, NullObject,
  ObjectNullMultiple256, Record, Referenceable, SystemClassWithMembersAndTypes,
};

#[test]
fn array_of_numbers() {
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

  assert_eq!(
    parse(&input),
    Ok((
      [].as_slice(),
      vec![Record::Referenceable(Referenceable::Arrays(Arrays {
        binary_library: None,
        array: Array::ArraySinglePrimitive(ArraySinglePrimitive {
          array_info: ArrayInfo { object_id: Int32(1), length: Int32(2) },
          members: vec![MemberPrimitiveUnTyped::Int64(Int64(67)), MemberPrimitiveUnTyped::Int64(Int64(42)),],
        }),
      })),]
    ))
  );
}
