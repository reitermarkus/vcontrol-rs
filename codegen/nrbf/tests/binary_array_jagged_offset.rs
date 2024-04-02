use std::collections::BTreeMap;

use nrbf::{
  binary_parser::Object,
  common::{AdditionalTypeInfo, ArrayInfo},
  data_type::Int32,
  enumeration::{BinaryArrayType, BinaryType, PrimitiveType},
  grammar::{Array, Arrays, MemberReferenceInner, Referenceable, RemotingMessage},
  record::{
    ArraySinglePrimitive, BinaryArray, MemberPrimitiveUnTyped, MemberReference, MessageEnd, SerializationHeader,
  },
};

#[test]
fn binary_array_jagged_offset() {
  #[rustfmt::skip]
  let input = [
    0,
      0x01, 0x00, 0x00, 0x00,
      0xFF, 0xFF, 0xFF, 0xFF,
      0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
    7,
      0x01, 0x00, 0x00, 0x00,
      4,
      0x01, 0x00, 0x00, 0x00,
      0x03, 0x00, 0x00, 0x00,
      208, 7, 0, 0,
      7,
      8,
      9,
        0x02, 0x00, 0x00, 0x00,
      9,
        0x03, 0x00, 0x00, 0x00,
      9,
        0x04, 0x00, 0x00, 0x00,
    15,
      0x02, 0x00, 0x00, 0x00,
      0x01, 0x00, 0x00, 0x00,
      8,
      0x01, 0x00, 0x00, 0x00,
    15,
      0x03, 0x00, 0x00, 0x00,
      0x02, 0x00, 0x00, 0x00,
      8,
      0x02, 0x00, 0x00, 0x00,
      0x03, 0x00, 0x00, 0x00,
    15,
      0x04, 0x00, 0x00, 0x00,
      0x03, 0x00, 0x00, 0x00,
      8,
      0x04, 0x00, 0x00, 0x00,
      0x05, 0x00, 0x00, 0x00,
      0x06, 0x00, 0x00, 0x00,
    11,
  ];

  let output = RemotingMessage {
    header: SerializationHeader {
      root_id: Int32(1),
      header_id: Int32(-1),
      major_version: Int32(1),
      minor_version: Int32(0),
    },
    binary_libraries: BTreeMap::new(),
    classes: BTreeMap::new(),
    pre_method_referenceables: vec![
      Referenceable::Arrays(Arrays {
        array: Array::BinaryArray(Int32(1), vec![Object::Ref(Int32(2)), Object::Ref(Int32(3)), Object::Ref(Int32(4))]),
      }),
      Referenceable::Arrays(Arrays {
        array: Array::ArraySinglePrimitive(Int32(2), vec![Object::Primitive(MemberPrimitiveUnTyped::Int32(Int32(1)))]),
      }),
      Referenceable::Arrays(Arrays {
        array: Array::ArraySinglePrimitive(
          Int32(3),
          vec![
            Object::Primitive(MemberPrimitiveUnTyped::Int32(Int32(2))),
            Object::Primitive(MemberPrimitiveUnTyped::Int32(Int32(3))),
          ],
        ),
      }),
      Referenceable::Arrays(Arrays {
        array: Array::ArraySinglePrimitive(
          Int32(4),
          vec![
            Object::Primitive(MemberPrimitiveUnTyped::Int32(Int32(4))),
            Object::Primitive(MemberPrimitiveUnTyped::Int32(Int32(5))),
            Object::Primitive(MemberPrimitiveUnTyped::Int32(Int32(6))),
          ],
        ),
      }),
    ],
    method_call_or_return: None,
    post_method_referenceables: vec![],
    end: MessageEnd,
  };

  assert_eq!(RemotingMessage::parse(&input), Ok(([].as_slice(), output)));
}
