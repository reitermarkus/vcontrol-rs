use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TableExtensionValue {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub internal_value: Option<serde_json::Value>,
  pub pk_value: Vec<u32>,
  pub ref_id: u16,
}
