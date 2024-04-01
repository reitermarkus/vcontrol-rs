use std::collections::BTreeMap;

use nrbf::{
  common::{ArrayInfo, ClassInfo, MemberTypeInfo},
  data_type::{Int32, LengthPrefixedString},
  enumeration::BinaryType,
  grammar::{
    CallArray, Class, Classes, MemberReference2, MemberReferenceInner, MethodCall, MethodCallOrReturn, Referenceable,
    RemotingMessage,
  },
  method_invocation::{MessageFlags, StringValueWithCode},
  record::{
    ArraySingleObject, BinaryLibrary, BinaryMethodCall, BinaryObjectString, ClassWithMembersAndTypes, MemberReference,
    MessageEnd, MethodCallArray, SerializationHeader,
  },
};

#[test]
fn method_call() {
  #[rustfmt::skip]
  let input = [
    0x00, 0x01, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // .....ÿÿÿÿ.......
    0x00, 0x15, 0x14, 0x00, 0x00, 0x00, 0x12, 0x0B, 0x53, 0x65, 0x6E, 0x64, 0x41, 0x64, 0x64, 0x72, // ........SendAddr
    0x65, 0x73, 0x73, 0x12, 0x6F, 0x44, 0x4F, 0x4A, 0x52, 0x65, 0x6D, 0x6F, 0x74, 0x69, 0x6E, 0x67, // ess.oDOJRemoting
    0x4D, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2E, 0x4D, 0x79, 0x53, 0x65, 0x72, 0x76, 0x65, // Metadata.MyServe
    0x72, 0x2C, 0x20, 0x44, 0x4F, 0x4A, 0x52, 0x65, 0x6D, 0x6F, 0x74, 0x69, 0x6E, 0x67, 0x4D, 0x65, // r,DOJRemotingMe
    0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2C, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, // tadata,Version=
    0x31, 0x2E, 0x30, 0x2E, 0x32, 0x36, 0x32, 0x32, 0x2E, 0x33, 0x31, 0x33, 0x32, 0x36, 0x2C, 0x20, // 1.0.2622.31326,
    0x43, 0x75, 0x6C, 0x74, 0x75, 0x72, 0x65, 0x3D, 0x6E, 0x65, 0x75, 0x74, 0x72, 0x61, 0x6C, 0x2C, // Culture=neutral,
    0x20, 0x50, 0x75, 0x62, 0x6C, 0x69, 0x63, 0x4B, 0x65, 0x79, 0x54, 0x6F, 0x6B, 0x65, 0x6E, 0x3D, // PublicKeyToken=
    0x6E, 0x75, 0x6C, 0x6C, 0x10, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x09, 0x02, 0x00, // null............
    0x00, 0x00, 0x0C, 0x03, 0x00, 0x00, 0x00, 0x51, 0x44, 0x4F, 0x4A, 0x52, 0x65, 0x6D, 0x6F, 0x74, // .......QDOJRemot
    0x69, 0x6E, 0x67, 0x4D, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2C, 0x20, 0x56, 0x65, 0x72, // ingMetadata,Ver
    0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x31, 0x2E, 0x30, 0x2E, 0x32, 0x36, 0x32, 0x32, 0x2E, 0x33, 0x31, // sion=1.0.2622.31
    0x33, 0x32, 0x36, 0x2C, 0x20, 0x43, 0x75, 0x6C, 0x74, 0x75, 0x72, 0x65, 0x3D, 0x6E, 0x65, 0x75, // 326,Culture=neu
    0x74, 0x72, 0x61, 0x6C, 0x2C, 0x20, 0x50, 0x75, 0x62, 0x6C, 0x69, 0x63, 0x4B, 0x65, 0x79, 0x54, // tral,PublicKeyT
    0x6F, 0x6B, 0x65, 0x6E, 0x3D, 0x6E, 0x75, 0x6C, 0x6C, 0x05, 0x02, 0x00, 0x00, 0x00, 0x1B, 0x44, // oken=null......D
    0x4F, 0x4A, 0x52, 0x65, 0x6D, 0x6F, 0x74, 0x69, 0x6E, 0x67, 0x4D, 0x65, 0x74, 0x61, 0x64, 0x61, // OJRemotingMetada
    0x74, 0x61, 0x2E, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x04, 0x00, 0x00, 0x00, 0x06, 0x53, // ta.Address.....S
    0x74, 0x72, 0x65, 0x65, 0x74, 0x04, 0x43, 0x69, 0x74, 0x79, 0x05, 0x53, 0x74, 0x61, 0x74, 0x65, // treet.City.State
    0x03, 0x5A, 0x69, 0x70, 0x01, 0x01, 0x01, 0x01, 0x03, 0x00, 0x00, 0x00, 0x06, 0x04, 0x00, 0x00, // .Zip............
    0x00, 0x11, 0x4F, 0x6E, 0x65, 0x20, 0x4D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F, 0x66, 0x74, 0x20, // ..OneMicrosoft
    0x57, 0x61, 0x79, 0x06, 0x05, 0x00, 0x00, 0x00, 0x07, 0x52, 0x65, 0x64, 0x6D, 0x6F, 0x6E, 0x64, // Way......Redmond
    0x06, 0x06, 0x00, 0x00, 0x00, 0x02, 0x57, 0x41, 0x06, 0x07, 0x00, 0x00, 0x00, 0x05, 0x39, 0x38, // ......WA......98
    0x30, 0x35, 0x34, 0x0B                                                                          // 054.
  ];

  let output = RemotingMessage {
    header: SerializationHeader {
      root_id: Int32(1),
      header_id: Int32(-1),
      major_version: Int32(1),
      minor_version: Int32(0),
    },
    binary_libraries: BTreeMap::from_iter([
      (
        Int32(3),
        LengthPrefixedString::from(
          "DOJRemotingMetadata, Version=1.0.2622.31326, Culture=neutral, PublicKeyToken=null"
        )
      )
    ]),
    classes: BTreeMap::from_iter([
      (
        Int32(2),
        Class::ClassWithMembersAndTypes(
          ClassWithMembersAndTypes {
            class_info: ClassInfo {
              object_id: Int32(2),
              name: LengthPrefixedString::from("DOJRemotingMetadata.Address"),
              member_names: vec![
                LengthPrefixedString::from("Street"),
                LengthPrefixedString::from("City"),
                LengthPrefixedString::from("State"),
                LengthPrefixedString::from("Zip"),
              ],
            },
            member_type_info: MemberTypeInfo {
              binary_type_enums: vec![
                BinaryType::String,
                BinaryType::String,
                BinaryType::String,
                BinaryType::String,
              ],
              additional_infos: vec![
                None,
                None,
                None,
                None,
              ],
            },
            library_id: Int32(3),
          },
        )
      )
    ]),
    pre_method_referenceables: vec![],
    method_call_or_return: Some(MethodCallOrReturn::MethodCall(
      MethodCall {
        binary_method_call: BinaryMethodCall {
          message_enum: MessageFlags::ARGS_IS_ARRAY | MessageFlags::NO_CONTEXT,
          method_name: StringValueWithCode::from(
            LengthPrefixedString::from("SendAddress")
          ),
          type_name: StringValueWithCode::from(
            LengthPrefixedString::from(
              "DOJRemotingMetadata.MyServer, DOJRemotingMetadata, Version=1.0.2622.31326, Culture=neutral, PublicKeyToken=null"
            )
          ),
          call_context: None,
          args: None,
        },
        call_array: Some(CallArray {
          call_array: MethodCallArray(ArraySingleObject {
            array_info: ArrayInfo {
              object_id: Int32(1),
              length: Int32(1),
            },
            member_references: vec![
              MemberReference2 {
                member_reference: MemberReferenceInner::MemberReference(
                  MemberReference {
                    id_ref: Int32(2),
                  },
                ),
              },
            ],
          }),
        }),
      },
    )),
    post_method_referenceables: vec![
      Referenceable::Classes(
        Classes {
          class_id: Int32(2),
          member_references: vec![
            MemberReference2 {
              member_reference: MemberReferenceInner::BinaryObjectString(
                BinaryObjectString {
                  object_id: Int32(4),
                  value: LengthPrefixedString::from("One Microsoft Way"),
                },
              )
            },
            MemberReference2 {
              member_reference: MemberReferenceInner::BinaryObjectString(
                BinaryObjectString {
                  object_id: Int32(5),
                  value: LengthPrefixedString::from("Redmond"),
                },
              )
            },
            MemberReference2 {
              member_reference: MemberReferenceInner::BinaryObjectString(
                BinaryObjectString {
                  object_id: Int32(6),
                  value: LengthPrefixedString::from("WA"),
                },
              )
            },
            MemberReference2 {
              member_reference: MemberReferenceInner::BinaryObjectString(
                BinaryObjectString {
                  object_id: Int32(7),
                  value: LengthPrefixedString::from("98054"),
                },
              )
            },
          ],
        },
      ),
    ],
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(&input), Ok(([].as_slice(), output)))
}
