use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::BufReader;

use base64::prelude::*;
use encoding_rs_io::DecodeReaderBytes;
use glob::glob;
use serde::Serialize;

mod raw;

fn value_if_non_empty(value: String) -> Option<String> {
  let value = value.trim();
  if value.is_empty() { None } else { Some(value.to_owned()) }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let mut cultures = BTreeMap::<u8, String>::new();
  let mut translations_raw = BTreeMap::<String, BTreeMap<String, String>>::new();

  for text_resource in glob("src/Textresource_*.xml")? {
    let text_resource = text_resource?;

    let f = File::open(text_resource)?;
    let decoder = DecodeReaderBytes::new(f);
    let io = BufReader::new(decoder);

    let document: raw::DocumentElement = quick_xml::de::from_reader(io)?;

    for culture in document.cultures.culture {
      cultures.insert(culture.id, culture.name);
    }

    for text_resource in document.text_resources.text_resource {
      let name = cultures.get(&text_resource.culture_id).unwrap();

      let value = raw::parse_translation_text(text_resource.value);
      let value = raw::clean_enum_text(&text_resource.label, None, value);

      let inner = translations_raw.entry(text_resource.label).or_insert_with(BTreeMap::new);
      inner.insert(name.clone(), value);
    }
  }

  let f = File::create("translations.raw.yml")?;
  serde_yaml::to_writer(f, &translations_raw)?;

  let reverse_translations_raw: BTreeMap<_, _> = translations_raw
    .into_iter()
    .filter_map(|(k, v)| {
      let text = raw::simplify_translation_text(v.get("de").unwrap());

      if text.is_empty() {
        return None;
      }

      Some((text, k))
    })
    .collect();

  let f = File::create("reverse_translations.raw.yml")?;
  serde_yaml::to_writer(f, &reverse_translations_raw)?;

  let f = File::open("src/DPDefinitions.xml")?;
  let decoder = DecodeReaderBytes::new(f);
  let io = BufReader::new(decoder);

  let import_export_data_holder: raw::ImportExportDataHolder = quick_xml::de::from_reader(io)?;

  let ecn_data_set = import_export_data_holder.ecn_data_set.diff_gram.ecn_data_set;

  let mut versions = BTreeMap::<String, String>::new();
  for version in ecn_data_set.ecn_version {
    versions.insert(version.name, version.value);
  }

  #[derive(Debug, Serialize)]
  struct DataPointType {
    name: String,
    description: String,
    status_event_type_id: u8,
    address: String,
  }
  let mut data_point_types = BTreeMap::<u16, DataPointType>::new();
  for data_point_type in ecn_data_set.ecn_datapoint_type {
    let id = data_point_type.id;
    let data_point_type = DataPointType {
      name: data_point_type.name,
      description: data_point_type.description,
      status_event_type_id: data_point_type.status_event_type_id,
      address: data_point_type.address,
    };

    data_point_types.insert(id, data_point_type);
  }

  let f = File::create("datapoint_definitions.raw.yml")?;
  serde_yaml::to_writer(f, &data_point_types)?;

  let mut data_point_type_event_type_links = BTreeMap::<u16, Vec<u16>>::new();
  for data_point_type_event_type_link in ecn_data_set.ecn_data_point_type_event_type_link {
    let event_type_ids = data_point_type_event_type_links
      .entry(data_point_type_event_type_link.data_point_type_id)
      .or_insert_with(Vec::new);
    event_type_ids.push(data_point_type_event_type_link.event_type_id);
  }

  #[derive(Debug, Serialize)]
  pub struct EventType {
    pub enum_type: bool,
    pub name: String,
    pub address: String,
    pub conversion: String,
    pub description: String,
    pub priority: u8,
    pub filter_criterion: bool,
    pub reporting_criterion: bool,
    pub type_: u8,
    pub url: String,
    pub default_value: String,
  }

  let mut event_types = BTreeMap::<u16, EventType>::new();
  for event_type in ecn_data_set.ecn_event_type {
    let id = event_type.id;
    event_types.insert(
      id,
      EventType {
        enum_type: event_type.enum_type,
        name: event_type.name,
        address: event_type.address,
        conversion: event_type.conversion,
        description: event_type.description,
        priority: event_type.priority,
        filter_criterion: event_type.filter_criterion,
        reporting_criterion: event_type.reporting_criterion,
        type_: event_type.type_,
        url: event_type.url,
        default_value: event_type.default_value,
      },
    );
  }

  #[derive(Debug, Serialize)]
  pub struct EventValueType {
    pub name: String,
    pub enum_address_value: i32,
    pub enum_replace_value: String,
    pub status_type_id: u8,
    pub unit: String,
    pub data_type: String,
    pub length: Option<u8>,
    pub stepping: Option<f32>,
    pub value_precision: Option<u16>,
    pub lower_border: Option<f32>,
    pub uppwer_border: Option<f32>,
    pub description: String,
  }
  let mut event_value_types = BTreeMap::<u16, EventValueType>::new();
  for event_value_type in ecn_data_set.ecn_event_value_type {
    let id = event_value_type.id;
    event_value_types.insert(
      id,
      EventValueType {
        name: event_value_type.name,
        enum_address_value: event_value_type.enum_address_value,
        enum_replace_value: event_value_type.enum_replace_value,
        status_type_id: event_value_type.status_type_id,
        unit: event_value_type.unit,
        data_type: event_value_type.data_type,
        length: event_value_type.length,
        stepping: event_value_type.stepping,
        value_precision: event_value_type.value_precision,
        lower_border: event_value_type.lower_border,
        uppwer_border: event_value_type.uppwer_border,
        description: event_value_type.description,
      },
    );
  }

  let mut event_type_event_value_type_links = BTreeMap::<u16, Vec<u16>>::new();
  for event_type_event_value_type_link in ecn_data_set.ecn_event_type_event_value_type_link {
    let value_types =
      event_type_event_value_type_links.entry(event_type_event_value_type_link.event_type_id).or_insert_with(Vec::new);
    value_types.push(event_type_event_value_type_link.event_value_id);
  }

  #[derive(Debug, Serialize)]
  pub struct TableExtension {
    pub table_name: String,
    pub field_name: String,
    pub label: String,
    pub pk_fields: String,
    pub internal_default_value: Vec<u8>,
    pub internal_data_type: u8,
    pub options_value: String,
  }
  let mut table_extensions = BTreeMap::<u16, TableExtension>::new();
  for table_extension in ecn_data_set.ecn_table_extension {
    let id = table_extension.id;
    table_extensions.insert(
      id,
      TableExtension {
        table_name: table_extension.table_name,
        field_name: table_extension.field_name,
        label: table_extension.label,
        pk_fields: table_extension.pk_fields,
        internal_default_value: table_extension.internal_default_value,
        internal_data_type: table_extension.internal_data_type,
        options_value: table_extension.options_value,
      },
    );
  }

  #[derive(Debug, Serialize)]
  pub struct TableExtensionValue {
    pub ref_id: u16,
    pub pk_value: String,
    pub internal_value: Vec<u8>,
  }
  let mut table_extension_values = BTreeMap::<u32, TableExtensionValue>::new();
  for table_extension_value in ecn_data_set.ecn_table_extension_value {
    let id = table_extension_value.id;
    table_extension_values.insert(
      id,
      TableExtensionValue {
        ref_id: table_extension_value.ref_id,
        pk_value: table_extension_value.pk_value,
        internal_value: table_extension_value.internal_value,
      },
    );
  }

  dbg!(event_type_event_value_type_links);

  return Ok(());

  let input = env::args().nth(1).unwrap();
  dbg!(&input);

  let input = BASE64_STANDARD.decode(input)?;
  let message = nrbf::RemotingMessage::parse(&input).unwrap();

  dbg!(message);

  Ok(())
}
