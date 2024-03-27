use nom::{combinator::opt, IResult};

use crate::record::{BinaryLibrary, BinaryMethodCall, MethodCallArray};

#[derive(Debug, Clone, PartialEq)]
pub struct CallArray<'i> {
  pub binary_library: Option<BinaryLibrary<'i>>,
  pub call_array: MethodCallArray<'i>,
}

impl<'i> CallArray<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;
    let (input, call_array) = MethodCallArray::parse(input)?;

    Ok((input, Self { binary_library, call_array }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodCall<'i> {
  pub binary_library: Option<BinaryLibrary<'i>>,
  pub binary_method_call: BinaryMethodCall<'i>,
  pub call_array: Option<CallArray<'i>>,
}

impl<'i> MethodCall<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;
    let (input, binary_method_call) = BinaryMethodCall::parse(input)?;
    let (input, call_array) = opt(CallArray::parse)(input)?;

    Ok((input, Self { binary_library, binary_method_call, call_array }))
  }
}
