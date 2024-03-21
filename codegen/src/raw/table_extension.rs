use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TableExtension {
  pub field_name: String,
  pub internal_data_type: u8,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub internal_default_value: Option<serde_json::Value>,
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub options_value: BTreeMap<String, String>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub pk_fields: Vec<String>,
  pub table_name: String,
}
