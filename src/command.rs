use phf;
use serde::de::{self, Deserialize, Deserializer};

use crate::{Error, Optolink, protocol::Protocol, DataType, RawType, Value};

#[derive(Debug, Clone, Copy)]
pub enum AccessMode {
  Read,
  Write,
  ReadWrite,
}

impl AccessMode {
  pub fn is_read(self) -> bool {
    match self {
      AccessMode::Read | AccessMode::ReadWrite => true,
      _ => false,
    }
  }

  pub fn is_write(self) -> bool {
    match self {
      AccessMode::Write | AccessMode::ReadWrite => true,
      _ => false,
    }
  }
}

impl<'de> Deserialize<'de> for AccessMode {
  fn deserialize<D>(deserializer: D) -> Result<AccessMode, D::Error>
  where
      D: Deserializer<'de>,
  {
    match String::deserialize(deserializer)?.as_str() {
      "read" => Ok(AccessMode::Read),
      "write" => Ok(AccessMode::Write),
      "read_write" => Ok(AccessMode::ReadWrite),
      variant => Err(de::Error::unknown_variant(&variant, &["read", "write", "read_write"])),
    }
  }
}

/// A command which can be executed on an Optolink connection.
#[derive(Debug)]
pub struct Command {
  pub addr: u16,
  pub mode: AccessMode,
  pub data_type: DataType,
  pub raw_type: RawType,
  pub block_len: usize,
  pub byte_len: usize,
  pub byte_pos: usize,
  pub bit_pos: usize,
  pub bit_len: usize,
  pub factor: Option<f64>,
  pub mapping: Option<phf::map::Map<i32, &'static str>>,
}

impl Command {
  pub fn get(&self, o: &mut Optolink, protocol: Protocol) -> Result<Value, Error> {
    log::trace!("Command::get(…)");

    if !self.mode.is_read() {
      return Err(Error::UnsupportedMode(format!("Address 0x{:04X} does not support reading.", self.addr)))
    }

    let mut buf = vec![0; self.block_len];
    protocol.get(o, self.addr, &mut buf)?;

    self.data_type.bytes_to_output(self.raw_type, &buf[self.byte_pos..(self.byte_pos + self.byte_len)], self.factor, &self.mapping, self.bit_pos, self.bit_len)
  }

  pub fn set(&self, o: &mut Optolink, protocol: Protocol, input: &Value) -> Result<(), Error> {
    log::trace!("Command::set(…)");

    if !self.mode.is_write() {
      return Err(Error::UnsupportedMode(format!("Address 0x{:04X} does not support writing.", self.addr)))
    }

    protocol.set(o, self.addr, &self.data_type.input_to_bytes(input, self.raw_type, self.factor, &self.mapping)?).map_err(Into::into)
  }
}
