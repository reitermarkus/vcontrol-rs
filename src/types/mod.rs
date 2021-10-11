use std::fmt;
use std::hash::Hasher;

mod cycle_time;
pub use self::cycle_time::CycleTime;

mod error;
pub use self::error::Error;

mod sys_time;
pub use self::sys_time::SysTime;

#[derive(PartialEq, Eq, Hash)]
pub(crate) enum Bytes {
  One([u8; 1]),
  Two([u8; 2]),
}

impl fmt::Debug for Bytes {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Bytes::One(bytes) => write!(f, "Bytes::One({:?})", bytes),
      Bytes::Two(bytes) => write!(f, "Bytes::Two({:?})", bytes),
    }
  }
}

impl Bytes {
  #[track_caller]
  pub fn from_bytes(bytes: &[u8]) -> Bytes {
    match bytes.len() {
      1 => Bytes::One([bytes[0]]),
      2 => Bytes::Two([bytes[0], bytes[1]]),
      _ => unreachable!("from_bytes"),
    }
  }

  pub fn to_bytes(&self) -> Vec<u8> {
    match self {
      Bytes::One(bytes) => bytes.to_vec(),
      Bytes::Two(bytes) => bytes.to_vec(),
    }
  }
}

impl phf_shared::PhfHash for Bytes {
  #[inline]
  fn phf_hash<H: Hasher>(&self, state: &mut H) {
    match self {
      Bytes::One(bytes) => bytes.to_vec().phf_hash(state),
      Bytes::Two(bytes) => bytes.to_vec().phf_hash(state),
    }
  }
}

impl phf_shared::FmtConst for Bytes {
  #[inline]
  fn fmt_const(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
