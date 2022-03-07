use core::hash::Hasher;
use core::fmt;
use std::hash::Hash;
use std::ops::RangeInclusive;

use phf_shared::{PhfHash, FmtConst};

/// Device identifier range used for detecting device type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeviceIdRange {
  pub id: u16,
  pub hardware_index_range: RangeInclusive<u8>,
  pub software_index_range: RangeInclusive<u8>,
  pub f0_range: RangeInclusive<u16>,
}

impl PhfHash for DeviceIdRange {
  fn phf_hash<H: Hasher>(&self, state: &mut H) {
    let id = self.id.to_be_bytes();
    let f0_start = self.f0_range.start().to_be_bytes();
    let f0_end = self.f0_range.end().to_be_bytes();

    [
      id[0], id[1],
      *self.hardware_index_range.start(), *self.software_index_range.start(),
      *self.hardware_index_range.end(), *self.software_index_range.end(),
      f0_start[0], f0_start[1],
      f0_end[0], f0_end[1],
    ].phf_hash(state)
  }
}

impl FmtConst for DeviceIdRange {
    fn fmt_const(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{:?}", self)
    }
}
