use nom::{combinator::opt, IResult};

use crate::{
  data_type::Int32,
  record::{BinaryMethodCall, MethodCallArray},
  BinaryParser,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CallArray<'i> {
  pub call_array: MethodCallArray<'i>,
}

impl<'i> CallArray<'i> {
  pub fn parse(input: &'i [u8], parser: &mut BinaryParser<'i>) -> IResult<&'i [u8], Self> {
    let (input, ()) = parser.parse_binary_library(input)?;

    let (input, call_array) = MethodCallArray::parse(input, parser)?;

    Ok((input, Self { call_array }))
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
