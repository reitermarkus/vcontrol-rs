//! 2.1.1 Common Data Types

use nom::IResult;

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
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, type_name) = LengthPrefixedString::parse(input)?;
    let (input, library_id) = Int32::parse_positive(input)?;

    Ok((input, Self { type_name, library_id }))
  }
}
