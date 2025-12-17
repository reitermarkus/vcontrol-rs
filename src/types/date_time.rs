use std::{fmt, str::FromStr};

use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, Timelike};
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{
  de::{Deserialize, Deserializer},
  ser::{Serialize, Serializer},
};

use crate::Error;

/// Maps a number from binary-coded-decimal (BCD) representation to decimal representation.
///
/// For example, `bcd_to_dec(0x15)` returns `15`.
#[inline]
fn bcd_to_dec(byte: u8) -> u8 {
  byte / 16 * 10 + byte % 16
}

/// Maps a number from decimal representation to binary-coded-decimal (BCD) representation.
///
/// For example, `dec_to_bcd(15)` returns `0x15`.
#[inline]
fn dec_to_bcd(dec: u8) -> u8 {
  dec / 10 * 16 + dec % 10
}

#[derive(Clone, Copy, PartialEq)]
pub struct Date(pub(crate) NaiveDate);

impl Date {
  /// Creates a new date from the given year, month and day.
  pub fn new(year: u16, month: u8, day: u8) -> Option<Self> {
    Some(Self(NaiveDate::from_ymd_opt(year.into(), month.into(), day.into())?))
  }

  pub fn from_bytes(bytes: &[u8; 8]) -> Result<Self, Error> {
    let year = u16::from(bcd_to_dec(bytes[0])) * 100 + u16::from(bcd_to_dec(bytes[1]));
    let month = bcd_to_dec(bytes[2]);
    let day = bcd_to_dec(bytes[3]);

    if let Some(date) = NaiveDate::from_ymd_opt(year.into(), month.into(), day.into()) {
      Ok(Self(date))
    } else {
      Err(Error::InvalidFormat(format!("invalid date: {year:04}-{month:02}-{day:02}")))
    }
  }

  pub fn to_bytes(&self) -> [u8; 8] {
    [
      dec_to_bcd((self.year() / 100) as u8),
      dec_to_bcd((self.year() % 100) as u8),
      dec_to_bcd(self.month()),
      dec_to_bcd(self.day()),
      self.weekday(),
      0,
      0,
      0,
    ]
  }

  /// Returns the year of this date.
  #[inline(always)]
  pub fn year(&self) -> u16 {
    self.0.year() as u16
  }

  /// Returns the month of this date.
  #[inline(always)]
  pub fn month(&self) -> u8 {
    self.0.month() as u8
  }

  /// Returns the day of this date.
  #[inline(always)]
  pub fn day(&self) -> u8 {
    self.0.day() as u8
  }

  /// Returns the weekday of this date, as a number from 0 (Monday) to 6 (Sunday).
  #[inline(always)]
  pub fn weekday(&self) -> u8 {
    self.0.weekday().num_days_from_monday() as u8
  }
}

impl FromStr for Date {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").map(Self).map_err(|_| ())
  }
}

impl<'de> Deserialize<'de> for Date {
  fn deserialize<D>(deserializer: D) -> Result<Date, D::Error>
  where
    D: Deserializer<'de>,
  {
    NaiveDate::deserialize(deserializer).map(Self)
  }
}

#[cfg(feature = "schemars")]
impl JsonSchema for Date {
  fn schema_name() -> String {
    "Date".into()
  }

  fn json_schema(generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
    let mut schema = generator.subschema_for::<String>().into_object();
    schema.format = Some("date".into());
    schema.into()
  }
}

impl Serialize for Date {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&self.to_string())
  }
}

impl fmt::Display for Date {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:04}-{:02}-{:02}", self.0.year(), self.0.month(), self.0.day())
  }
}

impl fmt::Debug for Date {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Date({})", self)
  }
}

#[derive(Clone, Copy, PartialEq)]
pub struct DateTime(pub(crate) NaiveDateTime);

impl DateTime {
  pub fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Result<Self, Error> {
    if let Some(datetime) = NaiveDate::from_ymd_opt(year.into(), month.into(), day.into())
      .and_then(|date| date.and_hms_opt(hour.into(), minute.into(), second.into()))
    {
      Ok(Self(datetime))
    } else {
      Err(Error::InvalidFormat(format!(
        "invalid date-time: {year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}"
      )))
    }
  }

  /// Create a DateTime from a unix timestamp.
  ///
  /// Note that this uses the system timezone.
  pub fn from_unix_timestamp(timestamp: u32) -> Self {
    Self(chrono::DateTime::from_timestamp_secs(timestamp.into()).unwrap().with_timezone(&Local).naive_local())
  }

  /// Returns the unix timestamp for this date-time.
  ///
  /// Note that this uses the system timezone.
  pub fn unix_timestamp(&self) -> u32 {
    self.0.and_local_timezone(Local).unwrap().timestamp() as u32
  }

  /// Returns the year of this date-time.
  pub fn year(&self) -> u16 {
    self.0.year() as u16
  }

