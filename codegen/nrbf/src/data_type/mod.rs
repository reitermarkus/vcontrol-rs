//! 2.1.1 Common Data Types

use nom::IResult;

use crate::error::{error_position, ErrorWithInput};

mod boolean;
pub use boolean::Boolean;
mod byte;
pub use byte::Byte;
mod int8;
pub use int8::Int8;
mod int16;
pub use int16::Int16;
mod int32;
pub use int32::Int32;
mod int64;
pub use int64::Int64;
mod uint16;
pub use uint16::UInt16;
mod uint32;
pub use uint32::UInt32;
mod uint64;
pub use uint64::UInt64;
mod char;
pub use char::Char;
mod double;
pub use double::Double;
mod single;
pub use single::Single;
mod timespan;
pub use timespan::TimeSpan;
mod datetime;
pub use datetime::DateTime;
mod decimal;
pub use decimal::Decimal;
mod length_prefixed_string;
pub use length_prefixed_string::LengthPrefixedString;

/// 2.1.1.8 `ClassTypeInfo`
#[derive(Debug, Clone, PartialEq)]
pub struct ClassTypeInfo<'i> {
  pub type_name: LengthPrefixedString<'i>,
  pub library_id: Int32,
}

impl<'i> ClassTypeInfo<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, ErrorWithInput<'i>> {
    let (input, type_name) = LengthPrefixedString::parse(input)
      .map_err(|err| err.map(|err| error_position!(err.input, ExpectedLengthPrefixedString)))?;
    let (input, library_id) = Int32::parse_positive(input)?;

    Ok((input, Self { type_name, library_id }))
  }
}

macro_rules! impl_primitive {
  ($ty:ident, $primitive:ty, $visit_fn:ident, $deserialize_fn:ident) => {
    impl From<$primitive> for $ty {
      #[inline]
      fn from(v: $primitive) -> Self {
        Self(v)
      }
    }

    impl From<$ty> for $primitive {
      #[inline]
      fn from(val: $ty) -> Self {
        val.0
      }
    }

    #[cfg(feature = "serde")]
    impl<'de> serde::Deserialize<'de> for $ty {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where
        D: serde::de::Deserializer<'de>,
      {
        use serde::de::{self, Error, MapAccess, Unexpected};

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
          type Value = $primitive;

          fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str(stringify!($ty))
          }

          fn $visit_fn<E>(self, value: $primitive) -> Result<Self::Value, E>
          where
            E: Error,
          {
            Ok(value)
          }

          fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
          where
            A: MapAccess<'de>,
          {
            if map.size_hint().unwrap_or(0) == 1 {
              if let Some(("m_value", n)) = map.next_entry::<&str, $primitive>()? {
                if map.next_entry::<&str, $primitive>()?.is_none() {
                  return Ok(n)
                }
              }
            }

            Err(Error::invalid_type(Unexpected::Map, &self))
          }
        }

        deserializer.$deserialize_fn(Visitor).map(Self)
      }
    }
  };
}
use impl_primitive;
