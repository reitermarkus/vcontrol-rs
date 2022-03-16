use arrayref::array_ref;

use crate::{AccessMode, conversion::Conversion, Parameter, Error, Optolink, protocol::Protocol, DataType, Value, types::{self, DeviceId, DeviceIdF0, DateTime, CircuitTimes}};

/// A command which can be executed on an Optolink connection.
#[derive(Debug)]
pub struct Command {
  pub(crate) addr: u16,
  pub(crate) mode: AccessMode,
  pub(crate) data_type: DataType,
  pub(crate) parameter: Parameter,
  pub(crate) block_len: usize,
  pub(crate) byte_len: usize,
  pub(crate) byte_pos: usize,
  pub(crate) bit_pos: usize,
  pub(crate) bit_len: usize,
  pub(crate) conversion: Option<Conversion>,
  pub(crate) lower_bound: Option<f32>,
  pub(crate) upper_bound: Option<f32>,
  pub(crate) unit: Option<&'static str>,
  pub(crate) mapping: Option<phf::map::Map<i32, &'static str>>,
}

impl Command {
  /// Get the command's access mode.
  pub fn access_mode(&self) -> AccessMode {
    self.mode
  }

  pub fn get(&self, o: &mut Optolink, protocol: Protocol) -> Result<Value, Error> {
    log::trace!("Command::get(…)");

    if !self.mode.is_read() {
      return Err(Error::UnsupportedMode(format!("Address 0x{:04X} does not support reading.", self.addr)))
    }

    let mut buf = vec![0; self.block_len];
    protocol.get(o, self.addr, &mut buf)?;

    let bytes = &buf[self.byte_pos..(self.byte_pos + self.byte_len)];

    if bytes.iter().all(|&b| b == 0xff) && !matches!(
      self.data_type, DataType::DeviceId | DataType::DeviceIdF0 | DataType::ErrorIndex
    ) {
      return Ok(Value::Empty)
    }

    let mut value = match &self.data_type {
      DataType::DeviceId => {
        if buf.len() != 8 {
          return Err(Error::InvalidFormat("array length is not 8".to_string()))
        }

        Value::DeviceId(DeviceId::from_bytes(array_ref![buf, 0, 8]))
      },
      DataType::DeviceIdF0 => {
        if buf.len() != 2 {
          return Err(Error::InvalidFormat("array length is not 2".to_string()))
        }

        Value::DeviceIdF0(DeviceIdF0::from_bytes(array_ref![buf, 0, 2]))
      },
      DataType::DateTime => {
        if bytes.len() != 8 {
          return Err(Error::InvalidFormat("array length is not 8".to_string()))
        }

        Value::DateTime(DateTime::from_bytes(array_ref![bytes, 0, 8]))
      },
      DataType::CircuitTimes => {
        if bytes.len() != 56 {
          return Err(Error::InvalidFormat("array length is not 56".to_string()))
        }

        Value::CircuitTimes(CircuitTimes::from_bytes(array_ref![bytes, 0, 56]))
      },
      DataType::ErrorIndex => Value::Int(bytes[0] as i64),
      DataType::Error => {
        if bytes.len() != 9 {
          return Err(Error::InvalidFormat("array length is not 9".to_string()))
        }

        Value::Error(types::Error::from_bytes(array_ref![bytes, 0, 9]))
      },
      DataType::String => {
        let end = bytes.iter().position(|&c| c == b'\0').unwrap_or(bytes.len());

        match String::from_utf8(bytes[..end].to_vec()) {
          Ok(s) => Value::String(s),
          Err(err) => return Err(Error::Utf8(err)),
        }
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
            Parameter::SInt4HighByteFirst => for &b in bytes.iter().take(4) {
              n = (n << 8) | (b as i32);
          	},
            _ => for &b in bytes.iter().rev().take(4) {
              n = (n << 8) | (b as i32);
            },
          }
        }

        match t {
          data_type @ DataType::Byte => match &self.parameter {
            Parameter::SByte => Value::Int(n as i8 as i64),
            Parameter::Byte => Value::Int(n as u8 as i64),
            parameter => unreachable!("Data type {:?} with parameter {:?}.", data_type, parameter),
          },
          data_type @ DataType::Int => match &self.parameter {
            Parameter::Byte =>  Value::Int(n as u8 as i64),
            Parameter::Int | Parameter::IntHighByteFirst => Value::Int(n as u16 as i64),
            Parameter::Int4 | Parameter::Int4HighByteFirst => Value::Int(n as u32 as i64),
            Parameter::SByte =>  Value::Int(n as i8 as i64),
            Parameter::SInt | Parameter::SIntHighByteFirst => Value::Int(n as i16 as i64),
            Parameter::SInt4 | Parameter::SInt4HighByteFirst => Value::Int(n as i32 as i64),
            parameter => unreachable!("Data type {:?} with parameter {:?} ({:?}).", data_type, parameter, bytes),
          },
          data_type @ DataType::Double => match &self.parameter {
            Parameter::Byte =>  Value::Double(n as f64 as u8 as f64),
            Parameter::Int | Parameter::IntHighByteFirst => Value::Double(n as f64 as u16 as f64),
            Parameter::Int4 | Parameter::Int4HighByteFirst => Value::Double(n as f64 as u32 as f64),
            Parameter::SByte =>  Value::Double(n as f64 as i8 as f64),
            Parameter::SInt | Parameter::SIntHighByteFirst => Value::Double(n as f64 as i16 as f64),
            Parameter::SInt4 | Parameter::SInt4HighByteFirst => Value::Double(n as f64 as i32 as f64),
            parameter => unreachable!("Data type {:?} with parameter {:?}.", data_type, parameter),
          },
          _ => unreachable!(),
        }
      }
    };

    if let Some(conversion) = &self.conversion {
      value.convert(conversion);
    }

    Ok(value)
  }

  pub fn set(&self, o: &mut Optolink, protocol: Protocol, mut input: Value) -> Result<(), Error> {
    log::trace!("Command::set(…)");

    if !self.mode.is_write() {
      return Err(Error::UnsupportedMode(format!("Address 0x{:04X} does not support writing.", self.addr)))
    }

    if let Some(conversion) = &self.conversion {
      input.convert_back(conversion);
    }

    let bytes = match (&self.data_type, input) {
      (DataType::DateTime, Value::DateTime(date_time)) => {
        date_time.to_bytes().to_vec()
      },
      (DataType::CircuitTimes, Value::CircuitTimes(cycletimes)) => {
        cycletimes.to_bytes().to_vec()
      },
      (DataType::ByteArray, Value::Array(bytes)) => {
        bytes.to_vec()
      },
      (DataType::String, Value::String(s)) => {
        s.as_bytes().to_vec()
      },
      (DataType::Error, Value::Error(error)) => {
        error.to_bytes().to_vec()
      }
      (DataType::Int | DataType::Byte, Value::Int(n)) => {
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
      },
      (DataType::Double, Value::Double(n)) => {
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
      },
      (data_type, input) => {
        return Err(Error::InvalidArgument(format!("expected {:?}, got {:?}", data_type, input)))
      }
    };

    protocol.set(o, self.addr, &bytes).map_err(Into::into)
  }
}
