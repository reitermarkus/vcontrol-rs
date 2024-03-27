use nom::{branch::alt, combinator::map, multi::many0, IResult};

pub mod common;
use common::*;
pub mod data_type;

pub mod enumeration;
pub mod method_invocation;
pub mod record;
use record::{MessageEnd, SerializationHeader};
pub mod grammar;
use grammar::{MethodCall, MethodReturn, Referenceable};

#[derive(Debug, Clone, PartialEq)]
pub enum Record<'i> {
  SerializationHeader(SerializationHeader),
  MethodReturn(MethodReturn<'i>),
  MethodCall(MethodCall<'i>),
  MessageEnd(MessageEnd),
  Referenceable(Referenceable<'i>),
}

impl<'i> Record<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Vec<Self>> {
    let (input, _) = SerializationHeader::parse(input)?;

    let (input, records) = many0(alt((
      map(Referenceable::parse, Self::Referenceable),
      alt((map(MethodCall::parse, Self::MethodCall), map(MethodReturn::parse, Self::MethodReturn))),
    )))(input)?;

    let (input, _) = MessageEnd::parse(input)?;

    Ok((input, records))
  }
}

pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<Record<'_>>> {
  Record::parse(input)
}
