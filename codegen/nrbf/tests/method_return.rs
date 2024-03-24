use nrbf::{
  data_type::{Int32, LengthPrefixedString},
  enumeration::BinaryType,
  method_invocation::{AnyValueWithCode, BinaryMethodReturn, MessageFlags, MethodCallArray, StringValueWithCode},
  parse, ArrayInfo, ArraySingleObject, BinaryLibrary, BinaryObjectString, CallArray, Class, ClassInfo,
  ClassWithMembersAndTypes, Classes, MemberReference, MemberReference2, MemberReference3, MemberTypeInfo, MethodCall,
  MethodReturn, Record, Referenceable,
};

#[test]
fn method_return() {
  #[rustfmt::skip]
  let input = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // ................
    0x00, 0x16, 0x11, 0x08, 0x00, 0x00, 0x12, 0x10, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, // ........Address
    0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x64, 0x0B,                                           // received.
  ];

  assert_eq!(
    parse(&input),
    Ok((
      [].as_slice(),
      vec![Record::MethodReturn(MethodReturn {
        binary_library: None,
        binary_method_return: BinaryMethodReturn {
          message_enum: MessageFlags::NO_ARGS | MessageFlags::NO_CONTEXT | MessageFlags::RETURN_VALUE_INLINE,
          return_value: Some(AnyValueWithCode::String(StringValueWithCode::from(LengthPrefixedString::from(
            "Address received"
          )),),),
          call_context: None,
          args: None,
        },
        return_call_array: None,
      })],
    ))
  )
}
