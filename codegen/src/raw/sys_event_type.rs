use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SysEventType {
  pub access_mode: String,
  pub address: String,
  pub bit_length: u8,
  pub bit_position: u8,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub block_factor: Option<u8>,
  pub block_length: u16,
  pub byte_length: u16,
  pub byte_position: u8,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub conversion: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub conversion_factor: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub conversion_offset: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub default_value: Option<u8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub fc_read: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub fc_write: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub lower_border: Option<u8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mapping_type: Option<u8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub option_list: Vec<String>,
  pub parameter: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub priority: Option<u8>,
  pub sdk_data_type: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stepping: Option<u8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub upper_border: Option<u8>,
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub value_list: BTreeMap<u8, String>,
}
