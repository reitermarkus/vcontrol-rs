use std::fmt;

use crate::{enumeration::PrimitiveType, record::RecordType};

/// Error while parsing a [`RemotingMessage`](crate::RemotingMessage).
#[derive(Debug, Clone, PartialEq)]
pub struct Error<'i> {
  pub(crate) inner: ErrorWithInput<'i>,
}

impl fmt::Display for Error<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self.inner.inner {
      ErrorInner::ExpectedType(expected_type) => write!(f, "expected {}", expected_type),
      ErrorInner::Eof => write!(f, "unexpected end of input"),
      _ => todo!(),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
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
  UnresolvableMemberReference,
  InvalidCallArrayId,
  Eof,
  TrailingData,
  MissingRootObject,
  InvalidNullCount,
  InvalidObjectId,
  InvalidLength,
  InvalidMajorVersion,
  InvalidMinorVersion,
  InvalidRootId,
  MissingMetadataId,
  InvalidMetadataId,
  InvalidArgs,
  ExpectedBinaryType,
  ExpectedBinaryArrayType,
  MissingLibraryId,
  InvalidLibraryId,
  DuplicateBinaryLibrary,
  DuplicateClass,
  ExpectedRecordType(RecordType),
  ExpectedClassInfo,
  DuplicateObjectId,
  ExpectedBinaryMethodCall,
  ExpectedStringValueWithCode,
  ExpectedArrayOfValueWithCode,
  ExpectedMessageFlags,
  ExpectedPrimitiveType,
  ExpectedLengthPrefixedString,
  Other,
  InvalidMessageFlags,
  ExpectedPrimitive(PrimitiveType),
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

macro_rules! error_position {
  ($input:expr, $error_inner:ident) => {{
    $crate::error::ErrorWithInput { input: $input, inner: $crate::error::ErrorInner::$error_inner }
  }};
  ($input:expr, $error_inner:ident ( $expr:expr )) => {{
    $crate::error::ErrorWithInput { input: $input, inner: $crate::error::ErrorInner::$error_inner($expr) }
  }};
}
pub(crate) use error_position;
