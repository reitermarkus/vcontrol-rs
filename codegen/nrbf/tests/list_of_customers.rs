use nrbf::{
  data_type::{Byte, Int32, LengthPrefixedString},
  enumeration::{BinaryType, PrimitiveType},
  method_invocation::{BinaryMethodCall, MessageFlags, MethodCallArray, StringValueWithCode},
  parse, AdditionalTypeInfo, Array, ArrayInfo, ArraySingleObject, ArraySingleString, Arrays, BinaryLibrary,
  BinaryObjectString, CallArray, Class, ClassInfo, ClassWithMembersAndTypes, Classes, MemberPrimitiveUnTyped,
  MemberReference, MemberReference2, MemberReference3, MemberTypeInfo, MethodCall, NullObject, ObjectNullMultiple256,
  Record, Referenceable, SystemClassWithMembersAndTypes,
};

#[test]
fn list_of_customers() {
  #[rustfmt::skip]
  let input = [
    0,
      1, 0, 0, 0,
      255, 255, 255, 255,
      1, 0, 0, 0,
      0, 0, 0, 0,
      4,
        1, 0, 0, 0,
        127,
          83, 121, 115, 116, 101, 109, 46, 67, 111, 108, 108, 101,
          99, 116, 105, 111, 110, 115, 46, 71, 101, 110, 101, 114,
          105, 99, 46, 76, 105, 115, 116, 96, 49, 91, 91, 83, 121,
          115, 116, 101, 109, 46, 83, 116, 114, 105, 110, 103, 44,
          32, 109, 115, 99, 111, 114, 108, 105, 98, 44, 32, 86, 101,
          114, 115, 105, 111, 110, 61, 52, 46, 48, 46, 48, 46, 48,
          44, 32, 67, 117, 108, 116, 117, 114, 101, 61, 110, 101, 117,
          116, 114, 97, 108, 44, 32, 80, 117, 98, 108, 105, 99, 75,
          101, 121, 84, 111, 107, 101, 110, 61, 98, 55, 55, 97, 53,
          99, 53, 54, 49, 57, 51, 52, 101, 48, 56, 57, 93, 93,
        3, 0, 0, 0,
        6,
          95, 105, 116, 101, 109, 115,
        5,
          95, 115, 105, 122, 101,
        8,
          95, 118, 101, 114, 115, 105, 111, 110,
      6, 0, 0,
      8, 8,
      9,
        2, 0, 0, 0,
      2, 0, 0, 0,
      2, 0, 0, 0,
    17,
      2, 0, 0, 0,
      4, 0, 0, 0,
    6,
      3, 0, 0, 0,
      3,
        66, 111, 98,
    6,
      4, 0, 0, 0,
      3,
        82, 111, 98,
    13,
      2,
    11
  ];

  assert_eq!(parse(&input), Ok(([].as_slice(), vec![
    Record::Referenceable(Referenceable::Classes(Classes {
      binary_library: None,
      class: Class::SystemClassWithMembersAndTypes(SystemClassWithMembersAndTypes {
        class_info: ClassInfo {
          object_id: 1,
          name: LengthPrefixedString::from(
            "System.Collections.Generic.List`1[[System.String, mscorlib, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089]]"
          ),
          member_names: vec![
            LengthPrefixedString::from("_items"),
            LengthPrefixedString::from("_size"),
            LengthPrefixedString::from("_version"),
          ],
        },
        member_type_info: MemberTypeInfo {
          binary_type_enums: vec![
            BinaryType::StringArray,
            BinaryType::Primitive,
            BinaryType::Primitive,
          ],
          additional_infos: vec![
            None,
            Some(AdditionalTypeInfo::Primitive(PrimitiveType::Int32)),
            Some(AdditionalTypeInfo::Primitive(PrimitiveType::Int32)),
          ],
        },
      }),
      member_references: vec![
        MemberReference2 {
          binary_library: None,
          member_reference: MemberReference3::MemberReference(MemberReference { id_ref: Int32(2) })
        },
        MemberReference2 {
          binary_library: None,
          member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(2))),
        },
        MemberReference2 {
          binary_library: None,
          member_reference: MemberReference3::MemberPrimitiveUnTyped(MemberPrimitiveUnTyped::Int32(Int32(2))),
        },
      ],
    })),
    Record::Referenceable(Referenceable::Arrays(Arrays {
      binary_library: None,
      array: Array::ArraySingleString(ArraySingleString {
        array_info: ArrayInfo {
          object_id: Int32(2),
          length: Int32(4),
        },
        members: vec![
          MemberReference3::BinaryObjectString(BinaryObjectString {
            object_id: 3,
            value: LengthPrefixedString::from("Bob"),
          }),
          MemberReference3::BinaryObjectString(BinaryObjectString {
            object_id: 4,
            value: LengthPrefixedString::from("Rob"),
          }),
          MemberReference3::NullObject(NullObject::ObjectNullMultiple256(
            ObjectNullMultiple256 {
              null_count: Byte(2),
            },
          )),
        ],
      }),
    })),
  ])));
}
