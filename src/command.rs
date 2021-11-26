use phf;
use serde::de::{self, Deserialize, Deserializer};

use crate::{AccessMode, Conversion, Parameter, Error, Optolink, protocol::Protocol, DataType, Value, types::{self, DateTime, CircuitTimes}};

/// A command which can be executed on an Optolink connection.
#[derive(Debug)]
pub struct Command {
  pub addr: u16,
  pub mode: AccessMode,
  pub data_type: DataType,
  pub parameter: Parameter,
  pub block_len: usize,
  pub byte_len: usize,
  pub byte_pos: usize,
  pub bit_pos: usize,
  pub bit_len: usize,
  pub conversion: Conversion,
  pub unit: Option<&'static str>,
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

    let bytes = &buf[self.byte_pos..(self.byte_pos + self.byte_len)];

    if bytes.iter().all(|&b| b == 0xff) {
      return Ok(Value::Empty)
    }

    let mut value = match &self.data_type {
      DataType::DateTime => Value::DateTime(DateTime::from_bytes(&bytes)),
      DataType::CircuitTimes => Value::CircuitTimes(CircuitTimes::from_bytes(&bytes)),
      DataType::Error => Value::Error(types::Error::from_bytes(&bytes)),
      DataType::String => {
        let end = bytes.iter().position(|&c| c == b'\0').unwrap_or(bytes.len());
        Value::String(String::from_utf8(bytes[..end].to_vec()).unwrap().to_owned())
      },
      DataType::ByteArray => Value::Array(bytes.to_vec()),
      t => {
        let mut n: i32 = 0;

        if self.bit_len > 0 {
          for i in 0..self.bit_len {
            let byte = (self.bit_pos + i) / 8;
            let bit = (self.bit_pos + i) % 8;
            let bit_mask = 1 << (8 - bit);

            if (buf[byte] & bit_mask) == 0 {
              n <<= 1;
            } else {
              n = (n << 1) | 0b1;
            }
          }
        } else {
          #[allow(arithmetic_overflow)]
          match self.parameter {
          	Parameter::IntHighByteFirst |
            Parameter::Int4HighByteFirst |
            Parameter::SIntHighByteFirst |
            Parameter::SInt4HighByteFirst => for &b in bytes.into_iter() {
              n = (n << 8) | (b as i32);
          	},
            _ => for &b in bytes.into_iter().rev() {
              n = (n << 8) | (b as i32);
            },
          }
        }

        match t {
          DataType::Byte => match self.parameter {
            Parameter::SByte => Value::Int(n as i8 as i64),
            Parameter::Byte => Value::Int(n as u8 as i64),
            _ => unreachable!(),
          },
          DataType::Int => match self.parameter {
            Parameter::Byte =>  Value::Int(n as u8 as i64),
            Parameter::Int | Parameter::IntHighByteFirst => Value::Int(n as u16 as i64),
            Parameter::Int4 | Parameter::Int4HighByteFirst => Value::Int(n as u32 as i64),
            Parameter::SByte =>  Value::Int(n as i8 as i64),
            Parameter::SInt | Parameter::SIntHighByteFirst => Value::Int(n as i16 as i64),
            Parameter::SInt4 | Parameter::SInt4HighByteFirst => Value::Int(n as i32 as i64),
            _ => unreachable!(),
          },
          DataType::Double => match self.parameter {
            Parameter::Byte =>  Value::Double(n as f64 as u8 as f64),
            Parameter::Int | Parameter::IntHighByteFirst => Value::Double(n as f64 as u16 as f64),
            Parameter::Int4 | Parameter::Int4HighByteFirst => Value::Double(n as f64 as u32 as f64),
            Parameter::SByte =>  Value::Double(n as f64 as i8 as f64),
            Parameter::SInt | Parameter::SIntHighByteFirst => Value::Double(n as f64 as i16 as f64),
            Parameter::SInt4 | Parameter::SInt4HighByteFirst => Value::Double(n as f64 as i32 as f64),
            _ => unreachable!(),
          },
          _ => unreachable!(),
        }
      }
    };

    self.conversion.convert(&mut value);

    Ok(value)
  }

  pub fn set(&self, o: &mut Optolink, protocol: Protocol, mut input: Value) -> Result<(), Error> {
    log::trace!("Command::set(…)");

    if !self.mode.is_write() {
      return Err(Error::UnsupportedMode(format!("Address 0x{:04X} does not support writing.", self.addr)))
    }

    self.conversion.convert_back(&mut input);

    let bytes = match &self.data_type {
      DataType::DateTime => {
        if let Value::DateTime(date_time) = input {
          date_time.to_bytes().to_vec()
        } else {
          return Err(Error::InvalidArgument(format!("expected DateTime, got {:?}", input)))
        }
      },
      DataType::CircuitTimes => {
        if let Value::CircuitTimes(cycletimes) = input {
          cycletimes.to_bytes().to_vec()
        } else {
          return Err(Error::InvalidArgument(format!("expected CircuitTimes, got {:?}", input)))
        }
      },
      DataType::ByteArray => {
        if let Value::Array(bytes) = input {
          bytes.to_vec()
        } else {
          return Err(Error::InvalidArgument(format!("expected ByteArray, got {:?}", input)))
        }
      },
      _ => {
        if let Value::Double(n) = input {
          match self.parameter {
            Parameter::Byte => (n as u8).to_le_bytes().to_vec(),
            Parameter::Int => (n as u16).to_le_bytes().to_vec(),
            Parameter::IntHighByteFirst => (n as u16).to_be_bytes().to_vec(),
            Parameter::Int4 => (n as u32).to_le_bytes().to_vec(),
            Parameter::Int4HighByteFirst => (n as u32).to_be_bytes().to_vec(),
            Parameter::SByte => (n as i8).to_le_bytes().to_vec(),
            Parameter::SInt => (n as i16).to_le_bytes().to_vec(),
            Parameter::SIntHighByteFirst => (n as i16).to_be_bytes().to_vec(),
            Parameter::SInt4 => (n as i32).to_le_bytes().to_vec(),
            Parameter::SInt4HighByteFirst => (n as i32).to_be_bytes().to_vec(),
            _ => unreachable!(),
          }
        } else {
          return Err(Error::InvalidArgument(format!("expected number, got {:?}", input)))
        }
      },
    };

    protocol.set(o, self.addr, &bytes).map_err(Into::into)
  }
}
