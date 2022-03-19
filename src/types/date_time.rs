use std::fmt;

use chrono::{NaiveDate, NaiveTime, NaiveDateTime, Datelike, Timelike};
#[cfg(feature = "impl_json_schema")]
use schemars::JsonSchema;
use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer};

#[inline]
fn byte_to_dec(byte: u8) -> u8 {
  byte / 16 * 10 + byte % 16
}

#[inline]
fn dec_to_byte(dec: u8) -> u8 {
  dec / 10 * 16 + dec % 10
}

#[derive(Clone)]
pub struct Date(pub(crate) NaiveDate);

impl Date {
  pub fn from_bytes(bytes: &[u8; 8]) -> Self {
    let year = u16::from(byte_to_dec(bytes[0])) * 100 + u16::from(byte_to_dec(bytes[1]));
    let month = byte_to_dec(bytes[2]);
    let day = byte_to_dec(bytes[3]);
    let date = NaiveDate::from_ymd(year.into(), month.into(), day.into());

    Self(date)
  }

  pub fn to_bytes(&self) -> [u8; 8] {
    [
      dec_to_byte((self.0.year() / 100) as u8),
      dec_to_byte((self.0.year() % 100) as u8),
      dec_to_byte(self.0.month() as u8),
      dec_to_byte(self.0.day() as u8),
      self.0.weekday().number_from_monday() as u8,
      0,
      0,
      0,
    ]
  }
}

impl From<Date> for NaiveDate {
  fn from(date: Date) -> Self {
    date.0
  }
}

impl From<NaiveDate> for Date {
  fn from(date: NaiveDate) -> Self {
    Self(date)
  }
}

impl<'de> Deserialize<'de> for Date {
  fn deserialize<D>(deserializer: D) -> Result<Date, D::Error>
  where
      D: Deserializer<'de>,
  {
    NaiveDate::deserialize(deserializer).map(Into::into)
  }
}

#[cfg(feature = "impl_json_schema")]
impl JsonSchema for Date {
  fn schema_name() -> String {
    "Date".into()
  }

  fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    let mut schema = gen.subschema_for::<String>().into_object();
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
    write!(f, "{:04}-{:02}-{:02}",
      self.0.year(),
      self.0.month(),
      self.0.day(),
    )
  }
}

impl fmt::Debug for Date {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Date(")?;
    fmt::Display::fmt(self, f)?;
    write!(f, ")")
  }
}

#[derive(Clone)]
pub struct DateTime(pub(crate) NaiveDateTime);

impl DateTime {
  pub fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> DateTime {
    NaiveDate::from_ymd(year.into(), month.into(), day.into()).and_hms(hour.into(), minute.into(), second.into()).into()
  }

  pub fn from_bytes(bytes: &[u8; 8]) -> Self {
    let year = u16::from(byte_to_dec(bytes[0])) * 100 + u16::from(byte_to_dec(bytes[1]));
    let month = byte_to_dec(bytes[2]);
    let day = byte_to_dec(bytes[3]);
    let date = NaiveDate::from_ymd(year.into(), month.into(), day.into());

    let hour = byte_to_dec(bytes[5]);
    let minute = byte_to_dec(bytes[6]);
    let second = byte_to_dec(bytes[7]);
    let time = NaiveTime::from_hms(hour.into(), minute.into(), second.into());

    Self(NaiveDateTime::new(date, time))
  }

  pub fn to_bytes(&self) -> [u8; 8] {
    [
      dec_to_byte((self.0.year() / 100) as u8),
      dec_to_byte((self.0.year() % 100) as u8),
      dec_to_byte(self.0.month() as u8),
      dec_to_byte(self.0.day() as u8),
      self.0.weekday().number_from_monday() as u8,
      dec_to_byte(self.0.hour() as u8),
      dec_to_byte(self.0.minute() as u8),
      dec_to_byte(self.0.second() as u8),
    ]
  }
}

impl From<DateTime> for NaiveDateTime {
  fn from(date_time: DateTime) -> Self {
    date_time.0
  }
}

impl From<NaiveDateTime> for DateTime {
  fn from(date_time: NaiveDateTime) -> Self {
    Self(date_time)
  }
}

impl<'de> Deserialize<'de> for DateTime {
  fn deserialize<D>(deserializer: D) -> Result<DateTime, D::Error>
  where
      D: Deserializer<'de>,
  {
    NaiveDateTime::deserialize(deserializer).map(Into::into)
  }
}

#[cfg(feature = "impl_json_schema")]
impl JsonSchema for DateTime {
  fn schema_name() -> String {
    "DateTime".into()
  }

  fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    let mut schema = gen.subschema_for::<String>().into_object();
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
    write!(f, "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
      self.0.year(),
      self.0.month(),
      self.0.day(),
      self.0.hour(),
      self.0.minute(),
      self.0.second(),
    )
  }
}

impl fmt::Debug for DateTime {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "DateTime(")?;
    fmt::Display::fmt(self, f)?;
    write!(f, ")")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use chrono::Timelike;
  use chrono::Datelike;

  #[test]
  fn new() {
    let time = DateTime::new(2018, 12, 23, 17, 49, 31);

    assert_eq!(time.0.year(), 2018);
    assert_eq!(time.0.month(), 12);
    assert_eq!(time.0.day(), 23);
    assert_eq!(time.0.weekday().number_from_monday(), 7);
    assert_eq!(time.0.hour(), 17);
    assert_eq!(time.0.minute(), 49);
    assert_eq!(time.0.second(), 31);
  }

  // #[test]
  // fn from_str() {
  //   let time = DateTime::from_str("2018-12-23T17:49:31").unwrap();
  //
  //   assert_eq!(time.0.year(), 2018);
  //   assert_eq!(time.0.month(), 12);
  //   assert_eq!(time.0.day(), 23);
  //   assert_eq!(time.0.weekday() as u8, 7);
  //   assert_eq!(time.0.hour(), 17);
  //   assert_eq!(time.0.minute(), 49);
  //   assert_eq!(time.0.second(), 31);
  // }

  #[test]
  fn from_bytes() {
    let time = DateTime::from_bytes(&[0x20, 0x18, 0x12, 0x23, 0x07, 0x17, 0x49, 0x31]);

    assert_eq!(time.0.year(), 2018);
    assert_eq!(time.0.month(), 12);
    assert_eq!(time.0.day(), 23);
    assert_eq!(time.0.weekday().number_from_monday(), 7);
    assert_eq!(time.0.hour(), 17);
    assert_eq!(time.0.minute(), 49);
    assert_eq!(time.0.second(), 31);
  }

  #[test]
  fn to_bytes() {
    let time = DateTime::new(2018, 12, 23, 17, 49, 31);
    assert_eq!(time.to_bytes(), [0x20, 0x18, 0x12, 0x23, 0x07, 0x17, 0x49, 0x31]);
  }
}
