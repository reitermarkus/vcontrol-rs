use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ConversionInner {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub factor: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub offset: Option<f64>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Conversion {
  S(String),
  M(BTreeMap<String, ConversionInner>),
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum EventValueType {
  Single {
    value_type: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    lower_border: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upper_border: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stepping: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<String>,
  },
  Multiple {
    value_list: BTreeMap<i32, String>,
  },
}

#[derive(Debug, Serialize)]
pub struct EventType {
  pub access_mode: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub address: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bit_length: Option<u8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bit_position: Option<u8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub block_factor: Option<u8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub block_length: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub byte_length: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub byte_position: Option<u8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub conversion: Option<Conversion>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub conversion_factor: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub conversion_offset: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub default_value: Option<serde_json::Value>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub enum_type: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub fc_read: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub fc_write: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub filter_criterion: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub lower_border: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mapping_type: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub option_list: Vec<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub parameter: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub priority: Option<u8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reporting_criterion: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sdk_data_type: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stepping: Option<f64>,
  pub type_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub unit: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub upper_border: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url: Option<String>,
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub value_list: BTreeMap<i32, String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub value_type: Option<String>,
}
