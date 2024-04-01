use std::collections::BTreeMap;

use nrbf::{
  data_type::{Int32, LengthPrefixedString},
  grammar::{MethodCallOrReturn, MethodReturn, RemotingMessage},
  method_invocation::{AnyValueWithCode, MessageFlags, StringValueWithCode},
  record::{BinaryMethodReturn, MessageEnd, SerializationHeader},
};

#[test]
fn method_return() {
  #[rustfmt::skip]
  let input = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // ................
    0x00, 0x16, 0x11, 0x08, 0x00, 0x00, 0x12, 0x10, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, // ........Address
    0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x64, 0x0B,                                           // received.
  ];

  let output = RemotingMessage {
    header: SerializationHeader {
      root_id: Int32(0),
      header_id: Int32(0),
      major_version: Int32(1),
      minor_version: Int32(0),
    },
    binary_libraries: BTreeMap::new(),
    pre_method_referenceables: vec![],
    method_call_or_return: Some(MethodCallOrReturn::MethodReturn(MethodReturn {
      binary_method_return: BinaryMethodReturn {
        message_enum: MessageFlags::NO_ARGS | MessageFlags::NO_CONTEXT | MessageFlags::RETURN_VALUE_INLINE,
        return_value: Some(AnyValueWithCode::String(StringValueWithCode::from(LengthPrefixedString::from(
          "Address received",
        )))),
        call_context: None,
        args: None,
      },
      return_call_array: None,
    })),
    post_method_referenceables: vec![],
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(&input), Ok(([].as_slice(), output)))
}
