use crate::types::{DeviceIdent, DeviceIdentF0};
use std::io;
use std::fmt;
use std::error::Error as StdError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
  UnsupportedDevice(DeviceIdent, Option<DeviceIdentF0>),
  UnsupportedCommand(String),
  UnsupportedMode(String),
  InvalidArgument(String),
  UnknownEnumVariant(String),
  Utf8(FromUtf8Error),
  Io(io::Error),
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Error {
    Error::Io(err)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::UnsupportedDevice(device_ident, device_ident_f0) => {
        write!(
          f, "Device ID 0x{:04X} HX 0x{:02X} SW 0x{:02X}",
          device_ident.id, device_ident.hardware_index, device_ident.software_index
        )?;

        if let Some(device_ident_f0) = device_ident_f0 {
          write!(f, " F0 0x{:04X}", device_ident_f0.0)?;
        }

        write!(f, " not supported.")
      },
      Error::UnsupportedCommand(command) => write!(f, "command {} is not supported", command),
      Error::UnsupportedMode(description) => description.fmt(f),
      Error::InvalidArgument(description) => description.fmt(f),
      Error::UnknownEnumVariant(description) => description.fmt(f),
      Error::Utf8(err) => err.fmt(f),
      Error::Io(err) => err.fmt(f),
    }
  }
}

impl StdError for Error {}
