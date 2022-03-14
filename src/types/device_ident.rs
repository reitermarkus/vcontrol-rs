use std::fmt;

#[derive(Clone, Copy)]
pub struct DeviceIdent {
    pub id: u16,
    pub hardware_index: u8,
    pub software_index: u8,
    pub protocol_version_lda: u8,
    pub protocol_version_rda: u8,
    pub developer_version: u16,
}

impl fmt::Debug for DeviceIdent {
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

impl DeviceIdent {
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

#[derive(Debug, Clone, Copy)]
pub struct DeviceIdentF0(pub(crate) u16);

impl DeviceIdentF0 {
  pub fn from_bytes(bytes: &[u8]) -> Self {
    Self(u16::from_be_bytes(bytes[0..2].try_into().unwrap()))
  }
}
