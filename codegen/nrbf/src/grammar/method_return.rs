use nom::{combinator::opt, multi::many_m_n, IResult};

use crate::{
  data_type::Int32,
  grammar::MemberReference2,
  record::{BinaryLibrary, BinaryMethodReturn, MethodReturnCallArray},
};

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnCallArray<'i> {
  pub binary_library: Option<BinaryLibrary<'i>>,
  pub return_call_array: MethodReturnCallArray<'i>,
  pub member_references: Vec<MemberReference2<'i>>,
}

impl<'i> ReturnCallArray<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;
    let (input, return_call_array) = MethodReturnCallArray::parse(input)?;
    let length = return_call_array.0.array_info.len();
    let (input, member_references) = many_m_n(length, length, MemberReference2::parse)(input)?;

    Ok((input, Self { binary_library, return_call_array, member_references }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.return_call_array.object_id()
  }
}

/// 2.7 Binary Record Grammar - `methodReturn`
#[derive(Debug, Clone, PartialEq)]
pub struct MethodReturn<'i> {
  pub binary_library: Option<BinaryLibrary<'i>>,
  pub binary_method_return: BinaryMethodReturn<'i>,
  pub return_call_array: Option<ReturnCallArray<'i>>,
}

impl<'i> MethodReturn<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;
    let (input, binary_method_return) = BinaryMethodReturn::parse(input)?;
    let (input, return_call_array) = opt(ReturnCallArray::parse)(input)?;

    Ok((input, Self { binary_library, binary_method_return, return_call_array }))
  }
}
