use core::hash::Hasher;
use core::fmt;
use std::hash::Hash;

use phf_shared::{PhfHash, FmtConst};

/// Device identifier range used for detecting device type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeviceIdRange {
  pub id: u16,
  pub hardware_index: Option<u8>,
  pub hardware_index_till: Option<u8>,
  pub software_index: Option<u8>,
  pub software_index_till: Option<u8>,
  pub f0: Option<u16>,
  pub f0_till: Option<u16>,
}

impl PhfHash for DeviceIdRange {
  fn phf_hash<H: Hasher>(&self, state: &mut H) {
    self.id.to_be_bytes().phf_hash(state);

    self.hardware_index.map(|b| [1, b]).unwrap_or([0, 0]).phf_hash(state);
    self.hardware_index_till.map(|b| [1, b]).unwrap_or([0, 0]).phf_hash(state);
    self.software_index.map(|b| [1, b]).unwrap_or([0, 0]).phf_hash(state);
    self.software_index_till.map(|b| [1, b]).unwrap_or([0, 0]).phf_hash(state);
    self.f0.map(|b| {
      let bytes = b.to_be_bytes();
      [1, bytes[0], bytes[1]]
    }).unwrap_or([0, 0, 0]).phf_hash(state);
    self.f0_till.map(|b| {
      let bytes = b.to_be_bytes();
      [1, bytes[0], bytes[1]]
    }).unwrap_or([0, 0, 0]).phf_hash(state);
  }
}

impl FmtConst for DeviceIdRange {
    fn fmt_const(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{:?}", self)
    }
}
