#[cfg(feature = "serde")]
use serde::de::{value::Error, Deserialize};

pub mod common;
pub mod data_type;
pub mod enumeration;
pub mod record;

mod binary_parser;
pub use binary_parser::BinaryParser;

mod remoting_message;
pub use remoting_message::{MethodCallOrReturn, RemotingMessage};

pub mod value;
pub use value::Value;

#[cfg(feature = "serde")]
pub fn from_slice<'i, T>(bytes: &'i [u8]) -> Result<T, Error>
where
  T: Deserialize<'i>,
{
  use nom::combinator::all_consuming;
  use serde::de::{Error, Unexpected};

  let (_, remoting_message) = all_consuming(RemotingMessage::parse)(bytes)
    .map_err(|_| Error::invalid_type(Unexpected::Other("parsing error"), &"remoting message"))?;
  T::deserialize(remoting_message)
}
