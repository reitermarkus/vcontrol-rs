use std::fmt;

use serde::{Serialize, Deserialize};

use super::SysTime;

#[derive(Clone, Deserialize, Serialize)]
pub struct Error {
  index: u8,
  time: SysTime,
}

impl Error {
  pub fn new(index: u8, time: SysTime) -> Self {
    Self { index, time }
  }

  pub fn from_bytes(bytes: &[u8]) -> Self {
    let index = bytes[0];
    let time = SysTime::from_bytes(&bytes[1..]);

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

  use crate::types::{FromBytes, ToBytes};

  #[test]
  fn new() {
    let time = SysTime::new(2018, 12, 23, 17, 49, 31);
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
    let time = SysTime::new(2018, 12, 23, 17, 49, 31);
    let error = Error::new(0xAC, time);

    assert_eq!(error.to_bytes(), [0xAC, 0x20, 0x18, 0x12, 0x23, 0x07, 0x17, 0x49, 0x31]);
  }
}
