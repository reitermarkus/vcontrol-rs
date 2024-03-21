use serde::{Deserialize, de::IgnoredAny};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventType {
  #[serde(rename = "Name")]
  pub name: Option<String>,
  #[serde(rename = "ID")]
  pub id: String,
  #[serde(default, rename = "Description")]
  pub description: String,
  #[serde(default, rename = "Active")]
  #[allow(unused)]
  pub active: IgnoredAny,
  #[serde(rename = "Priority")]
  pub priority: Option<u8>,
  #[serde(rename = "DataType")]
  pub data_type: Option<String>,
  #[serde(rename = "SDKDataType")]
  pub sdk_data_type: String,
  #[serde(rename = "ALZ")]
  pub alz: Option<u8>,
  #[serde(rename = "LowerBorder")]
  pub lower_border: Option<u8>,
  #[serde(rename = "UpperBorder")]
  pub upper_border: Option<u8>,
  #[serde(rename = "Stepping")]
  pub stepping: Option<u8>,
  #[serde(rename = "AccessMode")]
  pub access_mode: String,
  #[serde(rename = "Conversion")]
  pub conversion: Option<String>,
  #[serde(rename = "ConversionFactor")]
  pub conversion_factor: Option<f64>,
  #[serde(rename = "ConversionOffset")]
  pub conversion_offset: Option<f64>,
  #[serde(rename = "Address")]
  pub address: String, // HexU16PrefixUpper,
  #[serde(default, rename = "PrefixRead")]
  #[allow(unused)]
  pub prefix_read: IgnoredAny,
  #[serde(default, rename = "VitocomChannelID")]
  #[allow(unused)]
  pub vitocom_channel_id: IgnoredAny,
  #[serde(rename = "FCRead")]
  pub fc_read: String,
  #[serde(rename = "FCWrite")]
  pub fc_write: String,
  #[serde(default, rename = "PrefixWrite")]
  #[allow(unused)]
  pub prefix_write: IgnoredAny,
  #[serde(rename = "Parameter")]
  pub parameter: String,
  #[serde(rename = "BlockLength")]
  pub block_length: u16,
  #[serde(rename = "BytePosition")]
  pub byte_position: u8,
  #[serde(rename = "ByteLength")]
  pub byte_length: u16,
  #[serde(default, rename = "BitPosition")]
  pub bit_position: u8,
  #[serde(default, rename = "BitLength")]
  pub bit_length: u8,
  #[serde(rename = "BlockFactor")]
  pub block_factor: Option<u8>,
  #[serde(rename = "OptionList")]
  pub option_list: Option<String>,
  #[serde(rename = "ValueList")]
  pub value_list: Option<String>,
  #[serde(rename = "MappingType")]
  pub mapping_type: Option<u8>,
  #[serde(default, rename = "RPCHandler")]
  #[allow(unused)]
  pub rpc_handler: IgnoredAny,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventTypes {
  #[serde(rename = "EventType")]
  pub event_type: Vec<EventType>,
}
