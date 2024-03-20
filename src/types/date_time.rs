use std::fmt;

use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};
#[cfg(feature = "impl_json_schema")]
use schemars::JsonSchema;
use serde::{
  de::{Deserialize, Deserializer},
  ser::{Serialize, Serializer},
};

use crate::Error;

#[inline]
fn byte_to_dec(byte: u8) -> u8 {
  byte / 16 * 10 + byte % 16
}

#[inline]
fn dec_to_byte(dec: u8) -> u8 {
  dec / 10 * 16 + dec % 10
}

#[derive(Clone, PartialEq)]
pub struct Date(pub(crate) NaiveDate);

impl Date {
  pub fn from_bytes(bytes: &[u8; 8]) -> Result<Self, Error> {
    let year = u16::from(byte_to_dec(bytes[0])) * 100 + u16::from(byte_to_dec(bytes[1]));
    let month = byte_to_dec(bytes[2]);
    let day = byte_to_dec(bytes[3]);

    if let Some(date) = NaiveDate::from_ymd_opt(year.into(), month.into(), day.into()) {
      Ok(Self(date))
    } else {
      Err(Error::InvalidFormat(format!("invalid date: {year:04}-{month:02}-{day:02}")))
    }
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

impl<'de> Deserialize<'de> for Date {
  fn deserialize<D>(deserializer: D) -> Result<Date, D::Error>
  where
    D: Deserializer<'de>,
  {
    NaiveDate::deserialize(deserializer).map(Self)
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
    write!(f, "{:04}-{:02}-{:02}", self.0.year(), self.0.month(), self.0.day())
  }
}

impl fmt::Debug for Date {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Date({})", self)
  }
}

#[derive(Clone, PartialEq)]
pub struct DateTime(pub(crate) NaiveDateTime);

impl DateTime {
  pub fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Result<Self, Error> {
    if let Some(datetime) = NaiveDate::from_ymd_opt(year.into(), month.into(), day.into())
      .and_then(|date| date.and_hms_opt(hour.into(), minute.into(), second.into()))
    {
      Ok(Self(datetime))
    } else {
      Err(Error::InvalidFormat(format!(
        "invalid datetime: {year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}"
      )))
    }
  }

  pub fn from_bytes(bytes: &[u8; 8]) -> Result<Self, Error> {
    let year = u16::from(byte_to_dec(bytes[0])) * 100 + u16::from(byte_to_dec(bytes[1]));
    let month = byte_to_dec(bytes[2]);
    let day = byte_to_dec(bytes[3]);

    let hour = byte_to_dec(bytes[5]);
    let minute = byte_to_dec(bytes[6]);
    let second = byte_to_dec(bytes[7]);

    Self::new(year, month, day, hour, minute, second)
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

impl<'de> Deserialize<'de> for DateTime {
  fn deserialize<D>(deserializer: D) -> Result<DateTime, D::Error>
  where
    D: Deserializer<'de>,
  {
    NaiveDateTime::deserialize(deserializer).map(Self)
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
    write!(
      f,
      "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
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
    write!(f, "DateTime({})", self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use chrono::{Datelike, Timelike};

  #[test]
  fn new() {
    let time = DateTime::new(2018, 12, 23, 17, 49, 31).unwrap();

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
    let time = DateTime::from_bytes(&[0x20, 0x18, 0x12, 0x23, 0x07, 0x17, 0x49, 0x31]).unwrap();

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
    let time = DateTime::new(2018, 12, 23, 17, 49, 31).unwrap();

    assert_eq!(time.to_bytes(), [0x20, 0x18, 0x12, 0x23, 0x07, 0x17, 0x49, 0x31]);
  }
}
