use crate::Value;

macro_rules! convert_double {
  ($value:expr, $op:tt, $n:literal) => {
    if let Value::Double(n) = $value {
      *n = *n $op $n;
      return
    }
  }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Conversion {
  None,
  Div2,
  Div5,
  Div10,
  Div100,
  Div1000,
  Mul2,
  Mul5,
  Mul10,
  Mul100,
  Mul1000,
  MulOffset { factor: f64, offset: f64 },
  Sec2Minute,
  Sec2Hour,
  HexByte2AsciiByte,
  HexByte2Utf16Byte,
  HexByte2DecimalByte,
  HexByte2Version,
  FixedStringTerminalZeroes,
  DateBcd,
  DateTimeBcd,
  DayMonthBcd,
  DayToDate,
  Estrich,
  RotateBytes,
  IpAddress,
  LastBurnerCheck,
  LastCheckInterval,
}

impl Conversion {
  pub fn convert(&self, value: &mut Value) {
    match self {
      Self::None => return,
      Self::Div2 => convert_double!(value, /, 2.0),
      Self::Div5 => convert_double!(value, /, 5.0),
      Self::Div10 => convert_double!(value, /, 10.0),
      Self::Div100 => convert_double!(value, /, 100.0),
      Self::Div1000 => convert_double!(value, /, 1000.0),
      Self::Mul2 => convert_double!(value, *, 2.0),
      Self::Mul5 => convert_double!(value, *, 5.0),
      Self::Mul10 => convert_double!(value, *, 10.0),
      Self::Mul100 => convert_double!(value, *, 100.0),
      Self::MulOffset { factor, offset } => {
        if let Value::Double(n) = value {
          *n = *n * factor + offset;
          return
        }
      },
      Self::Sec2Minute => convert_double!(value, /, 60.0),
      Self::Sec2Hour => convert_double!(value, /, 3600.0),
      Self::HexByte2AsciiByte => {
        if let Value::Array(bytes) = value {
          let s = bytes.iter().filter(|b| **b != b'0').map(|b| char::from(*b)).collect::<String>();
          *value = Value::String(s);
          return
        }
      },
      Self::HexByte2Version => {
        if let Value::Array(bytes) = value {
          *value = Value::String(bytes.iter().map(|b| b.to_string()).collect::<Vec<_>>().join("."));
          return
        }
      },
      Self::DateBcd => {
        if let Value::DateTime(date_time) = value {
          let year = date_time.year();
          let month = date_time.month();
          let day = date_time.day();

          *value = Value::String(format!("{:04}-{:02}-{:02}", year, month, day));
          return
        }
      },
      Self::DateTimeBcd => {
        if let Value::DateTime(_) = value {
          return
        }
      },
      Self::RotateBytes => {
        if let Value::Array(array) = value {
          array.reverse();
          return
        }
      },
      _ => ()
    }

    log::warn!("Conversion {:?} not applicable to value {:?}.", self, value);
  }

  pub fn convert_back(&self, value: &mut Value) {
    match self {
      Self::None => return,
      Self::Div2 => convert_double!(value, *, 2.0),
      Self::Div5 => convert_double!(value, *, 5.0),
      Self::Div10 => convert_double!(value, *, 10.0),
      Self::Div100 => convert_double!(value, *, 100.0),
      Self::Div1000 => convert_double!(value, *, 1000.0),
      Self::Mul2 => convert_double!(value, /, 2.0),
      Self::Mul5 => convert_double!(value, /, 5.0),
      Self::Mul10 => convert_double!(value, /, 10.0),
      Self::Mul100 => convert_double!(value, /, 100.0),
      Self::MulOffset { factor, offset } => {
        if let Value::Double(n) = value {
          *n = (*n - offset) / factor;
          return
        }
      },
      _ => ()
    }

    unimplemented!("{:?}", self);
  }
}
