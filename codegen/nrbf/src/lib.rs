//! A parser for the .NET Remoting Binary Format (NRBF).

#![warn(missing_docs)]

#[cfg(feature = "serde")]
use serde::de::{value::Error, Deserialize};

pub(crate) mod common;
pub(crate) mod data_type;
pub(crate) mod enumeration;
pub(crate) mod record;

mod binary_parser;
pub(crate) use binary_parser::BinaryParser;

mod remoting_message;
pub use remoting_message::{MethodCall, MethodReturn, RemotingMessage};

pub mod value;
#[doc(inline)]
pub use value::Value;

/// Deserialize an instance of type `T` from bytes of a .NET Remoting message.
///
/// # Example
///
/// ```
/// # use const_str::concat_bytes;
/// # #[rustfmt::skip]
/// let message = concat_bytes!(
///   0,
///     b"\x01\x00\x00\x00",
///     b"\xFF\xFF\xFF\xFF",
///     b"\x01\x00\x00\x00",
///     b"\x00\x00\x00\x00",
///   6,
///     b"\x01\x00\x00\x00",
///     17, "This is a string.",
///   11,
/// );
///
/// assert_eq!(nrbf::from_slice(message), Ok("This is a string."));
/// ```
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
