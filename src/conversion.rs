use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "conversion", rename_all = "snake_case")]
pub(crate) enum Conversion {
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
  MulOffset {
    #[serde(flatten, rename = "conversion_factor")]
    factor: f64,
    #[serde(flatten, rename = "conversion_offset")]
    offset: f64
  },
  SecToMinute,
  SecToHour,
  HexByteToAsciiByte,
  HexByteToUtf16Byte,
  HexByteToDecimalByte,
  HexByteToVersion,
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
