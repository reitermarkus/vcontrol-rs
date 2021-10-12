use phf;
use serde::Deserialize;

use crate::{Error, Value, RawType, types::{self, SysTime, CycleTime}};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum DataType {
  Int,
  Double,
  String,
  Array,
  SysTime,
  CycleTime,
  Error,
}

impl DataType {
  pub fn bytes_to_output(&self, raw_type: RawType, bytes: &[u8], factor: Option<f64>, mapping: &Option<phf::map::Map<u8, &'static str>>) -> Result<Value, Error> {
    if bytes.iter().all(|&b| b == 0xff) {
      return Ok(Value::Empty)
    }

    // if let Some(mapping) = mapping {
    //   if let Some(text) = mapping.get(&bytes[0]) {
    //     return Ok(Value::String((*text).to_string()))
    //   }
    //
    //   return Err(Error::UnknownEnumVariant(format!("No enum mapping found for [{}].", bytes.iter().map(|byte| format!("0x{:02X}", byte)).collect::<Vec<String>>().join(", "))))
    // }

    Ok(match self {
      Self::SysTime => return Ok(Value::SysTime(SysTime::from_bytes(bytes))),
      Self::CycleTime => return Ok(Value::CycleTime(CycleTime::from_bytes(bytes))),
      Self::Error => return Ok(Value::Error(types::Error::from_bytes(bytes))),
      Self::String => return Ok(Value::String(String::from_utf8(bytes.to_vec()).unwrap())),
      Self::Array => return Ok(Value::Array(bytes.to_vec())),
      t => {
        let n = match raw_type {
          RawType::I8 => i64::from(i8::from_le_bytes([bytes[0]])),
          RawType::I16 => i64::from(i16::from_le_bytes([bytes[0], bytes[1]])),
          RawType::I32 => i64::from(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])),
          RawType::U8 => i64::from(u8::from_le_bytes([bytes[0]])),
          RawType::U16 => i64::from(u16::from_le_bytes([bytes[0], bytes[1]])),
          RawType::U32 => i64::from(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])),
          RawType::Array => unreachable!(),
        };

        match t {
          Self::Int => Value::Int(n),
          Self::Double => Value::Double(n as f64 / factor.unwrap_or(1.0)),
          _ => unreachable!(),
        }
      }
    })
  }

  pub fn input_to_bytes(&self, input: &Value, raw_type: RawType, factor: Option<f64>, mapping: &Option<phf::map::Map<u8, &'static str>>) -> Result<Vec<u8>, Error> {
    // if let Some(mapping) = mapping {
    //   if let Value::String(s) = input {
    //     return mapping.entries()
    //              .find_map(|(key, value)| if value == s { Some(key.to_le_bytes().to_vec()) } else { None })
    //              .ok_or_else(|| Error::InvalidArgument(format!("no mapping found for {:?}", s)))
    //   } else {
    //     return Err(Error::InvalidArgument(format!("expected string, found {:?}", input)))
    //   }
    // }

    Ok(match self {
      Self::SysTime => {
        if let Value::SysTime(systime) = input {
          systime.to_bytes().to_vec()
        } else {
          return Err(Error::InvalidArgument(format!("expected systime, found {:?}", input)))
        }
      },
      Self::CycleTime => {
        if let Value::CycleTime(cycletime) = input {
          cycletime.to_bytes().to_vec()
        } else {
          return Err(Error::InvalidArgument(format!("expected cycletime, found {:?}", input)))
        }
      },
      _ => {
        if let Value::Double(n) = input {
          let n = n * factor.unwrap_or(1.0);

          match raw_type {
            RawType::I8  => (n as i8).to_le_bytes().to_vec(),
            RawType::I16 => (n as i16).to_le_bytes().to_vec(),
            RawType::I32 => (n as i32).to_le_bytes().to_vec(),
            RawType::U8  => (n as u8).to_le_bytes().to_vec(),
            RawType::U16 => (n as u16).to_le_bytes().to_vec(),
            RawType::U32 => (n as u32).to_le_bytes().to_vec(),
            _ => unreachable!(),
          }
        } else {
          return Err(Error::InvalidArgument(format!("expected number, found {:?}", input)))
        }
      },
    })
  }
}
