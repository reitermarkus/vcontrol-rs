use std::mem;

use phf;
use serde::Deserialize;

use crate::{Conversion, Error, Value, RawType, types::{self, DateTime, CycleTimes}};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataType {
  String,
  Int,
  Double,
  DateTime,
  CycleTimes,
  Error,
  Array,
}

impl DataType {
  pub fn bytes_to_output(&self, raw_type: RawType, bytes: &[u8], conversion: &Conversion, bit_pos: usize, bit_len: usize) -> Result<Value, Error> {
    if bytes.iter().all(|&b| b == 0xff) {
      return Ok(Value::Empty)
    }

    let mut value = match self {
      Self::DateTime => Value::DateTime(DateTime::from_bytes(bytes)),
      Self::CycleTimes => Value::CycleTimes(CycleTimes::from_bytes(bytes)),
      Self::Error => Value::Error(types::Error::from_bytes(bytes)),
      Self::String => {
        let end = bytes.iter().position(|&c| c == b'\0').unwrap_or(bytes.len());
        Value::String(String::from_utf8(bytes[..end].to_vec()).unwrap().to_owned())
      },
      Self::Array => Value::Array(bytes.to_vec()),
      t => {
        let mut n: i32 = 0;

        if bit_len > 0 {
          for i in 0..bit_len {
            let byte = (bit_pos + i) / 8;
            let bit = (bit_pos + i) % 8;
            let bit_mask = 1 << (8 - bit);

            if (bytes[byte] & bit_mask) == 0 {
              n <<= 1;
            } else {
              n = (n << 1) | 0b1;
            }
          }
        } else {
          #[allow(arithmetic_overflow)]
          for &b in bytes.into_iter().rev() {
            n = (n << 8) | (b as i32);
          }
        }

        let n = match raw_type {
          RawType::I8 =>  n as i8 as i64,
          RawType::I16 => n as i16 as i64,
          RawType::I32 => n as i32 as i64,
          RawType::U8 =>  n as u8 as i64,
          RawType::U16 => n as u16 as i64,
          RawType::U32 => n as u32 as i64,
          RawType::Array => unreachable!(),
        };

        match t {
          Self::Int => Value::Int(n as i64),
          Self::Double => Value::Double(n as f64),
          _ => unreachable!(),
        }
      }
    };

    conversion.convert(&mut value);

    Ok(value)
  }

  pub fn input_to_bytes(&self, mut input: Value, raw_type: RawType, conversion: &Conversion) -> Result<Vec<u8>, Error> {
    conversion.convert_back(&mut input);

    Ok(match self {
      Self::DateTime => {
        if let Value::DateTime(date_time) = input {
          date_time.to_bytes().to_vec()
        } else {
          return Err(Error::InvalidArgument(format!("expected DateTime, found {:?}", input)))
        }
      },
      Self::CycleTimes => {
        if let Value::CycleTimes(cycletimes) = input {
          cycletimes.to_bytes().to_vec()
        } else {
          return Err(Error::InvalidArgument(format!("expected CycleTimes, found {:?}", input)))
        }
      },
      _ => {
        if let Value::Double(n) = input {
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
