use std::io;

use crate::Optolink;

mod kw2;
use self::kw2::Kw2;

mod p300;
use self::p300::P300;

#[derive(Debug, Clone, Copy)]
pub enum Protocol {
  Kw2,
  P300,
}

impl Protocol {
  /// Try detecting the protocol automatically.
  pub fn detect(o: &mut Optolink) -> Option<Self> {
    if P300::negotiate(o).is_ok() {
      return Some(Self::P300)
    }

    if Kw2::negotiate(o).is_ok() {
      return Some(Self::Kw2)
    }

    None
  }

  /// Negotiate the protocol.
  pub fn negotiate(&self, o: &mut Optolink) -> Result<(), io::Error> {
    match self {
      Self::Kw2 => Kw2::negotiate(o),
      Self::P300 => P300::negotiate(o),
    }
  }

  /// Reads the value at the address `addr` into `buf`.
  pub fn get(&self, o: &mut Optolink, addr: u16, buf: &mut [u8]) -> Result<(), io::Error> {
    match self {
      Self::Kw2 => Kw2::get(o, addr, buf),
      Self::P300 => P300::get(o, addr, buf),
    }
  }

  /// Writes the given value `value` to the the address `addr`.
  pub fn set(&self, o: &mut Optolink, addr: u16, value: &[u8]) -> Result<(), io::Error> {
    match self {
      Self::Kw2 => Kw2::set(o, addr, value),
      Self::P300 => P300::set(o, addr, value),
    }
  }
}
