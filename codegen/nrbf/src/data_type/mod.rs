//! 2.1.1 Common Data Types
use std::str::FromStr;

use nom::{
  branch::alt,
  bytes::complete::take,
  combinator::{map, map_opt, map_res},
  number::complete::{i8, le_f32, le_f64, le_i16, le_i32, le_i64, le_u16, le_u24, le_u32, le_u64, u8},
  sequence::{pair, preceded},
  IResult,
};

/// 2.1.1 `BOOLEAN`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Boolean(pub bool);

impl Boolean {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map_res(u8, |byte| {
      Ok(Self(match byte {
        0 => false,
        1 => true,
        _ => return Err(()),
      }))
    })(input)
  }
}

impl From<bool> for Boolean {
  fn from(v: bool) -> Self {
    Self(v)
  }
}

impl Into<bool> for Boolean {
  fn into(self) -> bool {
    self.0
  }
}

/// 2.1.1 `BYTE`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Byte(pub u8);

impl Byte {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(u8, Self)(input)
  }
}

impl From<u8> for Byte {
  fn from(v: u8) -> Self {
    Self(v)
  }
}

impl Into<u8> for Byte {
  fn into(self) -> u8 {
    self.0
  }
}

/// 2.1.1 `INT8`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Int8(pub i8);

impl Int8 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(i8, Self)(input)
  }
}

impl From<i8> for Int8 {
  fn from(v: i8) -> Self {
    Self(v)
  }
}

impl Into<i8> for Int8 {
  fn into(self) -> i8 {
    self.0
  }
}

/// 2.1.1 `INT16`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Int16(pub i16);

impl Int16 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_i16, Self)(input)
  }
}

impl From<i16> for Int16 {
  fn from(v: i16) -> Self {
    Self(v)
  }
}

impl Into<i16> for Int16 {
  fn into(self) -> i16 {
    self.0
  }
}

/// 2.1.1 `INT32`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Int32(pub i32);

impl Int32 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_i32, Self)(input)
  }
}

impl From<i32> for Int32 {
  fn from(v: i32) -> Self {
    Self(v)
  }
}

impl Into<i32> for Int32 {
  fn into(self) -> i32 {
    self.0
  }
}

/// 2.1.1 `INT64`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Int64(pub i64);

impl Int64 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_i64, Self)(input)
  }
}

impl From<i64> for Int64 {
  fn from(v: i64) -> Self {
    Self(v)
  }
}

impl Into<i64> for Int64 {
  fn into(self) -> i64 {
    self.0
  }
}

/// 2.1.1 `UINT16`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UInt16(pub u16);

impl UInt16 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_u16, Self)(input)
  }
}

impl From<u16> for UInt16 {
  fn from(v: u16) -> Self {
    Self(v)
  }
}

impl Into<u16> for UInt16 {
  fn into(self) -> u16 {
    self.0
  }
}

/// 2.1.1 `UINT32`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UInt32(pub u32);

impl UInt32 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_u32, Self)(input)
  }
}

impl From<u32> for UInt32 {
  fn from(v: u32) -> Self {
    Self(v)
  }
}

impl Into<u32> for UInt32 {
  fn into(self) -> u32 {
    self.0
  }
}

/// 2.1.1 `UINT64`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UInt64(pub u64);

impl UInt64 {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_u64, Self)(input)
  }
}

impl From<u64> for UInt64 {
  fn from(v: u64) -> Self {
    Self(v)
  }
}

impl Into<u64> for UInt64 {
  fn into(self) -> u64 {
    self.0
  }
}

/// 2.1.1.1 `Char`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Char(pub char);

impl Char {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(
      alt((
        map_opt(u8, |n| char::from_u32(n as u32)),
        map_opt(le_u16, |n| char::from_u32(n as u32)),
        map_opt(le_u24, |n| char::from_u32(n as u32)),
        map_opt(le_u32, char::from_u32),
      )),
      Self,
    )(input)
  }
}

impl From<char> for Char {
  fn from(v: char) -> Self {
    Self(v)
  }
}

impl Into<char> for Char {
  fn into(self) -> char {
    self.0
  }
}

/// 2.1.1.2 `Double`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Double(pub f64);

impl Double {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_f64, Self)(input)
  }
}

impl From<f64> for Double {
  fn from(v: f64) -> Self {
    Self(v)
  }
}

impl Into<f64> for Double {
  fn into(self) -> f64 {
    self.0
  }
}

/// 2.1.1.3 `Single`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Single(pub f32);

impl Single {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_f32, Self)(input)
  }
}

impl From<f32> for Single {
  fn from(v: f32) -> Self {
    Self(v)
  }
}

impl Into<f32> for Single {
  fn into(self) -> f32 {
    self.0
  }
}

/// 2.1.1.4 `TimeSpan`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeSpan(pub i64);

impl TimeSpan {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_i64, Self)(input)
  }
}

impl From<i64> for TimeSpan {
  fn from(v: i64) -> Self {
    Self(v)
  }
}

impl Into<i64> for TimeSpan {
  fn into(self) -> i64 {
    self.0
  }
}

/// 2.1.1.5 `DateTime`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DateTime(pub i64);

impl DateTime {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_i64, Self)(input)
  }
}

impl From<i64> for DateTime {
  fn from(v: i64) -> Self {
    Self(v)
  }
}

impl Into<i64> for DateTime {
  fn into(self) -> i64 {
    self.0
  }
}

mod length_prefixed_string;
pub use length_prefixed_string::LengthPrefixedString;

/// 2.1.1.7 `Decimal`
#[derive(Debug, Clone, PartialEq)]
pub struct Decimal(pub rust_decimal::Decimal);

impl Decimal {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map_res(LengthPrefixedString::parse, |s| rust_decimal::Decimal::from_str(s.as_str()).map(Self))(input)
  }
}

/// 2.1.1.8 `ClassTypeInfo`
#[derive(Debug, Clone, PartialEq)]
pub struct ClassTypeInfo<'i> {
  pub type_name: LengthPrefixedString<'i>,
  pub library_id: i32,
}

impl<'i> ClassTypeInfo<'i> {
  pub fn parse(mut input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, type_name) = LengthPrefixedString::parse(input)?;
    let (input, library_id) = le_i32(input)?;

    Ok((input, Self { type_name, library_id }))
  }
}
