use core::hash::Hasher;
use core::fmt;
use std::hash::Hash;

use phf_shared::{PhfHash, FmtConst};

/// Device identifier.
#[derive(Clone, Copy)]
pub struct DeviceId {
  pub(crate) id: u16,
  pub(crate) hardware_index: u8,
  pub(crate) software_index: u8,
  pub(crate) protocol_version_lda: u8,
  pub(crate) protocol_version_rda: u8,
  pub(crate) developer_version: u16,
}

impl fmt::Debug for DeviceId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "Device ID 0x{:04X}, HX {}, SW {}, LDA {}, RDA {}, DEV 0x{:04X}",
      self.id,
      self.hardware_index,
      self.software_index,
      self.protocol_version_lda,
      self.protocol_version_rda,
      self.developer_version
    )
  }
}

impl DeviceId {
  pub fn from_bytes(bytes: &[u8]) -> Self {
    Self {
      id: u16::from_be_bytes(bytes[0..2].try_into().unwrap()),
      hardware_index: u8::from_be_bytes(bytes[2..3].try_into().unwrap()),
      software_index: u8::from_be_bytes(bytes[3..4].try_into().unwrap()),
      protocol_version_lda: u8::from_be_bytes(bytes[4..5].try_into().unwrap()),
      protocol_version_rda: u8::from_be_bytes(bytes[5..6].try_into().unwrap()),
      developer_version: u16::from_be_bytes(bytes[6..8].try_into().unwrap()),
    }
  }

  pub fn to_bytes(&self) -> [u8; 8] {
    let id = self.id.to_be_bytes();
    let hardware_index = self.hardware_index.to_be_bytes();
    let software_index = self.software_index.to_be_bytes();
    let protocol_version_lda = self.protocol_version_lda.to_be_bytes();
    let protocol_version_rda = self.protocol_version_rda.to_be_bytes();
    let developer_version = self.developer_version.to_be_bytes();

    [
      id[0], id[1],
      hardware_index[0],
      software_index[0],
      protocol_version_lda[0],
      protocol_version_rda[0],
      developer_version[0], developer_version[1],
    ]
  }
}

/// Device F0 identifier.
#[derive(Debug, Clone, Copy)]
pub struct DeviceIdF0(pub(crate) u16);

impl DeviceIdF0 {
  pub fn from_bytes(bytes: &[u8]) -> Self {
    Self(u16::from_be_bytes(bytes[0..2].try_into().unwrap()))
  }
}

/// Device identifier range used for detecting device type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeviceIdRange {
  pub(crate) id: u16,
  pub(crate) hardware_index: Option<u8>,
  pub(crate) hardware_index_till: Option<u8>,
  pub(crate) software_index: Option<u8>,
  pub(crate) software_index_till: Option<u8>,
  pub(crate) f0: Option<u16>,
  pub(crate) f0_till: Option<u16>,
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
