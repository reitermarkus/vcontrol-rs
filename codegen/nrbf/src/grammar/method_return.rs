use nom::{combinator::opt, multi::many_m_n, IResult};

use crate::{
  binary_parser::Object,
  data_type::Int32,
  grammar::MemberReferenceInner,
  record::{BinaryMethodReturn, MethodReturnCallArray},
  BinaryParser,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnCallArray<'i> {
  pub return_call_array: MethodReturnCallArray,
  pub member_references: Vec<Object<'i>>,
}

impl<'i> ReturnCallArray<'i> {
  pub fn parse(input: &'i [u8], parser: &mut BinaryParser<'i>) -> IResult<&'i [u8], Self> {
    let (input, ()) = parser.parse_binary_library(input)?;

    let (input, return_call_array) = MethodReturnCallArray::parse(input)?;
    let length = return_call_array.0.array_info.len();
    let (input, member_references) =
      many_m_n(length, length, |input| parser.parse_member_reference(input, None))(input)?;

    Ok((input, Self { return_call_array, member_references }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.return_call_array.object_id()
  }
}

/// 2.7 Binary Record Grammar - `methodReturn`
#[derive(Debug, Clone, PartialEq)]
pub struct MethodReturn<'i> {
  pub binary_method_return: BinaryMethodReturn<'i>,
  pub return_call_array: Option<ReturnCallArray<'i>>,
}

impl<'i> MethodReturn<'i> {
  pub fn parse(input: &'i [u8], parser: &mut BinaryParser<'i>) -> IResult<&'i [u8], Self> {
    let (input, ()) = parser.parse_binary_library(input)?;

    let (input, binary_method_return) = BinaryMethodReturn::parse(input)?;
    let (input, return_call_array) = opt(|input| ReturnCallArray::parse(input, parser))(input)?;

    Ok((input, Self { binary_method_return, return_call_array }))
  }
}
