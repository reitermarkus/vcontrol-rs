use std::fmt;
use std::io;

use crate::Optolink;

mod vs1;
use self::vs1::Vs1;

mod vs2;
use self::vs2::Vs2;

#[derive(Debug, Clone, Copy)]
pub enum Protocol {
  Vs1,
  Vs2,
}

impl Protocol {
  /// Try detecting the protocol automatically.
  pub async fn detect(o: &mut Optolink) -> Option<Self> {
    if Vs2::negotiate(o).await.is_ok() {
      return Some(Self::Vs2)
    }

    if Vs1::negotiate(o).await.is_ok() {
      return Some(Self::Vs1)
    }

    None
  }

  /// Negotiate the protocol.
  pub async fn negotiate(&self, o: &mut Optolink) -> Result<(), io::Error> {
    match self {
      Self::Vs1 => Vs1::negotiate(o).await,
      Self::Vs2 => Vs2::negotiate(o).await,
    }
  }

  /// Reads the value at the address `addr` into `buf`.
  pub async fn get(&self, o: &mut Optolink, addr: u16, buf: &mut [u8]) -> Result<(), io::Error> {
    match self {
      Self::Vs1 => Vs1::get(o, addr, buf).await,
      Self::Vs2 => Vs2::get(o, addr, buf).await,
    }
  }

  /// Writes the given value `value` to the the address `addr`.
  pub async fn set(&self, o: &mut Optolink, addr: u16, value: &[u8]) -> Result<(), io::Error> {
    match self {
      Self::Vs1 => Vs1::set(o, addr, value).await,
      Self::Vs2 => Vs2::set(o, addr, value).await,
    }
  }
}

impl fmt::Display for Protocol {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Vs1 => "VS1",
      Self::Vs2 => "VS2",
    }.fmt(f)
  }
}
