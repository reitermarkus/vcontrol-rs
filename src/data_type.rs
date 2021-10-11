use phf;
use serde::Deserialize;

use crate::{Error, Value, FromBytes, ToBytes, RawType, types::{Bytes, SysTime, CycleTime}};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum DataType {
  I8,
  I16,
  I32,
  U8,
  U16,
  U32,
  F32,
  F64,
  String,
  Array,
  SysTime,
  CycleTime,
}

impl DataType {
  pub fn bytes_to_output(&self, raw_type: RawType, bytes: &[u8], factor: Option<f64>, mapping: &Option<phf::map::Map<Bytes, &'static str>>) -> Result<Value, Error> {
    if bytes.iter().all(|&b| b == 0xff) {
      return Ok(Value::Empty)
    }

    if let Some(mapping) = mapping {
      if let Some(text) = mapping.get(&Bytes::from_bytes(bytes)) {
        return Ok(Value::String((*text).to_string()))
      }

      return Err(Error::UnknownEnumVariant(format!("No enum mapping found for [{}].", bytes.iter().map(|byte| format!("0x{:02X}", byte)).collect::<Vec<String>>().join(", "))))
    }

    Ok(match self {
      Self::SysTime => return Ok(Value::SysTime(SysTime::from_bytes(bytes))),
      Self::CycleTime => return Ok(Value::CycleTime(CycleTime::from_bytes(bytes))),
      Self::String => return Ok(Value::String(String::from_utf8(bytes.to_vec()).unwrap())),
      Self::Array => return Ok(Value::Array(bytes.to_vec())),
      t => {
        let n = match raw_type {
          I8 => i64::from(i8::from_bytes(bytes).to_le()),
          I16 => i64::from(i16::from_bytes(bytes).to_le()),
          I32 => i64::from(i32::from_bytes(bytes).to_le()),
          U8 => i64::from(u8::from_bytes(bytes).to_le()),
          U16 => i64::from(u16::from_bytes(bytes).to_le()),
          U32 => i64::from(u32::from_bytes(bytes).to_le()),
          Array => unreachable!(),
        };

        match t {
          Self::I8 | Self::I16 | Self::I32 => Value::I32(n as i32),
          Self::U8 | Self::U16 | Self::U32 => Value::U32(n as u32),
          Self::F32 | Self::F64 => Value::F64(n as f64 / factor.unwrap_or(1.0)),
          _ => unreachable!(),
        }
      }
    })
  }

  pub fn input_to_bytes(&self, input: &Value, factor: Option<f64>, mapping: &Option<phf::map::Map<Bytes, &'static str>>) -> Result<Vec<u8>, Error> {
    if let Some(mapping) = mapping {
      if let Value::String(s) = input {
        return mapping.entries()
                 .find_map(|(key, value)| if value == s { Some(key.to_bytes()) } else { None })
                 .ok_or_else(|| Error::InvalidArgument(format!("no mapping found for {:?}", s)))
      } else {
        return Err(Error::InvalidArgument(format!("expected string, found {:?}", input)))
      }
    }

    Ok(match self {
      Self::SysTime => {
        if let Value::SysTime(systime) = input {
          systime.to_bytes()
        } else {
          return Err(Error::InvalidArgument(format!("expected systime, found {:?}", input)))
        }
      },
      Self::CycleTime => {
        if let Value::CycleTime(cycletime) = input {
          cycletime.to_bytes()
        } else {
          return Err(Error::InvalidArgument(format!("expected cycletime, found {:?}", input)))
        }
      },
      _ => {
        if let Value::F64(n) = input {
          let n = n * factor.unwrap_or(1.0);

          match self {
            Self::I8  => (n as i8).to_bytes(),
            Self::I16 => (n as i16).to_bytes(),
            Self::I32 => (n as i32).to_bytes(),
            Self::U8  => (n as u8).to_bytes(),
            Self::U16 => (n as u16).to_bytes(),
            Self::U32 => (n as u32).to_bytes(),
            _ => unreachable!(),
          }
        } else {
          return Err(Error::InvalidArgument(format!("expected number, found {:?}", input)))
        }
      },
    })
  }
}
