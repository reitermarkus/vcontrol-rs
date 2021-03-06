use phf;
use serde::de::{self, Deserialize, Deserializer};

use crate::{Error, Value, FromBytes, ToBytes, types::{Bytes, SysTime, CycleTime}};

#[derive(Debug, Clone)]
pub(crate) enum Unit {
  I8,
  I16,
  I32,
  U8,
  U16,
  U32,
  SysTime,
  CycleTime,
}

impl<'de> Deserialize<'de> for Unit {
  fn deserialize<D>(deserializer: D) -> Result<Unit, D::Error>
  where
      D: Deserializer<'de>,
  {
    match String::deserialize(deserializer)?.as_str() {
      "i8" => Ok(Unit::I8),
      "i16" => Ok(Unit::I16),
      "i32" => Ok(Unit::I32),
      "u8" => Ok(Unit::U8),
      "u16" => Ok(Unit::U16),
      "u32" => Ok(Unit::U32),
      "systime" => Ok(Unit::SysTime),
      "cycletime" => Ok(Unit::CycleTime),
      variant => Err(de::Error::unknown_variant(&variant, &["i8", "i16", "i32", "u8", "u16", "u32", "systime", "cycletime"])),
    }
  }
}

impl Unit {
  pub fn bytes_to_output(&self, bytes: &[u8], factor: f64, mapping: &Option<phf::map::Map<Bytes, &'static str>>) -> Result<Value, Error> {
    if let Some(mapping) = mapping {
      if let Some(text) = mapping.get(&Bytes::from_bytes(bytes)) {
        return Ok(Value::String((*text).to_string()))
      }

      return Err(Error::UnknownEnumVariant(format!("No enum mapping found for [{}].", bytes.iter().map(|byte| format!("0x{:02X}", byte)).collect::<Vec<String>>().join(", "))))
    }

    let n = match self {
      Unit::SysTime => return Ok(Value::SysTime(SysTime::from_bytes(bytes))),
      Unit::CycleTime => return Ok(Value::CycleTime(CycleTime::from_bytes(bytes))),
      Unit::I8 => i64::from(i8::from_bytes(bytes).to_le()),
      Unit::I16 => i64::from(i16::from_bytes(bytes).to_le()),
      Unit::I32 => i64::from(i32::from_bytes(bytes).to_le()),
      Unit::U8 => i64::from(u8::from_bytes(bytes).to_le()),
      Unit::U16 => i64::from(u16::from_bytes(bytes).to_le()),
      Unit::U32 => i64::from(u32::from_bytes(bytes).to_le()),
    };

    Ok(Value::Number(n as f64 / factor))
  }

  pub fn input_to_bytes(&self, input: &Value, factor: f64, mapping: &Option<phf::map::Map<Bytes, &'static str>>) -> Result<Vec<u8>, Error> {
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
      Unit::SysTime => {
        if let Value::SysTime(systime) = input {
          systime.to_bytes()
        } else {
          return Err(Error::InvalidArgument(format!("expected systime, found {:?}", input)))
        }
      },
      Unit::CycleTime => {
        if let Value::CycleTime(cycletime) = input {
          cycletime.to_bytes()
        } else {
          return Err(Error::InvalidArgument(format!("expected cycletime, found {:?}", input)))
        }
      },
      _ => {
        if let Value::Number(n) = input {
          let n = n * factor;

          match self {
            Unit::I8  => (n as i8).to_bytes(),
            Unit::I16 => (n as i16).to_bytes(),
            Unit::I32 => (n as i32).to_bytes(),
            Unit::U8  => (n as u8).to_bytes(),
            Unit::U16 => (n as u16).to_bytes(),
            Unit::U32 => (n as u32).to_bytes(),
            _ => unreachable!(),
          }
        } else {
          return Err(Error::InvalidArgument(format!("expected number, found {:?}", input)))
        }
      },
    })
  }
}
