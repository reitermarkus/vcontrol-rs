use std::collections::BTreeMap;

use serde::Serialize;
use stringcase::snake_case;

use crate::{
  cleaned::{self, Conversion},
  unique_mapping::UniqueMapping,
};

#[derive(Debug, Serialize)]
pub struct Command {
  pub addr: u16,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bit_len: Option<u8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bit_pos: Option<u8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub block_count: Option<u8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub block_len: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub byte_len: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub byte_pos: Option<u8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub conversion: Option<Conversion>,
  pub data_type: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub lower_border: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mapping: Option<usize>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mapping_type: Option<String>,
  pub mode: String,
  pub name: String,
  pub parameter: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub unit: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub upper_border: Option<f64>,
}

impl Command {
  pub fn from_event_type(
    event_type_id: String,
    event_type: cleaned::EventType,
    mappings_unique: &UniqueMapping<String, BTreeMap<i32, &usize>>,
  ) -> Command {
    let mut data_type = event_type.value_type.or(event_type.sdk_data_type).unwrap();

    let conversion = if let Some(conversion) = event_type.conversion {
      match conversion {
        Conversion::S(conversion) if conversion == "date_time_bcd" => {
          if data_type != "DateTime" {
            panic!("Conversion {conversion} not supported for type {data_type}.")
          }

          None
        },
        Conversion::S(conversion) if conversion == "date_bcd" => {
          if data_type == "DateTime" {
            data_type = "Date".to_owned();
          } else {
            panic!("Conversion {conversion} not supported for type {data_type}.")
          }

          None
        },
        conversion => Some(conversion),
      }
    } else {
      None
    };

    let enum_type = !event_type.value_list.is_empty();
    let mapping = if enum_type { mappings_unique.mapping.get(&event_type_id).cloned() } else { None };

    Command {
      addr: u16::from_str_radix(event_type.address.as_deref().unwrap().strip_prefix("0x").unwrap(), 16).unwrap(),
      bit_len: event_type.bit_length,
      bit_pos: event_type.bit_position,
      block_count: event_type.block_factor,
      block_len: event_type.block_length,
      byte_len: event_type.byte_length,
      byte_pos: event_type.byte_position,
      conversion,
      data_type,
      lower_border: event_type.lower_border,
      mapping,
      mapping_type: event_type.mapping_type,
      mode: event_type.access_mode.to_owned(),
      name: event_type_id.to_owned(),
      parameter: snake_case(event_type.parameter.as_deref().unwrap()),
      unit: event_type.unit,
      upper_border: event_type.upper_border,
    }
  }
}
