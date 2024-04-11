use std::fmt;

/// Error while parsing a [`RemotingMessage`](crate::RemotingMessage).
#[derive(Debug, Clone, PartialEq)]
pub struct Error {
  pub(crate) inner: ErrorInner,
}

pub(crate) struct ErrorWithInput<'i> {
  pub(crate) input: &'i [u8],
  pub(crate) inner: ErrorInner,
}

impl<'i> nom::error::ParseError<&'i [u8]> for ErrorWithInput<'i> {
  fn from_error_kind(input: &'i [u8], kind: nom::error::ErrorKind) -> Self {
    Self {
      input,
      inner: match kind {
        nom::error::ErrorKind::Eof => ErrorInner::Eof,
        _ => ErrorInner::Other,
      },
    }
  }

  fn append(_input: &'i [u8], _kind: nom::error::ErrorKind, other: Self) -> Self {
    other
  }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ErrorInner {
  ExpectedType(ExpectedType),
  ExpectedValue,
  ExpectedRemotingMessage,
  Eof,
  TrailingData,
  InvalidReferenceable,
  MissingRootObject,
  ExpectedMessageEnd,
  ExpectedHeader,
  MethodCallOrReturn,
  ExpectedBoolean,
  InvalidArgs,
  Other,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ExpectedType {
  Boolean,
  Byte,
  Char,
  DateTime,
  Decimal,
  Double,
  Int8,
  Int16,
  Int32,
  Int64,
  LengthPrefixedString,
  Single,
  TimeSpan,
  UInt16,
  UInt32,
  UInt64,
}

impl fmt::Display for ExpectedType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Boolean => "a BOOLEAN",
      Self::Byte => "a BYTE",
      Self::Char => "a CHAR",
      Self::DateTime => "a DateTime",
      Self::Decimal => "a Decimal",
      Self::Double => "a DOUBLE",
      Self::Int8 => "an INT8",
      Self::Int16 => "an INT16",
      Self::Int32 => "an INT32",
      Self::Int64 => "an INT64",
      Self::LengthPrefixedString => "a LengthPrefixedString",
      Self::Single => "a SINGLE",
      Self::TimeSpan => "a TimeSpan",
      Self::UInt16 => "a UINT16",
      Self::UInt32 => "a UINT32",
      Self::UInt64 => "a UINT64",
    }
    .fmt(f)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self.inner {
      ErrorInner::ExpectedType(expected_type) => write!(f, "expected {}", expected_type),
      ErrorInner::ExpectedValue => write!(f, "expected a value"),
      ErrorInner::ExpectedRemotingMessage => write!(f, "expected a remoting message"),
      ErrorInner::Eof => write!(f, "unexpected end of input"),
      _ => todo!(),
    }
  }
}
