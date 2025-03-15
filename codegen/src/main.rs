use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

use base64::prelude::*;
use convert_case::{Case, Casing};
use encoding_rs_io::DecodeReaderBytes;
use glob::glob;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use serde_json::Value;

mod raw;

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
      let value = raw::clean_enum_text(Some(&text_resource.label), None, value);

      let inner = translations_raw.entry(text_resource.label).or_insert_with(BTreeMap::new);
      inner.insert(name.clone(), value);
    }
  }

  let mut f = File::create("translations.raw.json")?;
  serde_json::to_writer_pretty(&mut f, &translations_raw)?;
  writeln!(f)?;

  let reverse_translations_raw: BTreeMap<_, _> = translations_raw
    .iter()
    .filter_map(|(k, v)| {
      let text = raw::simplify_translation_text(v.get("de").unwrap());

      if text.is_empty() {
        return None;
      }

      Some((text, k))
    })
    .collect();

  let mut f = File::create("reverse_translations.raw.json")?;
  serde_json::to_writer_pretty(&mut f, &reverse_translations_raw)?;
  writeln!(f)?;

  let f = File::open("src/DPDefinitions.xml")?;
  let decoder = DecodeReaderBytes::new(f);
  let io = BufReader::new(decoder);

  let import_export_data_holder: raw::ImportExportDataHolder = quick_xml::de::from_reader(io)?;

  let ecn_data_set = import_export_data_holder.ecn_data_set.diff_gram.ecn_data_set;

  let mut versions = BTreeMap::<String, String>::new();
  for version in ecn_data_set.ecn_version {
    versions.insert(version.name.to_case(Case::Snake), version.value);
  }

  let mut f = File::create("versions.used.json")?;
  serde_json::to_writer_pretty(&mut f, &versions)?;
  writeln!(f)?;

  #[derive(Debug, Serialize)]
  struct DataPointType {
    address: String,
    name: String,
    status_event_type_id: u8,
    event_types: Vec<u16>,
  }
  let mut data_point_types = BTreeMap::<u16, DataPointType>::new();
  for data_point_type in ecn_data_set.ecn_datapoint_type {
    let id = data_point_type.id;
    let data_point_type = DataPointType {
      address: data_point_type.address,
      name: data_point_type.name,
      status_event_type_id: data_point_type.status_event_type_id,
      event_types: Vec::new(),
    };

    data_point_types.insert(id, data_point_type);
  }

  for data_point_type_event_type_link in ecn_data_set.ecn_data_point_type_event_type_link {
    let data_point_type = data_point_types.get_mut(&data_point_type_event_type_link.data_point_type_id).unwrap();
    data_point_type.event_types.push(data_point_type_event_type_link.event_type_id);
  }

  #[derive(Debug, Serialize)]
  pub struct EventType {
    pub access_mode: &'static str,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub enum_type: bool,
    pub filter_criterion: bool,
    pub name: String,
    pub priority: u8,
    pub reporting_criterion: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub value_types: Vec<u16>,
  }

  fn value_if_non_empty(value: String) -> Option<String> {
    if value.is_empty() {
      return None;
    }

    Some(value)
  }

  let mut event_types = BTreeMap::<u16, EventType>::new();
  for event_type in ecn_data_set.ecn_event_type {
    let id = event_type.id;
    event_types.insert(
      id,
      EventType {
        access_mode: match event_type.type_ {
          1 => "read",
          2 => "write",
          3 => "read_write",
          t => unreachable!("unknown type: {t}"),
        },
        address: raw::strip_address(&event_type.address).into_owned(),
        conversion: raw::parse_conversion(&event_type.conversion),
        default_value: match value_if_non_empty(event_type.default_value) {
          Some(value) => {
            let value = value.replace(",", ".");
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&value) {
              Some(value)
            } else {
              lazy_static! {
                static ref DATE_REGEX: Regex = Regex::new(r"^(?<day>\d{2})\.(?<month>\d{2})\.(?<year>\d{4})$").unwrap();
                static ref DATE_TIME_REGEX: Regex = Regex::new(
                  r"^(?<day>\d{2})\.(?<month>\d{2})\.(?<year>\d{4})\s+(?<hour>\d{2}):(?<minute>\d{2}):(?<second>\d{2})$"
                )
                .unwrap();
              }

              if let Some(captures) = DATE_REGEX.captures(&value) {
                let day = captures["day"].parse::<u8>().unwrap();
                let month = captures["month"].parse::<u8>().unwrap();
                let year = captures["year"].parse::<u16>().unwrap();
                Some(Value::String(format!("{year:04}-{month:02}-{day:02}")))
              } else if let Some(captures) = DATE_TIME_REGEX.captures(&value) {
                let day = captures["day"].parse::<u8>().unwrap();
                let month = captures["month"].parse::<u8>().unwrap();
                let year = captures["year"].parse::<u16>().unwrap();
                let hour = captures["hour"].parse::<u8>().unwrap();
                let minute = captures["minute"].parse::<u8>().unwrap();
                let second = captures["second"].parse::<u8>().unwrap();
                Some(Value::String(format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}")))
              } else if let Some(value) = value.strip_prefix("0x") {
                use serde_with::DeserializeAs;

                let deserializer: serde::de::value::StrDeserializer<'_, serde::de::value::Error> =
                  serde::de::value::StrDeserializer::new(value);
                let bytes: Vec<u8> =
                  serde_with::hex::Hex::<serde_with::formats::Uppercase>::deserialize_as(deserializer).unwrap();
                Some(serde_json::to_value(bytes).unwrap())
              } else if matches!(value.as_str(), "" | "--" | "TBD") {
                None
              } else {
                Some(Value::String(value))
              }
            }
          },
          None => None,
        },
        description: value_if_non_empty(event_type.description),
        enum_type: event_type.enum_type,
        filter_criterion: event_type.filter_criterion,
        name: event_type.name,
        priority: event_type.priority,
        reporting_criterion: event_type.reporting_criterion,
        url: value_if_non_empty(event_type.url),
        value_types: Vec::new(),
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

  fn add_missing_enum_replace_value_translations(
    event_value_type: &mut EventValueType,
    translations: &mut BTreeMap<String, String>,
    reverse_translations: &BTreeMap<String, &String>,
  ) {
    if event_value_type.enum_replace_value.is_empty() {
      return;
    }

    let translation_id =
      event_value_type.enum_replace_value.strip_prefix("@@").unwrap_or(&event_value_type.enum_replace_value);

    if translations.contains_key(translation_id) {
      return;
    }

    if !event_value_type.description.is_empty() {
      if event_value_type.enum_replace_value.starts_with("ecnStatusEventType~") {
        let description =
          event_value_type.description.strip_prefix("ecnStatusEventType~").unwrap_or(&event_value_type.description);
        translations.insert(translation_id.to_owned(), description.to_owned());
        return;
      }

      let enum_text =
        raw::clean_enum_text(Some(&event_value_type.enum_replace_value), None, event_value_type.description.clone());

      if let Some(reverse_translation_id) = raw::parse_description(&enum_text, reverse_translations) {
        event_value_type.enum_replace_value = reverse_translation_id;
        return;
      }
    }

    translations.insert(translation_id.to_owned(), event_value_type.enum_replace_value.clone());
  }

  fn translation_fixes(id: &str) -> String {
    match id {
      "viessmann-ess.eventvaluetype.AnwahlDrsosselklappe~0" => {
        "viessmann-ess.eventvaluetype.AnwahlDrosselklappe~0".to_owned()
      },
      "viessmann-ess.eventvaluetype.AnwahlDrsosselklappe~1" => {
        "viessmann-ess.eventvaluetype.AnwahlDrosselklappe~1".to_owned()
      },
      "viessmann.eventvaluetype.name.WPR3_SGReady_Funktionen~0" => {
        "viessmann.eventvaluetype.WPR3_SGReady_Funktionen~0".to_owned()
      },
      "viessmann.eventvaluetype.name.WPR3_SGReady_Funktionen~1" => {
        "viessmann.eventvaluetype.WPR3_SGReady_Funktionen~1".to_owned()
      },
      "viessmann.eventvaluetype.name.WPR3_SGReady_Funktionen~2" => {
        "viessmann.eventvaluetype.WPR3_SGReady_Funktionen~2".to_owned()
      },
      "viessmann.eventvaluetype.name.WPR3_SGReady_Funktionen~3" => {
        "viessmann.eventvaluetype.WPR3_SGReady_Funktionen~3".to_owned()
      },
      "viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateoper_shortLWT~0" => {
        "viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateopen_shortLWT~0".to_owned()
      },
      "viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateoper_shortLWT~2" => {
        "viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateopen_shortLWT~2".to_owned()
      },
      "viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateoper_shortLWT~3" => {
        "viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateopen_shortLWT~3".to_owned()
      },
      "viessmann.eventvaluetype.K44_Flagtoindicateopen.shortICT~0" => {
        "viessmann.eventvaluetype.K44_Flagtoindicateopen_shortICT~0".to_owned()
      },
      "viessmann.eventvaluetype.K44_Flagtoindicateopen.shortICT~2" => {
        "viessmann.eventvaluetype.K44_Flagtoindicateopen_shortICT~2".to_owned()
      },
      "viessmann.eventvaluetype.K44_Flagtoindicateopen.shortICT~3" => {
        "viessmann.eventvaluetype.K44_Flagtoindicateopen_shortICT~3".to_owned()
      },
      "viessmann.eventvaluetype.K45_Flagtoindicateoper/shortLWT~2" => {
        "viessmann.eventvaluetype.K45_Flagtoindicateopen_shortICT~2".to_owned()
      },
      "viessmann.eventvaluetype.K45_Flagtoindicateoper/shortLWT~3" => {
        "viessmann.eventvaluetype.K45_Flagtoindicateopen_shortICT~3".to_owned()
      },
      _ => id.to_owned(),
    }
  }
  let mut translations_cleaned: BTreeMap<String, String> =
    translations_raw.iter().map(|(k, v)| (translation_fixes(k), v.get("en").cloned().unwrap())).collect();

  for (ref mut event_value_type_id, event_value_type) in &mut event_value_types {
    add_missing_enum_replace_value_translations(event_value_type, &mut translations_cleaned, &reverse_translations_raw)
  }

  let mut f = File::create("translations.cleaned.json")?;
  serde_json::to_writer_pretty(&mut f, &translations_cleaned)?;
  writeln!(f)?;

  let mut event_type_event_value_type_links = BTreeMap::<u16, Vec<u16>>::new();
  for event_type_event_value_type_link in ecn_data_set.ecn_event_type_event_value_type_link {
    let event_type = event_types.get_mut(&event_type_event_value_type_link.event_type_id).unwrap();
    event_type.value_types.push(event_type_event_value_type_link.event_value_id);
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

  #[derive(Debug, Serialize)]
  struct DataPointDefinitions {
    versions: BTreeMap<String, String>,
    datapoints: BTreeMap<u16, DataPointType>,
    event_types: BTreeMap<u16, EventType>,
    event_value_types: BTreeMap<u16, EventValueType>,
    table_extension_values: BTreeMap<u32, TableExtensionValue>,
    table_extensions: BTreeMap<u16, TableExtension>,
  }
  let data_point_definitions = DataPointDefinitions {
    versions: versions,
    datapoints: data_point_types,
    event_types: event_types,
    event_value_types: event_value_types,
    table_extension_values: table_extension_values,
    table_extensions: table_extensions,
  };
  let mut f = File::create("datapoint_definitions.raw.json")?;
  serde_json::to_writer_pretty(&mut f, &data_point_definitions)?;
  writeln!(f)?;

  let f = File::open("src/sysDeviceIdent.xml")?;
  let io = BufReader::new(f);
  let _system_device_identifier_event_types = raw::event_type::EventTypes::from_reader(io)?;

  let f = File::open("src/sysDeviceIdentExt.xml")?;
  let io = BufReader::new(f);
  let _system_device_identifier_event_types_ext = raw::event_type::EventTypes::from_reader(io)?;

  let f = File::open("src/sysEventType.xml")?;
  let io = BufReader::new(f);
  let system_event_types_raw = raw::event_type::EventTypes::from_reader(io)?;

  #[derive(Debug, Serialize)]
  struct SysEventType {
    pub access_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    pub address: u16,
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
    pub conversion_factor: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_offset: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fc_read: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fc_write: Option<&'static str>,
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

  let mut system_event_types = BTreeMap::<String, SysEventType>::new();
  for event_type in system_event_types_raw.event_type {
    system_event_types.insert(
      raw::strip_address(&event_type.id).into_owned(),
      SysEventType {
        access_mode: event_type.access_mode.to_case(Case::Snake),
        active: event_type.active,
        address: (*event_type.address),
        bit_length: event_type.bit_length,
        bit_position: event_type.bit_position,
        block_factor: event_type.block_factor,
        block_length: event_type.block_length,
        byte_length: event_type.byte_length,
        byte_position: event_type.byte_position,
        conversion: raw::parse_conversion(&event_type.conversion),
        conversion_factor: event_type.conversion_factor,
        conversion_offset: event_type.conversion_offset,
        default_value: event_type.alz,
        description: raw::parse_description(&event_type.description, &reverse_translations_raw),
        fc_read: raw::parse_function(&event_type.fc_read),
        fc_write: raw::parse_function(&event_type.fc_write),
        lower_border: event_type.lower_border,
        mapping_type: event_type.mapping_type,
        name: event_type.name,
        parameter: event_type.parameter,
        priority: event_type.priority,
        sdk_data_type: event_type.sdk_data_type,
        stepping: event_type.stepping,
        option_list: raw::parse_option_list(&event_type.option_list),
        upper_border: event_type.upper_border,
        value_list: raw::parse_value_list(&event_type.value_list)
          .into_iter()
          .map(|(k, v)| (k, raw::parse_description(&v, &reverse_translations_raw).unwrap()))
          .collect(),
      },
    );
  }

  let mut f = File::create("system_event_types.raw.json")?;
  serde_json::to_writer_pretty(&mut f, &system_event_types)?;
  writeln!(f)?;

  return Ok(());

  let input = env::args().nth(1).unwrap();
  dbg!(&input);

  let input = BASE64_STANDARD.decode(input)?;
  let message = nrbf::RemotingMessage::parse(&input).unwrap();

  dbg!(message);

  Ok(())
}
