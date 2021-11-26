use core::convert::Infallible;
use std::str::FromStr;
use std::fmt;

use serde::{Serialize, Deserialize};

use crate::types::{DateTime, CircuitTimes, Error};

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
