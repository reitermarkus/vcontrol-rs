use nrbf::{
  data_type::{Int32, LengthPrefixedString},
  enumeration::{BinaryType, PrimitiveType},
  method_invocation::{BinaryMethodCall, MessageFlags, MethodCallArray, StringValueWithCode},
  parse, AdditionalTypeInfo, Array, ArrayInfo, ArraySingleObject, ArraySingleString, Arrays, BinaryLibrary,
  BinaryObjectString, CallArray, Class, ClassInfo, ClassWithMembersAndTypes, Classes, MemberPrimitiveUnTyped,
  MemberReference, MemberReference2, MemberReference3, MemberTypeInfo, MethodCall, Record, Referenceable,
  SystemClassWithMembersAndTypes,
};

#[test]
fn array_of_customers() {
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
            MemberReference3::BinaryObjectString(BinaryObjectString {
              object_id: 2,
              value: LengthPrefixedString::from("Bob"),
            }),
            MemberReference3::BinaryObjectString(BinaryObjectString {
              object_id: 3,
              value: LengthPrefixedString::from("Rob"),
            }),
          ],
        }),
      }))],
    )),
  );
}
