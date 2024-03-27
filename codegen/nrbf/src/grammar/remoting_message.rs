use nom::{
  branch::alt,
  combinator::{map, opt},
  multi::many0,
  IResult,
};

use crate::{
  grammar::{MethodCall, MethodReturn, Referenceable},
  record::{MessageEnd, SerializationHeader},
};

#[derive(Debug, Clone, PartialEq)]
pub enum MethodCallOrReturn<'i> {
  MethodCall(MethodCall<'i>),
  MethodReturn(MethodReturn<'i>),
}

impl<'i> MethodCallOrReturn<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    alt((map(MethodCall::parse, Self::MethodCall), map(MethodReturn::parse, Self::MethodReturn)))(input)
  }
}

/// 2.7 Binary Record Grammar - `remotingMessage`
#[derive(Debug, Clone, PartialEq)]
pub struct RemotingMessage<'i> {
  pub header: SerializationHeader,
  pub pre_method_referenceables: Vec<Referenceable<'i>>,
  pub method_call_or_return: Option<MethodCallOrReturn<'i>>,
  pub post_method_referenceables: Vec<Referenceable<'i>>,
  pub end: MessageEnd,
}

impl<'i> RemotingMessage<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, header) = SerializationHeader::parse(input)?;
    let (input, pre_method_referenceables) = many0(Referenceable::parse)(input)?;
    let (input, method_call_or_return) = opt(MethodCallOrReturn::parse)(input)?;
    let (input, post_method_referenceables) = many0(Referenceable::parse)(input)?;
    let (input, end) = MessageEnd::parse(input)?;

    Ok((input, Self { header, pre_method_referenceables, method_call_or_return, post_method_referenceables, end }))
  }
}
