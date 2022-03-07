use core::convert::Infallible;
use std::str::FromStr;
use std::fmt;

use serde::{Serialize, Deserialize};

use crate::{conversion::Conversion, types::{DateTime, CircuitTimes, Error}};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
  Int(i64),
  Double(f64),
  Array(Vec<u8>),
  String(String),
  DateTime(DateTime),
  CircuitTimes(CircuitTimes),
  Error(Error),
  Empty
}

macro_rules! convert_double {
  ($value:expr, $op:tt, $n:literal) => {
    if let Value::Double(n) = $value {
      *n = *n $op $n;
      return
    }
  }
}

impl Value {
  pub fn convert(&mut self, conversion: &Conversion) {
    match conversion {
      Conversion::Div2 => convert_double!(self, /, 2.0),
      Conversion::Div5 => convert_double!(self, /, 5.0),
      Conversion::Div10 => convert_double!(self, /, 10.0),
      Conversion::Div100 => convert_double!(self, /, 100.0),
      Conversion::Div1000 => convert_double!(self, /, 1000.0),
      Conversion::Mul2 => convert_double!(self, *, 2.0),
      Conversion::Mul5 => convert_double!(self, *, 5.0),
      Conversion::Mul10 => convert_double!(self, *, 10.0),
      Conversion::Mul100 => convert_double!(self, *, 100.0),
      Conversion::MulOffset { factor, offset } => {
        if let Value::Double(n) = self {
          *n = *n * factor + offset;
          return
        }
      },
      Conversion::SecToMinute => convert_double!(self, /, 60.0),
      Conversion::SecToHour => convert_double!(self, /, 3600.0),
      Conversion::HexByteToAsciiByte => {
        if let Value::Array(bytes) = self {
          let s = bytes.iter().filter(|b| **b != b'0').map(|b| char::from(*b)).collect::<String>();
          *self = Value::String(s);
          return
        }
      },
      Conversion::HexByteToVersion => {
        if let Value::Array(bytes) = self {
          *self = Value::String(bytes.iter().map(|b| b.to_string()).collect::<Vec<_>>().join("."));
          return
        }
      },
      Conversion::DateBcd => {
        if let Value::DateTime(date_time) = self {
          let year = date_time.year();
          let month = date_time.month();
          let day = date_time.day();

          *self = Value::String(format!("{:04}-{:02}-{:02}", year, month, day));
          return
        }
      },
      Conversion::DateTimeBcd => {
        if let Value::DateTime(_) = self {
          return
        }
      },
      Conversion::RotateBytes => {
        if let Value::Array(array) = self {
          array.reverse();
          return
        }
      },
      _ => ()
    }

    log::warn!("Conversion {:?} not applicable to value {:?}.", conversion, self);
  }

  pub fn convert_back(&mut self, conversion: &Conversion) {
    match conversion {
      Conversion::Div2 => convert_double!(self, *, 2.0),
      Conversion::Div5 => convert_double!(self, *, 5.0),
      Conversion::Div10 => convert_double!(self, *, 10.0),
      Conversion::Div100 => convert_double!(self, *, 100.0),
      Conversion::Div1000 => convert_double!(self, *, 1000.0),
      Conversion::Mul2 => convert_double!(self, /, 2.0),
      Conversion::Mul5 => convert_double!(self, /, 5.0),
      Conversion::Mul10 => convert_double!(self, /, 10.0),
      Conversion::Mul100 => convert_double!(self, /, 100.0),
      Conversion::MulOffset { factor, offset } => {
        if let Value::Double(n) = self {
          *n = (*n - offset) / factor;
          return
        }
      },
      _ => ()
    }

    unimplemented!("{:?}", self);
  }
}

#[derive(Debug, Serialize)]
pub struct OutputValue {
  pub value: Value,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub unit: Option<&'static str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mapping: Option<&'static phf::map::Map<i32, &'static str>>,
}

impl fmt::Display for OutputValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self.value {
      Value::Int(n) => {
        if let Some(mapping) = self.mapping {
          write!(f, "{}", mapping.get(&(*n as i32)).unwrap())?;
        } else {
          write!(f, "{}", n)?;
        }
      },
      Value::Double(n) => write!(f, "{}", n)?,
      Value::Array(array) => write!(f, "{:?}", array)?,
      Value::DateTime(date_time) => write!(f, "{}", date_time)?,
      Value::Error(error) => {
        write!(f, "{}", self.mapping.unwrap().get(&(error.index() as i32)).unwrap())?;
      },
      Value::CircuitTimes(cycle_times) => write!(f, "{:#?}", cycle_times)?,
      Value::String(string) => write!(f, "{}", string)?,
      Value::Empty => return Ok(()),
    }

    if let Some(unit) = self.unit {
      write!(f, " {}", unit)?;
    }

    Ok(())
  }
}

impl FromStr for Value {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Value, Self::Err> {
    if let Ok(number) = s.parse::<i64>() {
      return Ok(Value::Int(number))
    }

    if let Ok(number) = s.parse::<f64>() {
      return Ok(Value::Double(number))
    }

    if let Ok(date_time) = s.parse::<DateTime>() {
      return Ok(Value::DateTime(date_time))
    }

    // if let Ok(cycletime) = s.parse::<[CircuitTime; 4]>() {
    //   return Ok(Value::CircuitTimes(cycletime))
    // }

    Ok(Value::String(s.to_owned()))
  }
}
