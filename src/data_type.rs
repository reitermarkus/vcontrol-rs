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
        macro_rules! int_from_bytes {
          ($ty:ty) => {{
            let mut n: $ty = 0;

            #[allow(arithmetic_overflow)]
            for (i, &b) in bytes.into_iter().rev().enumerate() {
              n = (n << 8) | (b as $ty);
            }

            if bit_len > 0 {
              n = ((n << bit_pos) >> (bytes.len() * 8 - bit_len))
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
          Self::Int => Value::Int(n),
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
