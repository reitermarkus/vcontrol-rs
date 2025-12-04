use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
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
  MulOffset {
    factor: f64,
    #[serde(default)]
    offset: f64,
  },
  SecToMinute,
  SecToHour,
  HexByteToAsciiByte,
  HexByteToUtf16Byte,
  HexByteToDecimalByte,
  HexByteToVersion,
  FixedStringTerminalZeroes,
  DayMonthBcd,
  DayToDate,
  Estrich,
  RotateBytes,
  IpAddress,
  LastBurnerCheck,
  LastCheckInterval,
}
