use core::convert::Infallible;
use std::str::FromStr;

use serde::{Serialize, Deserialize};

use crate::types::{SysTime, CycleTime, Error};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
  Int(i64),
  Double(f64),
  Array(Vec<u8>),
  String(String),
  SysTime(SysTime),
  CycleTime(CycleTime),
  Error(Error),
  Empty
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

    if let Ok(systime) = s.parse::<SysTime>() {
      return Ok(Value::SysTime(systime))
    }

    if let Ok(cycletime) = s.parse::<CycleTime>() {
      return Ok(Value::CycleTime(cycletime))
    }

    Ok(Value::String(s.to_owned()))
  }
}
