use core::convert::Infallible;
use std::str::FromStr;

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

#[derive(Debug)]
pub enum ValueMeta {
  None,
  Unit(&'static str),
  Mapping(&'static phf::map::Map<i32, &'static str>),
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
