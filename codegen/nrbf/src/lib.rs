#[cfg(feature = "serde")]
use serde::de::{value::Error, Deserialize};

pub mod common;
pub mod data_type;
pub mod enumeration;
pub mod grammar;
pub mod method_invocation;
pub mod record;

pub mod binary_parser;
pub use binary_parser::BinaryParser;

#[cfg(feature = "serde")]
pub fn from_stream<'i, T>(bytes: &'i [u8]) -> Result<T, Error>
where
  T: Deserialize<'i>,
{
  use nom::combinator::all_consuming;
  use serde::de::{Error, Unexpected};

  use grammar::RemotingMessage;

  let (_, remoting_message) = all_consuming(RemotingMessage::parse)(bytes)
    .map_err(|_| Error::invalid_type(Unexpected::Other("parsing error"), &"remoting message"))?;
  T::deserialize(remoting_message)
}
