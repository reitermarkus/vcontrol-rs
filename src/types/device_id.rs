#[cfg(feature = "impl_json_schema")]
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// Device identifier.
#[cfg_attr(feature = "impl_json_schema", derive(JsonSchema))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DeviceId {
  pub(crate) id: u16,
  pub(crate) hardware_index: u8,
  pub(crate) software_index: u8,
  pub(crate) protocol_version_lda: u8,
  pub(crate) protocol_version_rda: u8,
  pub(crate) developer_version: u16,
}

impl DeviceId {
  #[allow(unused)]
  pub fn from_bytes(bytes: &[u8; 8]) -> Self {
    Self {
      id: u16::from_be_bytes([bytes[0], bytes[1]]),
      hardware_index: bytes[2],
      software_index: bytes[3],
      protocol_version_lda: bytes[4],
      protocol_version_rda: bytes[5],
      developer_version: u16::from_be_bytes([bytes[6], bytes[7]]),
    }
  }

  #[allow(unused)]
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
#[cfg_attr(feature = "impl_json_schema", derive(JsonSchema))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DeviceIdF0(pub(crate) u16);

impl DeviceIdF0 {
  pub fn from_bytes(bytes: &[u8; 2]) -> Self {
    Self(u16::from_be_bytes(bytes[0..2].try_into().unwrap()))
  }
}
