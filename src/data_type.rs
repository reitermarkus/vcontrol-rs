use std::mem;

use phf;
use serde::Deserialize;

use crate::{Error, Value, RawType, types::{self, SysTime, CycleTimes}};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataType {
  Int,
  Double,
  String,
  Array,
  SysTime,
  CycleTimes,
  Error,
}

impl DataType {
  pub fn bytes_to_output(&self, raw_type: RawType, bytes: &[u8], factor: Option<f64>, mapping: &Option<phf::map::Map<i32, &'static str>>, bit_pos: usize, bit_len: usize) -> Result<Value, Error> {
    if bytes.iter().all(|&b| b == 0xff) {
      return Ok(Value::Empty)
    }

    Ok(match self {
      Self::SysTime => Value::SysTime(SysTime::from_bytes(bytes)),
      Self::CycleTimes => Value::CycleTimes(CycleTimes::from_bytes(bytes)),
      Self::Error => Value::Error(types::Error::from_bytes(bytes)),
      Self::String => {
        let end = bytes.iter().position(|&c| c == b'\0').unwrap_or(bytes.len());
        Value::String(String::from_utf8(bytes[..end].to_vec()).unwrap().to_owned())
      },
      Self::Array => Value::Array(bytes.to_vec()),
      t => {
        macro_rules! int_from_bytes {
          ($ty:ty) => {{
            let mut n: $ty = 0;

            #[allow(arithmetic_overflow)]
            for (_i, &b) in bytes.into_iter().rev().enumerate() {
              n = (n << 8) | (b as $ty);
            }

            if bit_len > 0 {
              n = ((n << bit_pos) >> (mem::size_of::<$ty>() - bit_len))
            }

            n as i64
          }}
        }

        let n = match raw_type {
          RawType::I8 =>  int_from_bytes!(i8),
          RawType::I16 => int_from_bytes!(i16),
          RawType::I32 => int_from_bytes!(i32),
          RawType::U8 =>  int_from_bytes!(u8),
          RawType::U16 => int_from_bytes!(u16),
          RawType::U32 => int_from_bytes!(u32),
          RawType::Array => unreachable!(),
        };

        match t {
          Self::Int => {
            if let Some(mapping) = mapping {
              if let Some(text) = mapping.get(&(n as i32)) {
                return Ok(Value::String((*text).to_string()))
              }

              return Err(Error::UnknownEnumVariant(format!("No enum mapping found for {}.", n)))
            }

            Value::Int(n)
          },
          Self::Double => Value::Double(n as f64 / factor.unwrap_or(1.0)),
          _ => unreachable!(),
        }
      }
    })
  }

  pub fn input_to_bytes(&self, input: &Value, raw_type: RawType, factor: Option<f64>, mapping: &Option<phf::map::Map<i32, &'static str>>) -> Result<Vec<u8>, Error> {
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
      Self::CycleTimes => {
        if let Value::CycleTimes(cycletimes) = input {
          cycletimes.to_bytes().to_vec()
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
