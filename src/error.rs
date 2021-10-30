use crate::types::DeviceIdent;
use std::io;
use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
  UnsupportedDevice(DeviceIdent),
  UnsupportedCommand(String),
  UnsupportedMode(String),
  InvalidArgument(String),
  UnknownEnumVariant(String),
  Io(io::Error)
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Error {
    Error::Io(err)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::UnsupportedDevice(device_ident) => write!(f, "Device ID 0x{:04X} HX {} SW {} not supported.", device_ident.id, device_ident.hardware_index, device_ident.software_index),
      Error::UnsupportedCommand(command) => write!(f, "command {} is not supported", command),
      Error::UnsupportedMode(description) => description.fmt(f),
      Error::InvalidArgument(description) => description.fmt(f),
      Error::UnknownEnumVariant(description) => description.fmt(f),
      Error::Io(err) => err.fmt(f),
    }
  }
}

impl StdError for Error {}
