#[cfg(feature = "serde")]
use serde::{
  de::{self, Deserializer, Visitor},
  forward_to_deserialize_any,
};

#[cfg(feature = "serde")]
use crate::value::ValueDeserializer;
use crate::{BinaryParser, Error, Value};

/// A remote method call.
#[derive(Debug, Clone, PartialEq)]
pub struct MethodCall<'i> {
  /// The method name.
  pub method_name: &'i str,
  /// The server type name.
  pub type_name: &'i str,
  /// The logical call ID, if present.
  pub call_context: Option<&'i str>,
  /// The arguments, if present.
  pub args: Option<Vec<Value<'i>>>,
}

/// Information returned by a remote method.
#[derive(Debug, Clone, PartialEq)]
pub struct MethodReturn<'i> {
  /// The return value.
  pub return_value: Option<Value<'i>>,
  /// The logical call ID, if present.
  pub call_context: Option<&'i str>,
  /// The arguments, if present.
  pub args: Option<Vec<Value<'i>>>,
}

/// A .NET Remoting message.
///
/// # Example
///
/// ```
/// use std::collections::BTreeMap;
///
/// use nrbf::{RemotingMessage, Value};
///
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
/// assert_eq!(
///   RemotingMessage::parse(message),
///   Ok(RemotingMessage::Value(Value::String("This is a string."))),
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum RemotingMessage<'i> {
  /// A method call.
  MethodCall(MethodCall<'i>),
  /// A method return.
  MethodReturn(MethodReturn<'i>),
  /// A value.
  Value(Value<'i>),
}

impl<'i> RemotingMessage<'i> {
  /// Parse a [`RemotingMessage`] from bytes.
  pub fn parse(input: &'i [u8]) -> Result<Self, Error> {
    let parser = BinaryParser::default();
    parser.deserialize(input)
  }

  #[cfg(feature = "serde")]
  fn to_deserializer<V: Visitor<'i>>(&self, visitor: &V) -> Result<ValueDeserializer<'i, '_>, de::value::Error> {
    use serde::de::{Error, Unexpected};

    match self {
      Self::MethodCall(..) => Err(de::value::Error::invalid_type(Unexpected::Other("method call"), visitor)),
      Self::MethodReturn(..) => Err(de::value::Error::invalid_type(Unexpected::Other("method return"), visitor)),
      Self::Value(root_object) => Ok(ValueDeserializer::new(root_object)),
    }
  }
}

#[cfg(feature = "serde")]
impl<'de> Deserializer<'de> for RemotingMessage<'de> {
  type Error = de::value::Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    self.to_deserializer(&visitor)?.deserialize_any(visitor)
  }

  fn deserialize_struct<V>(
    self,
    name: &'static str,
    fields: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    self.to_deserializer(&visitor)?.deserialize_struct(name, fields, visitor)
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map enum identifier ignored_any
  }
}
