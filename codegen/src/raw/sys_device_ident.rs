use std::io::BufRead;

use hex_serde_util::HexU16PrefixUpper;
use quick_xml::DeError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EventType {
  #[serde(rename = "Name")]
  name: String,
  #[serde(rename = "ID")]
  id: String,
  #[serde(rename = "Active")]
  active: bool,
  #[serde(rename = "Priority")]
  priority: u8,
  #[serde(rename = "DataType")]
  data_type: String,
  #[serde(rename = "SDKDataType")]
  sdk_data_type: String,
  #[serde(rename = "AccessMode")]
  access_mode: String,
  #[serde(rename = "Address")]
  address: HexU16PrefixUpper,
  #[serde(rename = "FCRead")]
  fc_read: String,
  #[serde(rename = "FCWrite")]
  fc_write: String,
  #[serde(rename = "Parameter")]
  parameter: String,
  #[serde(rename = "BlockLength")]
  block_length: u8,
  #[serde(rename = "BytePosition")]
  byte_position: u8,
  #[serde(rename = "ByteLength")]
  byte_length: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventTypes {
  #[serde(rename = "EventType")]
  event_type: Vec<EventType>,
}

impl EventTypes {
  pub fn from_reader<R>(reader: R) -> Result<Self, DeError>
  where
    R: BufRead,
  {
    quick_xml::de::from_reader(reader)
  }
}
