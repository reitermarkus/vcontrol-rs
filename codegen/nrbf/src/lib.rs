#[cfg(feature = "serde")]
use serde::de::{value::Error, DeserializeOwned};

pub mod common;
pub mod data_type;
pub mod enumeration;
pub mod grammar;
pub mod method_invocation;
pub mod record;

#[cfg(feature = "serde")]
pub fn from_stream<T>(bytes: &[u8]) -> Result<T, Error>
where
  T: DeserializeOwned,
{
  use nom::combinator::all_consuming;

  use grammar::RemotingMessage;

  let (_, remoting_message) = all_consuming(RemotingMessage::parse)(bytes).unwrap();
  T::deserialize(&remoting_message)
}
