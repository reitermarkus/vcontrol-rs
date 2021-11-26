use crate::Value;

macro_rules! number_conversion {
  ($ty:ident, $op:tt, $n:expr) => {
    enum $ty {}

    impl $ty {
      pub fn convert(value: &mut Value) {
        match value {
          Value::Int(n) => {
            *n = *n $op $n;
          },
          Value::Double(n) => {
            *n = *n $op $n as f64;
          },
          _ => unimplemented!(),
        }
      }
    }
  }
}

number_conversion!(Div2, /, 2);
number_conversion!(Div5, /, 5);
number_conversion!(Div10, /, 10);
number_conversion!(Div100, /, 100);
number_conversion!(Div1000, /, 1000);
number_conversion!(Mul2, *, 2);
number_conversion!(Mul5, *, 5);
number_conversion!(Mul10, *, 10);
number_conversion!(Mul100, *, 100);
number_conversion!(Mul1000, *, 1000);
number_conversion!(Sec2Minute, /, 60);
number_conversion!(Sec2Hour, /, 3600);

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
      Self::None => (),
      Self::Div2 => Div2::convert(value),
      Self::Div5 => Div5::convert(value),
      Self::Div10 => Div10::convert(value),
      Self::Div100 => Div100::convert(value),
      Self::Div1000 => Div1000::convert(value),
      Self::Mul2 => Mul2::convert(value),
      Self::Mul5 => Mul5::convert(value),
      Self::Mul10 => Mul10::convert(value),
      Self::Mul100 => Mul100::convert(value),
      Self::Mul1000 => Mul1000::convert(value),
      Self::MulOffset { factor, offset } => {
        let factor = factor * 10f64.powf(*offset);

        match value {
          Value::Int(n) => {
            let res = *n as f64 * factor;

            if res.fract() == 0.0 {
              *n = res as i64;
            } else {
              *value = Value::Double(res);
            }
          },
          Value::Double(n) => {
            *n = *n as f64 * factor;
          },
          _ => unimplemented!(),
        }
      },
      Self::Sec2Minute => Sec2Minute::convert(value),
      Self::Sec2Hour => Sec2Hour::convert(value),
      Self::HexByte2AsciiByte => {
        match value {
          Value::Array(bytes) => {
            *value = Value::String(bytes.iter().map(|b| format!("{:02X}", b)).collect());
          },
          _ => unimplemented!(),
        }
      },
      Self::HexByte2Version => {
        match value {
          Value::Array(bytes) => {
            *value = Value::String(bytes.iter().map(|b| b.to_string()).collect::<Vec<_>>().join("."));
          },
          _ => unimplemented!(),
        }
      },
      Self::DateBcd => {
        match value {
          Value::DateTime(date_time) => {
            let year = date_time.year();
            let month = date_time.month();
            let day = date_time.day();

            *value = Value::String(format!("{:04}-{:02}-{:02}", year, month, day));
          },
          _ => unimplemented!(),
        }
      },
      Self::DateTimeBcd => {
        match value {
          Value::DateTime(_) => (),
          _ => unimplemented!(),
        }
      },
      Self::RotateBytes => {
        match value {
          Value::Int(n) => {
            // TODO: Check what `RotateBytes` is actually supposed to do.
            *n = (*n).rotate_left(8);
          },
          _ => unimplemented!(),
        }
      },
      _ => {
        unimplemented!("{:?}", self)
      }
    }
  }

  pub fn convert_back(&self, value: &mut Value) {
    match self {
      Self::None => (),
      Self::Div2 => Mul2::convert(value),
      Self::Div5 => Mul5::convert(value),
      Self::Div10 => Mul10::convert(value),
      Self::Div100 => Mul100::convert(value),
      Self::Div1000 => Mul1000::convert(value),
      Self::Mul2 => Div2::convert(value),
      Self::Mul5 => Div5::convert(value),
      Self::Mul10 => Div10::convert(value),
      Self::Mul100 => Div100::convert(value),
      Self::Mul1000 => Div1000::convert(value),
      Self::MulOffset { factor, offset } => {
        let factor = factor * 10f64.powf(*offset);

        match value {
          Value::Int(n) => {
            let res = *n as f64 / factor;

            if res.fract() == 0.0 {
              *n = res as i64;
            } else {
              *value = Value::Double(res);
            }
          },
          Value::Double(n) => {
            *n = *n as f64 / factor;
          },
          _ => unimplemented!(),
        }
      },
      _ => {
        unimplemented!("{:?}", self)
      }
    }
  }
}
