use std::io::BufRead;

use hex_serde_util::HexU16PrefixUpper;
use quick_xml::DeError;
use serde::{Deserialize, de::IgnoredAny};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventType {
  #[serde(default, rename = "Name")]
  name: String,
  #[serde(rename = "ID")]
  id: String,
  #[serde(default, rename = "Description")]
  description: String,
  #[serde(rename = "Active")]
  active: Option<bool>,
  #[serde(rename = "Priority")]
  priority: Option<u8>,
  #[serde(rename = "DataType")]
  data_type: Option<String>,
  #[serde(rename = "SDKDataType")]
  sdk_data_type: String,
  #[serde(rename = "ALZ")]
  alz: Option<u8>,
  #[serde(rename = "LowerBorder")]
  lower_border: Option<u8>,
  #[serde(rename = "UpperBorder")]
  upper_border: Option<u8>,
  #[serde(rename = "Stepping")]
  stepping: Option<u8>,
  #[serde(rename = "AccessMode")]
  access_mode: String,
  #[serde(default, rename = "Conversion")]
  conversion: String,
  #[serde(default, rename = "ConversionFactor")]
  conversion_factor: u8,
  #[serde(default, rename = "ConversionOffset")]
  conversion_offset: u8,
  #[serde(rename = "Address")]
  address: HexU16PrefixUpper,
  #[serde(default, rename = "PrefixRead")]
  #[allow(unused)]
  prefix_read: IgnoredAny,
  #[serde(default, rename = "VitocomChannelID")]
  #[allow(unused)]
  vitocom_channel_id: IgnoredAny,
  #[serde(rename = "FCRead")]
  fc_read: String,
  #[serde(rename = "FCWrite")]
  fc_write: String,
  #[serde(default, rename = "PrefixWrite")]
  #[allow(unused)]
  prefix_write: IgnoredAny,
  #[serde(rename = "Parameter")]
  parameter: String,
  #[serde(rename = "BlockLength")]
  block_length: u16,
  #[serde(rename = "BytePosition")]
  byte_position: u8,
  #[serde(rename = "ByteLength")]
  byte_length: u16,
  #[serde(default, rename = "BitPosition")]
  bit_position: u8,
  #[serde(default, rename = "BitLength")]
  bit_length: u8,
  #[serde(rename = "BlockFactor")]
  block_factor: Option<u8>,
  #[serde(default, rename = "OptionList")]
  option_list: String,
  #[serde(default, rename = "ValueList")]
  value_list: String,
  #[serde(rename = "MappingType")]
  mapping_type: Option<u8>,
  #[serde(default, rename = "RPCHandler")]
  #[allow(unused)]
  rpc_handler: IgnoredAny,
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
