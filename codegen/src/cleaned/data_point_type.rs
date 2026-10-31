use std::collections::BTreeSet;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DataPointType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub address: Option<String>,
  pub event_types: BTreeSet<u32>,
  pub event_type_groups: Vec<EventTypeGroup>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub f0: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub f0_till: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub identification: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub identification_extension: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub identification_extension_till: Option<String>,
  pub name: String,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub options: Vec<String>,
  pub status_event_type_id: u8,
}

#[derive(Debug, Clone, Serialize)]
pub struct EventTypeGroup {
  pub id: u32,
  pub name: String,
  pub parent_id: Option<u32>,
  pub event_type_ids: Vec<u32>,
}
