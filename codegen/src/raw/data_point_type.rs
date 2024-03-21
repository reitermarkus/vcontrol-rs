use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DataPointType {
  pub address: String,
  pub name: String,
  pub status_event_type_id: u8,
}
