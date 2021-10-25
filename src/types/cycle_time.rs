use std::convert::TryInto;
use std::fmt;
use std::str::FromStr;

use serde::{Serialize, Serializer, Deserialize, Deserializer, de};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CycleTimes {
  mon: CycleTime,
  tue: CycleTime,
  wed: CycleTime,
  thu: CycleTime,
  fri: CycleTime,
  sat: CycleTime,
  sun: CycleTime,
}

impl CycleTimes {
  pub fn from_bytes(bytes: &[u8]) -> Self {
    Self {
      mon: CycleTime::from_bytes(&bytes[0..8]),
      tue: CycleTime::from_bytes(&bytes[8..16]),
      wed: CycleTime::from_bytes(&bytes[16..24]),
      thu: CycleTime::from_bytes(&bytes[24..32]),
      fri: CycleTime::from_bytes(&bytes[32..40]),
      sat: CycleTime::from_bytes(&bytes[40..48]),
      sun: CycleTime::from_bytes(&bytes[48..56]),
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

#[derive(Clone)]
pub struct CycleTime([u8; 8]);

impl CycleTime {
  fn byte_to_time(&self, i: usize) -> Option<(u8, u8)> {
    match self.0[i] {
      0xff => None,
      byte => Some((byte >> 3, (byte & 0b111) * 10)),
    }
  }

  fn times(&self) -> [TimeSpan; 4] {
    [
      TimeSpan { from: self.byte_to_time(0).into(), to: self.byte_to_time(1).into() },
      TimeSpan { from: self.byte_to_time(2).into(), to: self.byte_to_time(3).into() },
      TimeSpan { from: self.byte_to_time(4).into(), to: self.byte_to_time(5).into() },
      TimeSpan { from: self.byte_to_time(6).into(), to: self.byte_to_time(7).into() },
    ]
  }

  pub fn from_bytes(bytes: &[u8]) -> Self {
    Self(bytes[..8].try_into().unwrap())
  }

  pub fn to_bytes(&self) -> [u8; 8] {
    self.0
  }
}

#[derive(Serialize, Clone)]
struct TimeSpan {
  from: Time,
  to: Time,
}

impl fmt::Display for TimeSpan {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} – {}", self.from, self.to)
  }
}

#[derive(Serialize, Clone)]
struct Time {
  hh: String,
  mm: String,
}

impl fmt::Display for Time {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:02}:{:02}", self.hh, self.mm)
  }
}

impl From<Option<(u8, u8)>> for Time {
  fn from(tuple: Option<(u8, u8)>) -> Self {
    if let Some((hh, mm)) = tuple {
      Time { hh: format!("{:02}", hh), mm: format!("{:02}", mm) }
    } else {
      Time { hh: "--".into(), mm: "--".into() }
    }
  }
}

impl Serialize for CycleTime {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    #[derive(Serialize)]
    struct TimeSpanFull {
      full: String,
      from: TimeFull,
      to: TimeFull,
    }

    #[derive(Serialize)]
    struct TimeFull {
      full: String,
      hh: String,
      mm: String,
    }

    self.times().iter()
      .map(|ts|
        TimeSpanFull {
          full: ts.to_string(),
          from: TimeFull {
            full: ts.from.to_string(),
            hh: ts.from.hh.to_owned(),
            mm: ts.from.mm.to_owned(),
          },
          to: TimeFull {
            full: ts.to.to_string(),
            hh: ts.to.hh.to_owned(),
            mm: ts.to.mm.to_owned(),
          },
      })
      .collect::<Vec<TimeSpanFull>>()
      .serialize(serializer)
  }
}

impl FromStr for CycleTime {
  type Err = String;

  fn from_str(s: &str) -> Result<CycleTime, Self::Err> {
    Err(format!("could not parse {}, from_str is not implemented for CycleTime", s))
  }
}

impl<'de> Deserialize<'de> for CycleTime {
  fn deserialize<D>(deserializer: D) -> Result<CycleTime, D::Error>
  where
      D: Deserializer<'de>,
  {
    let string = String::deserialize(deserializer)?;
    CycleTime::from_str(&string).map_err(de::Error::custom)
  }
}

impl fmt::Display for CycleTime {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}",
      self.times().iter().map(|timespan| timespan.to_string()).collect::<Vec<String>>().join(","),
    )
  }
}

impl fmt::Debug for CycleTime {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "CycleTime(")?;
    fmt::Display::fmt(self, f)?;
    write!(f, ")")
  }
}
