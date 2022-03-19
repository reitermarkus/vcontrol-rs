use core::fmt;

use arrayref::array_ref;
#[cfg(feature = "impl_json_schema")]
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::Device;
use super::DateTime;

#[cfg_attr(feature = "impl_json_schema", derive(JsonSchema))]
#[derive(Clone, Deserialize, Serialize)]
pub struct Error {
  index: u8,
  time: DateTime,
}

impl Error {
  pub fn new(index: u8, time: DateTime) -> Self {
    Self { index, time }
  }

  pub fn index(&self) -> u8 {
    self.index
  }

  pub fn to_str(&self, device: &Device) -> Option<&'static str> {
    device.errors().get(&(self.index as i32)).cloned()
  }

  pub fn time(&self) -> &DateTime {
    &self.time
  }

  pub fn from_bytes(bytes: &[u8; 9]) -> Self {
    let index = bytes[0];
    let time = DateTime::from_bytes(array_ref![bytes, 1, 8]);

    Self { index, time }
  }

  pub fn to_bytes(&self) -> [u8; 9] {
    let mut bytes = [0; 9];
    let time_bytes = self.time.to_bytes();
    bytes[0] = self.index;
    bytes[1..].copy_from_slice(&time_bytes);
    bytes
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: Error {:02X}", self.time, self.index)
  }
}

impl fmt::Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Error({}, ", self.index)?;
    self.time.fmt(f)?;
    write!(f, ")")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let time = DateTime::new(2018, 12, 23, 17, 49, 31);
    let error = Error::new(0xAC, time.clone());

    assert_eq!(error.index, 0xAC);
    assert_eq!(time.year(), 2018);
    assert_eq!(time.month(), 12);
    assert_eq!(time.day(), 23);
    assert_eq!(time.weekday(), 7);
    assert_eq!(time.hour(), 17);
    assert_eq!(time.minute(), 49);
    assert_eq!(time.second(), 31);
  }

  #[test]
  fn from_bytes() {
    let error = Error::from_bytes(&[0xAC, 0x20, 0x18, 0x12, 0x23, 0x07, 0x17, 0x49, 0x31]);

    assert_eq!(error.index, 0xAC);
    assert_eq!(error.time.year(), 2018);
    assert_eq!(error.time.month(), 12);
    assert_eq!(error.time.day(), 23);
    assert_eq!(error.time.weekday(), 7);
    assert_eq!(error.time.hour(), 17);
    assert_eq!(error.time.minute(), 49);
    assert_eq!(error.time.second(), 31);
  }

  #[test]
  fn to_bytes() {
    let time = DateTime::new(2018, 12, 23, 17, 49, 31);
    let error = Error::new(0xAC, time);

    assert_eq!(error.to_bytes(), [0xAC, 0x20, 0x18, 0x12, 0x23, 0x07, 0x17, 0x49, 0x31]);
  }
}
