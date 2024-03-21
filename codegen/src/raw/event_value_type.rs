use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EventValueType {
  pub data_type: String,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub description: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub enum_address_value: Option<i32>,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub enum_replace_value: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub length: Option<u8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub lower_border: Option<f64>,
  pub name: String,
  pub status_type_id: u8,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stepping: Option<f64>,
  pub unit: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub upper_border: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub value_precision: Option<u16>,
}