  /// Returns the month of this date-time.
  pub fn month(&self) -> u8 {
    self.0.month() as u8
  }

  /// Returns the day of this date-time.
  pub fn day(&self) -> u8 {
    self.0.day() as u8
  }

  /// Returns the weekday of this date-time, as a number from 0 (Monday) to 6 (Sunday).
  pub fn weekday(&self) -> u8 {
    self.0.weekday().num_days_from_monday() as u8
  }

  /// Returns the hour of this date-time.
  #[inline(always)]
  pub fn hour(&self) -> u8 {
    self.0.hour() as u8
  }

  /// Returns the minute of this date-time.
  #[inline(always)]
  pub fn minute(&self) -> u8 {
    self.0.minute() as u8
  }

  /// Returns the second of this date-time.
  #[inline(always)]
  pub fn second(&self) -> u8 {
    self.0.second() as u8
  }

  pub fn from_bytes(bytes: &[u8; 8]) -> Result<Self, Error> {
    let year = u16::from(bcd_to_dec(bytes[0])) * 100 + u16::from(bcd_to_dec(bytes[1]));
    let month = bcd_to_dec(bytes[2]);
    let day = bcd_to_dec(bytes[3]);

    let hour = bcd_to_dec(bytes[5]);
    let minute = bcd_to_dec(bytes[6]);
    let second = bcd_to_dec(bytes[7]);

    Self::new(year, month, day, hour, minute, second)
  }

  pub fn to_bytes(&self) -> [u8; 8] {
    [
      dec_to_bcd((self.year() / 100) as u8),
      dec_to_bcd((self.year() % 100) as u8),
      dec_to_bcd(self.month()),
      dec_to_bcd(self.day()),
      self.weekday(),
      dec_to_bcd(self.hour()),
      dec_to_bcd(self.minute()),
      dec_to_bcd(self.second()),
    ]
  }
}

impl FromStr for DateTime {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S").map(Self).map_err(|_| ())
  }
}

impl<'de> Deserialize<'de> for DateTime {
  fn deserialize<D>(deserializer: D) -> Result<DateTime, D::Error>
  where
    D: Deserializer<'de>,
  {
    NaiveDateTime::deserialize(deserializer).map(Self)
  }
}

#[cfg(feature = "schemars")]
impl JsonSchema for DateTime {
  fn schema_name() -> String {
    "DateTime".into()
  }

  fn json_schema(generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
    let mut schema = generator.subschema_for::<String>().into_object();
    schema.format = Some("date-time".into());
    schema.into()
  }
}

impl Serialize for DateTime {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&self.to_string())
  }
}

impl fmt::Display for DateTime {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
      self.year(),
      self.month(),
      self.day(),
      self.hour(),
      self.minute(),
      self.second(),
    )
  }
}

impl fmt::Debug for DateTime {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "DateTime({})", self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bcd_to_dec() {
    for x in 0..=99 {
      let n = u8::from_str_radix(dbg!(&format!("{x:02}")), 16).unwrap();
      assert_eq!(bcd_to_dec(n), x);
    }
  }

  #[test]
  fn test_dec_to_bcd() {
    for n in 0..=99 {
      assert_eq!(format!("{:#04x}", dec_to_bcd(n)), format!("0x{n:02}"));
    }
  }

  #[test]
  fn new() {
    let time = DateTime::new(2018, 12, 23, 17, 49, 31).unwrap();

    assert_eq!(time.year(), 2018);
    assert_eq!(time.month(), 12);
    assert_eq!(time.day(), 23);
    assert_eq!(time.weekday(), 6);
    assert_eq!(time.hour(), 17);
    assert_eq!(time.minute(), 49);
    assert_eq!(time.second(), 31);
  }

  #[test]
  fn from_str() {
    let time = DateTime::from_str("2018-12-23T17:49:31").unwrap();

    assert_eq!(time.year(), 2018);
    assert_eq!(time.month(), 12);
    assert_eq!(time.day(), 23);
    assert_eq!(time.weekday(), 6);
    assert_eq!(time.hour(), 17);
    assert_eq!(time.minute(), 49);
    assert_eq!(time.second(), 31);
  }

  #[test]
  fn from_bytes() {
    let time = DateTime::from_bytes(&[0x20, 0x25, 0x12, 0x17, 0x02, 0x23, 0x31, 0x14]).unwrap();

    assert_eq!(time.year(), 2025);
    assert_eq!(time.month(), 12);
    assert_eq!(time.day(), 17);
    assert_eq!(time.weekday(), 2);
    assert_eq!(time.hour(), 23);
    assert_eq!(time.minute(), 31);
    assert_eq!(time.second(), 14);
  }

  #[test]
  fn to_bytes() {
    let time = DateTime::new(2018, 12, 23, 17, 49, 31).unwrap();

    assert_eq!(time.to_bytes(), [0x20, 0x18, 0x12, 0x23, 0x06, 0x17, 0x49, 0x31]);
  }
}
