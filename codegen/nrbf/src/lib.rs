#[cfg(feature = "serde")]
use serde::de::{value::Error, Deserialize};

pub(crate) mod common;
pub mod data_type;
pub(crate) mod enumeration;
pub(crate) mod record;

mod binary_parser;
pub use binary_parser::BinaryParser;

mod remoting_message;
pub use remoting_message::{MethodCall, MethodReturn, RemotingMessage};

pub mod value;
pub use value::Value;

/// Deserialize an instance of type `T` from bytes of a .NET Remoting message.
#[cfg(feature = "serde")]
pub fn from_slice<'i, T>(bytes: &'i [u8]) -> Result<T, Error>
where
  T: Deserialize<'i>,
{
  use nom::combinator::all_consuming;
  use serde::de::Error;

  let (_, remoting_message) =
    all_consuming(RemotingMessage::parse)(bytes).map_err(|err| Error::custom(format!("parsing error: {}", err)))?;
  T::deserialize(remoting_message)
}
