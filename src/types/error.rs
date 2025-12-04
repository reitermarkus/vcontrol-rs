use core::fmt;

use arrayref::array_ref;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::DateTime;
use crate::Device;

#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Error {
  index: u8,
  time: Option<DateTime>,
}

impl Error {
  pub fn new(index: u8, time: DateTime) -> Self {
    Self { index, time: Some(time) }
  }

  pub fn index(&self) -> u8 {
    self.index
  }

  pub fn to_str(&self, device: &Device) -> Option<&'static str> {
    device.errors().get(&(self.index as i32)).cloned()
  }

  pub fn time(&self) -> Option<&DateTime> {
    self.time.as_ref()
  }

  pub fn from_bytes(bytes: &[u8; 9]) -> Result<Self, crate::Error> {
    let index = bytes[0];

    let time = match DateTime::from_bytes(array_ref![bytes, 1, 8]) {
      Ok(time) => Some(time),
      Err(_) if index == 0 => None,
      Err(err) => return Err(crate::Error::InvalidFormat(format!("invalid time for error {index}: {err}"))),
    };

    Ok(Self { index, time })
  }

  pub fn to_bytes(&self) -> [u8; 9] {
    let mut bytes = [0; 9];

    bytes[0] = self.index;

    if let Some(time) = &self.time {
      let time_bytes = time.to_bytes();
      bytes[1..].copy_from_slice(&time_bytes);
    }

    bytes
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if let Some(time) = &self.time {
      write!(f, "{}: Error {:02X}", time, self.index)
    } else {
      write!(f, "0000-00-00: Error {:02X}", self.index)
    }
  }
}

impl fmt::Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_tuple("Error").field(&self.index).field(&self.time).finish()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use chrono::{Datelike, Timelike};

  #[test]
  fn new() {
    let time = DateTime::new(2018, 12, 23, 17, 49, 31).unwrap();
    let error = Error::new(0xAC, time.clone());

    assert_eq!(error.index, 0xAC);
    assert_eq!(error.time.as_ref().unwrap().0.year(), 2018);
    assert_eq!(error.time.as_ref().unwrap().0.month(), 12);
    assert_eq!(error.time.as_ref().unwrap().0.day(), 23);
    assert_eq!(error.time.as_ref().unwrap().0.weekday().number_from_monday(), 7);
    assert_eq!(error.time.as_ref().unwrap().0.hour(), 17);
    assert_eq!(error.time.as_ref().unwrap().0.minute(), 49);
    assert_eq!(error.time.as_ref().unwrap().0.second(), 31);
  }

  #[test]
  fn from_bytes() {
    let error = Error::from_bytes(&[0xAC, 0x20, 0x18, 0x12, 0x23, 0x07, 0x17, 0x49, 0x31]).unwrap();

    assert_eq!(error.index, 0xAC);
    assert_eq!(error.time.as_ref().unwrap().0.year(), 2018);
    assert_eq!(error.time.as_ref().unwrap().0.month(), 12);
    assert_eq!(error.time.as_ref().unwrap().0.day(), 23);
    assert_eq!(error.time.as_ref().unwrap().0.weekday().number_from_monday(), 7);
    assert_eq!(error.time.as_ref().unwrap().0.hour(), 17);
    assert_eq!(error.time.as_ref().unwrap().0.minute(), 49);
    assert_eq!(error.time.as_ref().unwrap().0.second(), 31);
  }

  #[test]
  fn to_bytes() {
    let time = DateTime::new(2018, 12, 23, 17, 49, 31).unwrap();
    let error = Error::new(0xAC, time);

    assert_eq!(error.to_bytes(), [0xAC, 0x20, 0x18, 0x12, 0x23, 0x07, 0x17, 0x49, 0x31]);
  }
}
