use nom::{combinator::opt, multi::many_m_n, IResult};

use crate::{
  binary_parser::Object,
  data_type::Int32,
  record::{BinaryMethodCall, MethodCallArray},
  BinaryParser,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CallArray<'i> {
  pub call_array: MethodCallArray,
  pub member_references: Vec<Object<'i>>,
}

impl<'i> CallArray<'i> {
  pub fn parse(input: &'i [u8], parser: &mut BinaryParser<'i>) -> IResult<&'i [u8], Self> {
    let (input, ()) = parser.parse_binary_library(input)?;

    let (input, call_array) = MethodCallArray::parse(input)?;
    let length = call_array.0.array_info.len();
    let (input, member_references) =
      many_m_n(length, length, |input| parser.parse_member_reference(input, None))(input)?;

    Ok((input, Self { call_array, member_references }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.call_array.object_id()
  }
}

/// 2.7 Binary Record Grammar - `methodCall`
#[derive(Debug, Clone, PartialEq)]
pub struct MethodCall<'i> {
  pub binary_method_call: BinaryMethodCall<'i>,
  pub call_array: Option<CallArray<'i>>,
}

impl<'i> MethodCall<'i> {
  pub fn parse(input: &'i [u8], parser: &mut BinaryParser<'i>) -> IResult<&'i [u8], Self> {
    let (input, ()) = parser.parse_binary_library(input)?;

    let (input, binary_method_call) = BinaryMethodCall::parse(input, parser)?;
    let (input, call_array) = opt(|input| CallArray::parse(input, parser))(input)?;

    Ok((input, Self { binary_method_call, call_array }))
  }
}
