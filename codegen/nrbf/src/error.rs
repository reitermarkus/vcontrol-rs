use std::fmt;

use crate::{enumeration::PrimitiveType, record::RecordType};

/// Error while parsing a [`RemotingMessage`](crate::RemotingMessage).
#[derive(Debug, Clone, PartialEq)]
pub struct Error<'i> {
  pub(crate) input: &'i [u8],
  pub(crate) inner: ErrorInner,
}

impl fmt::Display for Error<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.inner)
  }
}

impl<'i> nom::error::ParseError<&'i [u8]> for Error<'i> {
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
  Eof,
  TrailingData,
  UnresolvableMemberReference,
  InvalidCallArrayId,
  MissingRootObject,
  InvalidNullCount,
  InvalidObjectId,
  DuplicateObjectId,
  InvalidLength,
  InvalidMajorVersion,
  InvalidMinorVersion,
  InvalidRootId,
  MissingMetadataId,
  InvalidMetadataId,
  InvalidArgs,
  UnexpectedClass,
  ExpectedBinaryType,
  ExpectedBinaryArrayType,
  MissingLibraryId,
  InvalidLibraryId,
  DuplicateLibraryId,
  ExpectedRecordType(RecordType),
  ExpectedClassInfo,
  ExpectedMessageFlags,
  InvalidMessageFlags,
  ExpectedPrimitiveType,
  Other,
  ExpectedPrimitive(PrimitiveType),
}

impl fmt::Display for ErrorInner {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Eof => write!(f, "unexpected end of input"),
      Self::TrailingData => write!(f, "unexpected trailing data"),
      Self::UnresolvableMemberReference => write!(f, "unresolvable member reference"),
      Self::InvalidCallArrayId => write!(f, "invalid call array ID"),
      Self::MissingRootObject => write!(f, "missing root object"),
      Self::InvalidNullCount => write!(f, "invalid NULL count"),
      Self::InvalidObjectId => write!(f, "invalid object ID"),
      Self::DuplicateObjectId => write!(f, "duplicate object ID"),
      Self::InvalidLength => write!(f, "invalid length"),
      Self::InvalidMajorVersion => write!(f, "invalid major version"),
      Self::InvalidMinorVersion => write!(f, "invalid minor version"),
      Self::InvalidRootId => write!(f, "invalid root ID"),
      Self::MissingMetadataId => write!(f, "missing metadata ID"),
      Self::InvalidMetadataId => write!(f, "invalid metadata ID"),
      Self::InvalidArgs => write!(f, "invalid method arguments"),
      Self::UnexpectedClass => write!(f, "unexpected class"),
      Self::ExpectedBinaryType => write!(f, "expected BinaryType"),
      Self::ExpectedBinaryArrayType => write!(f, "expected BinaryArrayType"),
      Self::MissingLibraryId => write!(f, "missing library ID"),
      Self::InvalidLibraryId => write!(f, "invalid library ID"),
      Self::DuplicateLibraryId => write!(f, "duplicate library ID"),
      Self::ExpectedRecordType(record_type) => write!(f, "expected {}", record_type.description()),
      Self::ExpectedClassInfo => write!(f, "expected ClassInfo"),
      Self::ExpectedMessageFlags => write!(f, "expected MessageFlags"),
      Self::InvalidMessageFlags => write!(f, "invalid MessageFlags"),
      Self::ExpectedPrimitiveType => write!(f, "expected PrimitiveType"),
      Self::Other => write!(f, "other error"),
      Self::ExpectedPrimitive(primitive_type) => write!(f, "expected {}", primitive_type.description()),
    }
  }
}

macro_rules! error_position {
  ($input:expr, $error_inner:ident) => {{
    $crate::error::Error { input: $input, inner: $crate::error::ErrorInner::$error_inner }
  }};
  ($input:expr, $error_inner:ident ( $expr:expr )) => {{
    $crate::error::Error { input: $input, inner: $crate::error::ErrorInner::$error_inner($expr) }
  }};
}
pub(crate) use error_position;
