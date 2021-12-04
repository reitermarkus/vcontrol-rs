use std::convert::TryInto;
use std::fmt;

#[cfg(feature = "impl_json_schema")]
use schemars::JsonSchema;
use serde::{Serialize, Serializer, Deserialize, Deserializer, de};

#[cfg_attr(feature = "impl_json_schema", derive(JsonSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitTimes {
  mon: CircuitTime,
  tue: CircuitTime,
  wed: CircuitTime,
  thu: CircuitTime,
  fri: CircuitTime,
  sat: CircuitTime,
  sun: CircuitTime,
}

impl CircuitTimes {
  pub fn from_bytes(bytes: &[u8]) -> Self {
    Self {
      mon: CircuitTime::from_bytes(&bytes[0..8]),
      tue: CircuitTime::from_bytes(&bytes[8..16]),
      wed: CircuitTime::from_bytes(&bytes[16..24]),
      thu: CircuitTime::from_bytes(&bytes[24..32]),
      fri: CircuitTime::from_bytes(&bytes[32..40]),
      sat: CircuitTime::from_bytes(&bytes[40..48]),
      sun: CircuitTime::from_bytes(&bytes[48..56]),
    }
  }

  pub fn to_bytes(&self) -> [u8; 56] {
    let mon = self.mon.to_bytes();
    let tue = self.tue.to_bytes();
    let wed = self.wed.to_bytes();
    let thu = self.thu.to_bytes();
    let fri = self.fri.to_bytes();
    let sat = self.sat.to_bytes();
    let sun = self.sun.to_bytes();

    [
      mon[0], mon[1], mon[2], mon[3], mon[4], mon[5], mon[6], mon[7],
      tue[0], tue[1], tue[2], tue[3], tue[4], tue[5], tue[6], tue[7],
      wed[0], wed[1], wed[2], wed[3], wed[4], wed[5], wed[6], wed[7],
      thu[0], thu[1], thu[2], thu[3], thu[4], thu[5], thu[6], thu[7],
      fri[0], fri[1], fri[2], fri[3], fri[4], fri[5], fri[6], fri[7],
      sat[0], sat[1], sat[2], sat[3], sat[4], sat[5], sat[6], sat[7],
      sun[0], sun[1], sun[2], sun[3], sun[4], sun[5], sun[6], sun[7],
    ]
  }
}

#[cfg_attr(feature = "impl_json_schema", derive(JsonSchema))]
#[derive(Clone, Serialize, Deserialize)]
pub struct CircuitTime([Option<TimeSpan>; 4]);

impl CircuitTime {
  pub fn from_bytes(bytes: &[u8]) -> Self {
    Self([
      Time::from_byte(bytes[0]).zip(Time::from_byte(bytes[1])).map(|(from, to)| TimeSpan { from, to }),
      Time::from_byte(bytes[2]).zip(Time::from_byte(bytes[3])).map(|(from, to)| TimeSpan { from, to }),
      Time::from_byte(bytes[4]).zip(Time::from_byte(bytes[5])).map(|(from, to)| TimeSpan { from, to }),
      Time::from_byte(bytes[6]).zip(Time::from_byte(bytes[7])).map(|(from, to)| TimeSpan { from, to }),
    ])
  }

  pub fn to_bytes(&self) -> [u8; 8] {
    let timespan1 = self.0[0].map(|t| t.to_bytes()).unwrap_or([0xff, 0xff]);
    let timespan2 = self.0[1].map(|t| t.to_bytes()).unwrap_or([0xff, 0xff]);
    let timespan3 = self.0[2].map(|t| t.to_bytes()).unwrap_or([0xff, 0xff]);
    let timespan4 = self.0[3].map(|t| t.to_bytes()).unwrap_or([0xff, 0xff]);

    [
      timespan1[0], timespan1[1],
      timespan2[0], timespan2[1],
      timespan3[0], timespan3[1],
      timespan4[0], timespan4[1],
    ]
  }
}

impl fmt::Display for CircuitTime {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut comma = false;
    for timespan in self.0 {
      if comma {
        write!(f, ", ")?;
      }

      if let Some(timespan) = timespan {
        write!(f, "{}", timespan)?;
      } else {
        write!(f, "--:-- – --:--")?;
      }

      comma = true;
    }

    Ok(())
  }
}

impl fmt::Debug for CircuitTime {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "CircuitTime(")?;
    fmt::Display::fmt(self, f)?;
    write!(f, ")")
  }
}

#[cfg_attr(feature = "impl_json_schema", derive(JsonSchema))]
#[derive(Clone, Copy, Serialize, Deserialize)]
struct TimeSpan {
  from: Time,
  to: Time,
}

impl TimeSpan {
  pub fn to_bytes(&self) -> [u8; 2] {
    [self.from.to_byte(), self.to.to_byte()]
  }
}

impl fmt::Display for TimeSpan {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} – {}", self.from, self.to)
  }
}

#[cfg_attr(feature = "impl_json_schema", derive(JsonSchema))]
#[derive(Clone, Copy, Serialize, Deserialize)]
struct Time {
  hh: u8,
  mm: u8,
}

impl Time {
  pub const fn from_byte(byte: u8) -> Option<Self> {
    match byte {
      0xff => None,
      byte => {
        let hh = byte >> 3;
        let mm = (byte & 0b111) * 10;

        Some(Self { hh, mm })
      },
    }
  }

  pub const fn to_byte(&self) -> u8 {
    self.hh << 3 | (self.mm / 10)
  }
}

impl fmt::Display for Time {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:02}:{:02}", self.hh, self.mm)
  }
}
