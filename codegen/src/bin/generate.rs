use std::{
  collections::{BTreeMap, BTreeSet},
  fmt::Debug,
  fs::{self, File},
  io::{BufReader, BufWriter, Write},
  path::Path,
};

use encoding_rs_io::DecodeReaderBytes;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Serialize, de::DeserializeOwned};
use stringcase::snake_case;

use codegen::{
  cleaned::{self, Conversion, ConversionInner},
  command::Command,
  files, raw,
  unique_mapping::UniqueMapping,
};

fn translation_fixes(id: &str) -> String {
  match id {
    "viessmann-ess.eventvaluetype.AnwahlDrsosselklappe~0" => "viessmann-ess.eventvaluetype.AnwahlDrosselklappe~0",
    "viessmann-ess.eventvaluetype.AnwahlDrsosselklappe~1" => "viessmann-ess.eventvaluetype.AnwahlDrosselklappe~1",
    "viessmann.eventvaluetype.name.WPR3_SGReady_Funktionen~0" => "viessmann.eventvaluetype.WPR3_SGReady_Funktionen~0",
    "viessmann.eventvaluetype.name.WPR3_SGReady_Funktionen~1" => "viessmann.eventvaluetype.WPR3_SGReady_Funktionen~1",
    "viessmann.eventvaluetype.name.WPR3_SGReady_Funktionen~2" => "viessmann.eventvaluetype.WPR3_SGReady_Funktionen~2",
    "viessmann.eventvaluetype.name.WPR3_SGReady_Funktionen~3" => "viessmann.eventvaluetype.WPR3_SGReady_Funktionen~3",
    "viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateoper_shortLWT~0" => {
      "viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateopen_shortLWT~0"
    },
    "viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateoper_shortLWT~2" => {
      "viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateopen_shortLWT~2"
    },
    "viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateoper_shortLWT~3" => {
      "viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateopen_shortLWT~3"
    },
    "viessmann.eventvaluetype.K44_Flagtoindicateopen.shortICT~0" => {
      "viessmann.eventvaluetype.K44_Flagtoindicateopen_shortICT~0"
    },
    "viessmann.eventvaluetype.K44_Flagtoindicateopen.shortICT~2" => {
      "viessmann.eventvaluetype.K44_Flagtoindicateopen_shortICT~2"
    },
    "viessmann.eventvaluetype.K44_Flagtoindicateopen.shortICT~3" => {
      "viessmann.eventvaluetype.K44_Flagtoindicateopen_shortICT~3"
    },
    "viessmann.eventvaluetype.K45_Flagtoindicateoper/shortLWT~2" => {
      "viessmann.eventvaluetype.K45_Flagtoindicateopen_shortICT~2"
    },
    "viessmann.eventvaluetype.K45_Flagtoindicateoper/shortLWT~3" => {
      "viessmann.eventvaluetype.K45_Flagtoindicateopen_shortICT~3"
    },
    _ => id,
  }
  .to_owned()
}

static EMPTY_VALUE_TRANSLATION: &str = "viessmann-ess.eventvaluetype.ModulBetriebsart~3";

fn value_list_fixes(id: &str) -> &str {
  match id {
    "viessmann.eventvaluetype.name.HO2B_Geraetetyp~8" => "viessmann.eventvaluetype.HO2B_Geraetetyp~8",
    "viessmann.eventvaluetype..SC100_SoftwareIndex~14" => "viessmann.eventvaluetype.SC100_SoftwareIndex~14",
    "viessmann.eventvaluetype.name.SR13_FktDrehzahlPumpe~3" => "viessmann.eventvaluetype.SR13_FktDrehzahlPumpe~3",
    "viessmann.eventvaluetype.Vitotwin_Fuehlereingang~15" => "viessmann.eventvaluetype.Vitotwin_Fuehlereingang~3", // Translation does not exist.
    "viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateoper_shortLWT~0" => {
      "viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateopen_shortLWT~0"
    },
    "viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateoper/shortLWT~3" => {
      "viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateopen_shortLWT~3"
    },
    "viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateoper/shortLWT~2" => {
      "viessmann.eventvaluetype.WPR3_Split.K43_Flagtoindicateopen_shortLWT~2"
    },
    "viessmann.eventvaluetype.K44_Flagtoindicateopen.shortICT~0" => {
      "viessmann.eventvaluetype.K44_Flagtoindicateopen_shortICT~0"
    },
    "viessmann.eventvaluetype.K45_Flagtoindicateopen_shortIRT~2" => {
      "viessmann.eventvaluetype.K45_Flagtoindicateopen_shortICT~2"
    },
    "viessmann.eventvaluetype.K45_Flagtoindicateopen_shortIRT~3" => {
      "viessmann.eventvaluetype.K45_Flagtoindicateopen_shortICT~3"
    },
    "viessmann.eventvaluetype.name.K4F_Protectionreason_0~4" => "viessmann.eventvaluetype.K4F_Protectionreason_0~4",
    "viessmann.eventvaluetype.WPR3_Split.KC0_Main_mode_variant~5" => {
      "viessmann.eventvaluetype.WPR3_Split.KC0_Main_mode_variant~4"
    }, // Translation does not exist.
    "viessmann.eventvaluetype.WPR3_Split.KC4_Main_mode_variant_diagnostics~2" => {
      "viessmann.eventvaluetype.WPR3_Split.KC0_Main_mode_variant~2"
    }, // Translation does not exist.
    "viessmann.eventvaluetype.WPR3_Split.KC4_Main_modevariant_diagnostics~0" => {
      "viessmann.eventvaluetype.WPR3_Split.KC0_Main_mode_variant~0"
    }, // Translation does not exist.
    "viessmann.eventvaluetype.WPR3_Split.KC4_Main_mode_variant_diagnostics~16" => {
      "viessmann.eventvaluetype.WPR3_Split.KC0_Main_mode_variant~3"
    }, // Translation does not exist.
    "viessmann.eventvaluetype.WPR3_Split.KC4_Main_mode_variant_diagnostics~1" => {
      "viessmann.eventvaluetype.WPR3_Split.KC0_Main_mode_variant~1"
    }, // Translation does not exist.
    "viessmann.eventvaluetype.name.WPR3_Split.KC5_Protection_code~3" => {
      "viessmann.eventvaluetype.WPR3_Split.KC5_Protection_code~3"
    },
    "viessmann.eventvaluetype.WPR3_Split.KE1_IFeel_mode_status~1" => {
      "viessmann.eventvaluetype.WPR3_Split.KE1_IFeel_mode_status~2"
    }, // Translation does not exist.
    "viessmann.eventvaluetype.WPR3_Split.KE4_Reset_NLOAD~1" => {
      "viessmann.eventvaluetype.name.WPR3_Split.KE4_Reset_NLOAD~1"
    },
    "viessmann.eventvaluetype.name.WPR3_Split.KEF_On_Off_Status~1" => {
      "viessmann.eventvaluetype.WPR3_Split.KEF_On_Off_Status~1"
    },
    "viessmann.eventvaluetype.WPR3_Split.KF1_self_test_jumper_1~1" => {
      "viessmann.eventtype.name.WPR3_Split.KF1_self_test_jumper_1~1"
    },
    "viessmann.eventvaluetype.WPR3_Split.KF10_jumper_10~0" => "viessmann.eventvaluetype.WPR3_Split.KF2_jumper_10~0", // Translation does not exist.
    "viessmann.eventvaluetype.WPR3_Split.KF10_jumper_10~1" => "viessmann.eventvaluetype.WPR3_Split.KF2_jumper_10~1", // Translation does not exist.
    "viessmann-ess.eventvaluetype.nciNetConfig~0" => "viessmann.eventvaluetype.nciNetConfig~0",
    "viessmann-ess.eventvaluetype.nciNetConfig~1" => "viessmann.eventvaluetype.nciNetConfig~1",
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~0" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~1" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~2" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~4" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~5" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~6" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~7" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~8" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~9" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~10" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~11" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~12" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~13" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~14" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~15" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-1" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-2" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-3" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-4" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-5" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-6" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-10" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-11" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-12" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_alarm_type~-13" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~-1" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~0" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~1" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~2" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~3" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~4" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~5" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~8" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~7" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~9" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~10" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    "viessmann-ess.eventvaluetype.SNVTAlarm_priority_level~11" => EMPTY_VALUE_TRANSLATION, // Translation does not exist.
    id => id,
  }
}

fn map_conversion(
  conversion: String,
  conversion_factor: Option<f64>,
  conversion_offset: Option<f64>,
) -> Option<Conversion> {
  let conversion = raw::parse_conversion(&conversion);
  let conversion_factor = conversion_factor.filter(|&factor| factor != 0.0);
  let conversion_offset = conversion_offset.filter(|&offset| offset != 0.0);

  if conversion_factor.is_some() || conversion_offset.is_some() {
    Some(Conversion::M(
      [(
        conversion.unwrap_or_else(|| "mul_offset".to_owned()),
        ConversionInner { factor: conversion_factor, offset: conversion_offset },
      )]
      .into_iter()
      .collect(),
    ))
  } else {
    conversion.map(Conversion::S)
  }
}

fn clean_event_type_name(name: &str) -> &str {
  let name = match name {
    "@@WW_Temperatur_Mitte_ab_Bit_0" => "WW_Temperatur_Mitte_ab_Bit_0",
    "@@WW_Temperatur_Mitte_ab_Bit_4" => "WW_Temperatur_Mitte_ab_Bit_4",
    "@@viessmann.eventvaluetype.name.WPR3_Split.KC0_Main_mode_variant" => {
      "@@viessmann.eventtype.name.WPR3_Split.KC0_Main_mode_variant"
    },
    name => name,
  };

  let name = name.strip_prefix("@@viessmann.eventtype.name.viessmann.eventtype.name.").unwrap_or(name);
  let name = name.strip_prefix("@@viessmann.eventtype.name.").unwrap_or(name);
  let name = name.strip_prefix("@@viessmann-ess.eventtype.name.viessmann.eventtype.name.").unwrap_or(name);
  let name = name.strip_prefix("@@viessmann-ess.eventtype.name.").unwrap_or(name);
  name
}

fn is_data_point_type_supported(data_point_type_address: &str, data_point_type: &cleaned::DataPointType) -> bool {
  // Remove devices without identification number.
  if data_point_type.identification.is_none() {
    return false;
  }

  // Remove unsupported devices.
  if data_point_type_address.starts_with("@@BatteryEnergyStorageSystem.") {
    return false;
  }
  if data_point_type_address.starts_with("BESS") {
    return false;
  }
  if data_point_type_address.starts_with("DEKATEL") {
    return false;
  }
  if data_point_type_address.starts_with("OpenTherm") {
    return false;
  }
  if data_point_type_address.starts_with("Vitocom") {
    return false;
  }
  if data_point_type_address.starts_with("Vitogate") {
    return false;
  }
  if data_point_type_address.starts_with("Vitowin") {
    return false;
  }

  true
}

fn is_event_type_supported(event_type_id: &str, event_type: &cleaned::EventType) -> bool {
  if event_type_id.starts_with("Node_")
    || event_type_id.starts_with("nciNet")
    || event_type_id.starts_with("Ecotronic_LAN")
    || event_type_id.starts_with("HO2B_IP")
    || event_type_id.starts_with("HO2B_DynamicIP")
    || event_type_id.starts_with("HO2B_LAN")
    || event_type_id.starts_with("HO2B_Proxy")
    || event_type_id.starts_with("ecnsysEventType~VCOMLan")
    || event_type_id.starts_with("ecnsysLON")
    || event_type_id.starts_with("ecnsysVitocom")
    || event_type_id.starts_with("vcLan")
    || event_type_id.starts_with("vcNotfax")
    || event_type_id.starts_with("vlogVSNotfax")
  {
    return false;
  }

  if event_type.address.is_none() {
    return false;
  }

  event_type.fc_read.as_deref().is_some_and(|fc| fc == "virtual_read")
    || event_type.fc_write.as_deref().is_some_and(|fc| fc == "virtual_write")
}

fn translations_raw() -> anyhow::Result<BTreeMap<String, BTreeMap<String, String>>> {
  let mut cultures = BTreeMap::new();
  let mut translations_raw = BTreeMap::new();

  for text_resource in files::TEXT_RESOURCES {
    let document: raw::DocumentElement = load_xml(text_resource)?;

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
  save_json("translations.raw.json", &translations_raw)?;
  Ok(translations_raw)
}

fn reverse_translations_raw(
  translations_raw: &BTreeMap<String, BTreeMap<String, String>>,
) -> anyhow::Result<BTreeMap<String, &String>> {
  let reverse_translations_raw = translations_raw
    .iter()
    .filter_map(|(k, v)| {
      let text = cleaned::simplify_translation_text(v.get("de").unwrap());

      if text.is_empty() {
        return None;
      }

      Some((text, k))
    })
    .collect();
  save_json("reverse_translations.raw.json", &reverse_translations_raw)?;
  Ok(reverse_translations_raw)
}

fn add_missing_enum_replace_value_translations(
  event_value_type: &mut raw::EventValueType,
  translations: &mut BTreeMap<String, String>,
  reverse_translations: &BTreeMap<String, &String>,
) {
  if event_value_type.enum_replace_value.is_empty() {
    return;
  }

  let enum_replace_value = event_value_type.enum_replace_value.clone();

  let translation_id = enum_replace_value.strip_prefix("@@").unwrap_or(&enum_replace_value);
  if translations.contains_key(translation_id) {
    return;
  }

  if !event_value_type.description.is_empty() {
    if enum_replace_value.starts_with("ecnStatusEventType~") {
      let description =
        event_value_type.description.strip_prefix("ecnStatusEventType~").unwrap_or(&event_value_type.description);
      translations.insert(translation_id.to_owned(), description.to_owned());
      return;
    }

    let enum_text = raw::clean_enum_text(Some(&enum_replace_value), None, event_value_type.description.clone());

    if let Some(reverse_translation_id) = cleaned::parse_description(&enum_text, &translations, reverse_translations) {
      event_value_type.enum_replace_value = reverse_translation_id;
      return;
    }
  }

  translations.insert(translation_id.to_owned(), enum_replace_value);
}

fn load_xml<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> anyhow::Result<T> {
  let f = File::open(Path::new(env!("CARGO_MANIFEST_DIR")).join(path))?;
  let decoder = DecodeReaderBytes::new(f);
  let io = BufReader::new(decoder);
  Ok(quick_xml::de::from_reader(io)?)
}

fn save_json<P: AsRef<Path>, V: Serialize>(path: P, data: V) -> anyhow::Result<()> {
  let build_dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../build"));
  fs::create_dir_all(build_dir)?;
  let mut f = BufWriter::new(File::create(build_dir.join(path))?);
  serde_json::to_writer_pretty(&mut f, &data)?;
  writeln!(f)?;
  Ok(())
}

fn main() -> anyhow::Result<()> {
  let translations_raw = translations_raw()?;
  let reverse_translations_raw = reverse_translations_raw(&translations_raw)?;

  let import_export_data_holder: raw::ImportExportDataHolder = load_xml(files::DP_DEFINITIONS)?;

  let ecn_data_set = import_export_data_holder.ecn_data_set.diff_gram.ecn_data_set;

  let versions: BTreeMap<_, _> =
    ecn_data_set.ecn_version.into_iter().map(|version| (snake_case(&version.name), version.value)).collect();
  save_json(format!("versions.used.json"), &versions)?;

  let data_point_types_raw: BTreeMap<_, _> = ecn_data_set
    .ecn_datapoint_type
    .into_iter()
    .map(|data_point_type| {
      (
        data_point_type.id,
        raw::DataPointType {
          address: data_point_type.address,
          name: data_point_type.name,
          status_event_type_id: data_point_type.status_event_type_id,
        },
      )
    })
    .collect();

  let mut data_point_type_event_types = ecn_data_set.ecn_data_point_type_event_type_link.into_iter().fold(
    BTreeMap::new(),
    |mut acc, data_point_type_event_type_link| {
      acc
        .entry(data_point_type_event_type_link.data_point_type_id)
        .or_insert_with(BTreeSet::new)
        .insert(data_point_type_event_type_link.event_type_id);
      acc
    },
  );

  let event_types_raw: BTreeMap<_, _> =
    ecn_data_set.ecn_event_type.into_iter().map(|event_type| (event_type.id, event_type)).collect();

  let mut event_value_types_raw = BTreeMap::<u16, raw::EventValueType>::new();
  for event_value_type in ecn_data_set.ecn_event_value_type {
    let id = event_value_type.id;
    event_value_types_raw.insert(
      id,
      raw::EventValueType {
        data_type: event_value_type.data_type,
        description: event_value_type.description,
        enum_address_value: event_value_type.enum_address_value,
        enum_replace_value: event_value_type.enum_replace_value,
        length: event_value_type.length,
        lower_border: event_value_type.lower_border,
        name: event_value_type.name,
        status_type_id: event_value_type.status_type_id,
        stepping: event_value_type.stepping,
        unit: event_value_type.unit,
        upper_border: event_value_type.upper_border,
        value_precision: event_value_type.value_precision,
      },
    );
  }

  let mut translations: BTreeMap<_, _> =
    translations_raw.iter().map(|(k, v)| (k.to_owned(), v.get("de").unwrap().to_owned())).collect();
  for event_value_type in event_value_types_raw.values_mut() {
    add_missing_enum_replace_value_translations(event_value_type, &mut translations, &reverse_translations_raw)
  }

  let mut event_type_value_types = ecn_data_set.ecn_event_type_event_value_type_link.into_iter().fold(
    BTreeMap::new(),
    |mut acc, event_type_event_value_type_link| {
      let event_types = acc.entry(event_type_event_value_type_link.event_type_id).or_insert_with(BTreeSet::new);
      event_types.insert(event_type_event_value_type_link.event_value_id);
      acc
    },
  );

  let table_extensions_raw: BTreeMap<_, _> = ecn_data_set
    .ecn_table_extension
    .into_iter()
    .map(|table_extension| {
      (
        table_extension.id,
        raw::TableExtension {
          field_name: snake_case(
            table_extension
              .field_name
              .strip_prefix(&format!("label.tableextension.{}.", table_extension.table_name))
              .map(|s| s)
              .unwrap_or(&table_extension.field_name),
          ),
          internal_data_type: table_extension.internal_data_type,
          internal_default_value: match nrbf::from_slice(&table_extension.internal_default_value) {
            Ok(serde_json::Value::Number(n)) => Some(serde_json::Value::Number(n)),
            // For enums, this is the index for `options_value`, so convert strings to integers.
            Ok(serde_json::Value::String(s)) if table_extension.internal_data_type == 6 => match s.as_str() {
              "" => None,
              s => Some(serde_json::Value::Number(s.parse::<u8>().unwrap().into())),
            },
            Ok(serde_json::Value::String(s)) => match s.as_str() {
              _ => Some(serde_json::Value::String(s)),
            },
            v => unreachable!("unhandled default value: {v:?}"),
          },
          options_value: table_extension
            .options_value
            .into_iter()
            .map(|s| {
              let (k, v) = s.split_once("=").unwrap();
              (v.to_owned(), k.to_owned())
            })
            .collect(),
          pk_fields: table_extension.pk_fields.iter().map(|field| snake_case(&field)).collect(),
          table_name: table_extension.table_name,
        },
      )
    })
    .collect();

  let table_extension_values_raw: BTreeMap<_, _> = ecn_data_set
    .ecn_table_extension_value
    .into_iter()
    .map(|table_extension_value| {
      (
        table_extension_value.id,
        raw::TableExtensionValue {
          internal_value: match nrbf::from_slice(&table_extension_value.internal_value) {
            Ok(v) => Some(v),
            v => unreachable!("unhandled default value: {v:?}"),
          },
          pk_value: table_extension_value.pk_value.into_iter().map(|s| s.parse().unwrap()).collect(),
          ref_id: table_extension_value.ref_id,
        },
      )
    })
    .collect();

  let mut translations_cleaned =
    translations_raw.iter().map(|(k, v)| (translation_fixes(k), v.get("en").cloned().unwrap())).collect();
  for event_value_type in event_value_types_raw.values_mut() {
    add_missing_enum_replace_value_translations(event_value_type, &mut translations_cleaned, &reverse_translations_raw)
  }
  save_json("translations.cleaned.json", &translations_cleaned)?;

  let event_value_types_cleaned: BTreeMap<_, _> = event_value_types_raw
    .into_iter()
    .filter_map(|(event_value_type_id, mut event_value_type)| {
      let unit = cleaned::parse_unit(&event_value_type.unit);

      let data_type = match event_value_type.data_type.as_str() {
        "DateTime" => cleaned::EventValueType::Single {
          value_type: "DateTime",
          lower_border: None,
          upper_border: None,
          stepping: None,
          unit: None,
        },
        "Binary" => {
          let name = event_value_type.name.as_str();
          let value_type = if name == "ecnsysEventType~ErrorIndex" {
            "ErrorIndex"
          } else if name == "ecnsysEventType~Error" || name.starts_with("@@viessmann.eventvaluetype.name.FehlerHisFA") {
            "Error"
          } else if name == "Mapping~Schaltzeiten" {
            "CircuitTimes"
          } else {
            "ByteArray"
          };
          cleaned::EventValueType::Single {
            value_type,
            lower_border: None,
            upper_border: None,
            stepping: None,
            unit: None,
          }
        },
        // Replace temperature enum mapping used.
        "VarChar" if event_value_type.name.starts_with("@@viessmann.eventvaluetype.name.Ecotronic_Party~") => {
          let value = event_value_type
            .name
            .strip_prefix("@@viessmann.eventvaluetype.name.Ecotronic_Party~")
            .unwrap()
            .parse::<u8>()
            .unwrap();

          cleaned::EventValueType::Single {
            value_type: "Int",
            lower_border: Some(value.into()),
            upper_border: Some(value.into()),
            stepping: Some(1.0),
            unit: Some("°C".into()),
          }
        },
        "VarChar" | "NText" => {
          if let Some(address_value) = event_value_type.enum_address_value {
            let enum_replace_value =
              event_value_type.enum_replace_value.strip_prefix("@@").unwrap_or(&event_value_type.enum_replace_value);
            let enum_replace_value = translation_fixes(enum_replace_value);
            let value_list_entry = value_list_fixes(&enum_replace_value);

            let mut value_list = BTreeMap::new();
            value_list.insert(address_value, value_list_entry.to_owned());
            cleaned::EventValueType::Multiple { value_list }
          } else {
            cleaned::EventValueType::Single {
              value_type: "String",
              lower_border: None,
              upper_border: None,
              stepping: None,
              unit: None,
            }
          }
        },
        data_type @ ("Int" | "Float" | "Bit") => {
          let data_type = match data_type {
            "Int" => "Int",
            "Float" => "Double",
            "Bit" => "Bit",
            _ => unreachable!(),
          };
          cleaned::EventValueType::Single {
            value_type: data_type,
            lower_border: event_value_type.lower_border.take(),
            upper_border: event_value_type.upper_border.take(),
            stepping: event_value_type.stepping.take(),
            unit: unit.map(|unit| unit.to_owned()),
          }
        },
        _ => unreachable!(),
      };

      Some((event_value_type_id, data_type))
    })
    .collect();

  let _system_device_identifier_event_types_raw: raw::EventTypes = load_xml(files::SYS_DEVICE_IDENT)?;
  let _system_device_identifier_event_types_ext_raw: raw::EventTypes = load_xml(files::SYS_DEVICE_IDENT_EXT)?;
  let system_event_types_raw: raw::EventTypes = load_xml(files::SYS_EVENT_TYPE)?;

  let system_event_types: BTreeMap<_, _> = system_event_types_raw
    .event_type
    .into_iter()
    .map(|event_type| {
      (
        raw::strip_address(&event_type.id, &event_type.address).to_owned(),
        raw::SysEventType {
          access_mode: snake_case(&event_type.access_mode),
          address: event_type.address.clone(),
          bit_length: event_type.bit_length,
          bit_position: event_type.bit_position,
          block_factor: event_type.block_factor,
          block_length: event_type.block_length,
          byte_length: event_type.byte_length,
          byte_position: event_type.byte_position,
          conversion: event_type.conversion,
          conversion_factor: event_type.conversion_factor,
          conversion_offset: event_type.conversion_offset,
          default_value: event_type.alz,
          description: cleaned::parse_description(&event_type.description, &translations, &reverse_translations_raw),
          fc_read: cleaned::parse_function(event_type.fc_read),
          fc_write: cleaned::parse_function(event_type.fc_write),
          lower_border: event_type.lower_border,
          mapping_type: event_type.mapping_type,
          name: event_type.name,
          parameter: event_type.parameter,
          priority: event_type.priority,
          sdk_data_type: event_type.sdk_data_type,
          stepping: event_type.stepping,
          option_list: {
            if let Some(option_list) = event_type.option_list {
              raw::parse_option_list(&option_list)
            } else {
              Vec::new()
            }
          },
          upper_border: event_type.upper_border,
          value_list: {
            if let Some(value_list) = event_type.value_list {
              raw::parse_value_list(&value_list)
                .into_iter()
                .map(|(k, v)| (k, cleaned::parse_description(&v, &translations, &reverse_translations_raw).unwrap()))
                .collect()
            } else {
              BTreeMap::new()
            }
          },
        },
      )
    })
    .collect();
  save_json("system_event_types.raw.json", &system_event_types)?;

  let mut data_point_types_cleaned: BTreeMap<_, _> = data_point_types_raw
    .into_iter()
    .map(|(id, data_point_type)| {
      (
        id,
        cleaned::DataPointType {
          address: Some(data_point_type.address),
          event_types: data_point_type_event_types.remove(&id).unwrap(),
          f0: None,
          f0_till: None,
          identification: None,
          identification_extension: None,
          identification_extension_till: None,
          name: data_point_type.name.clone(),
          options: Vec::new(),
          status_event_type_id: data_point_type.status_event_type_id,
        },
      )
    })
    .collect();

  let mut event_types_cleaned: BTreeMap<_, _> = event_types_raw
    .iter()
    .map(|(id, event_type)| {
      let event_type_id = clean_event_type_name(&event_type.name).to_owned();
      let value_types = event_type_value_types.remove(id).unwrap_or(BTreeSet::new());
      let value_type = value_types.into_iter().fold(None, |acc, value_type_id| {
        let value_type = event_value_types_cleaned.get(&value_type_id).cloned().unwrap();
        match (acc, value_type) {
          (None, value_type) => Some(value_type),
          (
            Some(cleaned::EventValueType::Multiple { mut value_list }),
            cleaned::EventValueType::Multiple { value_list: other_value_list },
          ) => {
            value_list.extend(other_value_list);
            Some(cleaned::EventValueType::Multiple { value_list })
          },
          (
            Some(cleaned::EventValueType::Single { value_type, lower_border, upper_border, stepping, ref unit }),
            cleaned::EventValueType::Single {
              value_type: other_value_type,
              lower_border: other_lower_border,
              upper_border: other_upper_border,
              stepping: other_stepping,
              unit: ref other_unit,
            },
          ) if value_type == other_value_type && stepping == other_stepping && unit == other_unit => {
            // Treat multiple identical value types as single type.
            Some(cleaned::EventValueType::Single {
              value_type,
              lower_border: lower_border.zip(other_lower_border).map(|(a, b)| a.min(b)),
              upper_border: upper_border.zip(other_upper_border).map(|(a, b)| a.max(b)),
              stepping,
              unit: unit.clone(),
            })
          },
          (Some(acc), other) => {
            unreachable!("invalid single value type: {acc:?} != {other:?}")
          },
        }
      });

      let mut event_type_cleaned = cleaned::EventType {
        access_mode: match event_type.access_mode {
          1 => "read",
          2 => "write",
          3 => "read_write",
          t => unreachable!("unknown access mode: {t}"),
        }
        .to_owned(),
        address: Some(event_type.address.clone()),
        bit_length: None,
        bit_position: None,
        block_factor: None,
        block_length: None,
        byte_length: None,
        byte_position: None,
        conversion: Some(Conversion::S(event_type.conversion.clone())),
        conversion_factor: None,
        conversion_offset: None,
        default_value: cleaned::parse_default_value(&event_type.default_value),
        description: cleaned::parse_description(&event_type.description, &translations, &reverse_translations_raw),
        enum_type: Some(event_type.enum_type),
        fc_read: None,
        fc_write: None,
        filter_criterion: Some(event_type.filter_criterion),
        lower_border: None,
        mapping_type: None,
        name: Some(event_type.name.clone()),
        option_list: Vec::new(),
        parameter: None,
        priority: Some(event_type.priority),
        reporting_criterion: Some(event_type.reporting_criterion),
        sdk_data_type: None,
        stepping: None,
        type_id: event_type_id,
        unit: None,
        upper_border: None,
        url: if event_type.url.is_empty() { None } else { Some(event_type.url.clone()) },
        value_list: BTreeMap::new(),
        value_type: None,
      };

      match value_type {
        Some(cleaned::EventValueType::Single { value_type, lower_border, upper_border, stepping, unit }) => {
          event_type_cleaned.value_type = Some(value_type.to_owned());
          event_type_cleaned.lower_border = lower_border;
          event_type_cleaned.upper_border = upper_border;
          event_type_cleaned.stepping = stepping;
          event_type_cleaned.unit = unit;
        },
        Some(cleaned::EventValueType::Multiple { value_list }) => {
          event_type_cleaned.value_list =
            event_type_cleaned.value_list.into_iter().chain(value_list.into_iter()).collect();
        },
        None => (),
      }

      (*id, event_type_cleaned)
    })
    .collect();

  for (_, table_extension_value) in &table_extension_values_raw {
    let table_extension = table_extensions_raw.get(&table_extension_value.ref_id).unwrap();

    let pk: BTreeMap<_, _> = table_extension.pk_fields.iter().zip(table_extension_value.pk_value.iter()).collect();
    let id = pk.get(&"id".to_owned()).unwrap();

    let table_name = &table_extension.table_name;
    let field_name = &table_extension.field_name;
    let value = table_extension_value.internal_value.clone();

    match table_name.as_str() {
      "ecnDatapointType" => {
        let Some(data_point_type_cleaned) = data_point_types_cleaned.get_mut(id) else { continue };

        match (field_name.as_str(), table_extension.internal_data_type, value) {
          ("f0", 3, Some(serde_json::Value::Number(n))) => {
            data_point_type_cleaned.f0 = n.as_i64().map(|n| n as u16);
          },
          ("f0_till", 3, Some(serde_json::Value::Number(n))) => {
            data_point_type_cleaned.f0_till = n.as_i64().map(|n| n as u16);
          },
          ("identification", 4, Some(serde_json::Value::String(s))) => {
            data_point_type_cleaned.identification = if s.is_empty() { None } else { Some(s) };
          },
          ("identification_extension", 4, Some(serde_json::Value::String(s))) => {
            data_point_type_cleaned.identification_extension = if s.is_empty() { None } else { Some(s) };
          },
          ("identification_extension_till", 4, Some(serde_json::Value::String(s))) => {
            data_point_type_cleaned.identification_extension_till = if s.is_empty() { None } else { Some(s) };
          },
          ("options", 7, Some(serde_json::Value::String(s))) => {
            data_point_type_cleaned.options = s.split(";").map(|o| o.to_owned()).collect();
          },
          ("vsko", 0, Some(serde_json::Value::Bool(_)))
          | (
            "code_access_level1"
            | "code_access_level2"
            | "event_optimisation_exception_list"
            | "product_name"
            | "short_name",
            4,
            Some(serde_json::Value::String(_)),
          )
          | ("controller_type" | "error_type" | "event_optimisation", 6, Some(serde_json::Value::Number(_))) => {
            // Unused.
          },
          (field_name, internal_data_type, value) => {
            unreachable!("unhandled table extension: {field_name} {internal_data_type:?} {value:?}");
          },
        };
      },
      "ecnEventType" => {
        let Some(event_type_cleaned) = event_types_cleaned.get_mut(id) else { continue };

        match (field_name.as_str(), table_extension.internal_data_type, value) {
          ("address", 4, Some(serde_json::Value::String(s))) => {
            event_type_cleaned.address = Some(s);
          },
          ("bit_length", 3, Some(serde_json::Value::Number(n))) => {
            let n = n.as_u64().unwrap();
            if n != 0 {
              event_type_cleaned.bit_length = Some(n.try_into().unwrap());
            }
          },
          ("bit_position", 3, Some(serde_json::Value::Number(n))) => {
            event_type_cleaned.bit_position = Some(n.as_u64().unwrap().try_into().unwrap());
          },
          ("block_length", 3, Some(serde_json::Value::Number(n))) => {
            event_type_cleaned.block_length = Some(n.as_u64().unwrap().try_into().unwrap());
          },
          ("block_factor", 3, Some(serde_json::Value::Number(n))) => {
            let n = n.as_u64().unwrap();
            if n != 0 {
              event_type_cleaned.block_factor = Some(n.try_into().unwrap());
            }
          },
          ("byte_length", 3, Some(serde_json::Value::Number(n))) => {
            event_type_cleaned.byte_length = Some(n.as_u64().unwrap().try_into().unwrap());
          },
          ("byte_position", 3, Some(serde_json::Value::Number(n))) => {
            event_type_cleaned.byte_position = Some(n.as_u64().unwrap().try_into().unwrap());
          },
          ("conversion_factor", 2, Some(serde_json::Value::Number(n))) => {
            event_type_cleaned.conversion_factor = Some(n.as_f64().unwrap());
          },
          ("conversion_offset", 2, Some(serde_json::Value::Number(n))) => {
            event_type_cleaned.conversion_offset = Some(n.as_f64().unwrap());
          },
          ("fc_read", 7, Some(serde_json::Value::String(s))) => {
            event_type_cleaned.fc_read = cleaned::parse_function(s);
          },
          ("fc_write", 7, Some(serde_json::Value::String(s))) => {
            event_type_cleaned.fc_write = cleaned::parse_function(s);
          },
          ("option", 4, Some(serde_json::Value::String(ref s))) => {
            event_type_cleaned.option_list = raw::parse_option_list(s);
          },
          ("mapping_type", 6, Some(serde_json::Value::Number(n))) => {
            let options_value = &table_extension.options_value;
            let mapping_type = options_value.get(&n.to_string()).unwrap();
            if mapping_type != "NoMap" {
              event_type_cleaned.mapping_type = Some(mapping_type.to_owned());
            }
          },
          ("parameter", 4, Some(serde_json::Value::String(s))) => {
            event_type_cleaned.parameter = Some(s);
          },
          ("sdk_data_type", 7, Some(serde_json::Value::String(s))) => {
            event_type_cleaned.sdk_data_type = Some(s);
          },
          (
            "ba_cnet_gateway" | "is_vd100" | "knx_gateway" | "mod_bus_gateway" | "lon_snvt_gateway",
            0,
            Some(serde_json::Value::Bool(_)),
          )
          | ("trigger_mode_parameter_n", 2, Some(serde_json::Value::Number(_)))
          | ("trigger_mode_parameter_t" | "dpt_typ" | "dpt_sub", 3, Some(serde_json::Value::Number(_)))
          | (
            "function_value"
            | "prefix_read"
            | "prefix_write"
            | "vitocom_channel_id"
            | "knx_gateway_objekt"
            | "knx_conversion"
            | "lon_snvt_gateway_objekt",
            4,
            Some(serde_json::Value::String(_)),
          )
          | (
            "definition_type" | "rpc_handler" | "trigger_mode" | "qo_s_level",
            6,
            Some(serde_json::Value::Number(_)),
          )
          | ("ba_cnet_gateway_objekt" | "mod_bus_gateway_objekt", 7, Some(serde_json::Value::String(_))) => {
            // Unused.
          },
          (field_name, data_type, value) => {
            unreachable!("unhandled table extension: {field_name} {data_type} {value:?}")
          },
        }
      },
      "ecnEventTypeGroup" => continue,
      table_name => {
        unreachable!("unknown table: {table_name}")
      },
    }
  }

  let event_types_cleaned: BTreeMap<u32, cleaned::EventType> = event_types_cleaned
    .into_iter()
    .filter_map(|(id, mut event_type)| {
      let event_type_id = clean_event_type_name(event_type.name.as_ref()?);

      if !is_event_type_supported(event_type_id, &event_type) {
        return None;
      }

      if let Some(Conversion::S(conversion)) = event_type.conversion.take() {
        event_type.conversion =
          map_conversion(conversion, event_type.conversion_factor.take(), event_type.conversion_offset.take());
      }

      if let Some(block_factor) = event_type.block_factor {
        if matches!(event_type.value_type.as_deref(), Some("CircuitTimes")) {
          if block_factor == 7 {
            event_type.block_factor = None;
          } else if block_factor != 56 {
            panic!("Unsupported block factor {block_factor} for CircuitTimes: {event_type:?}")
          }
        } else {
          let block_length = event_type.block_length.unwrap();
          if (block_length % u16::from(block_factor)) != 0 {
            panic!("Block length {block_length} not divisible by block factor {block_factor}: {event_type:?}")
          }
        }
      }

      Some((id, event_type))
    })
    .collect();

  let data_point_types_cleaned: BTreeMap<_, _> = data_point_types_cleaned
    .into_iter()
    .filter_map(|(_, mut data_point_type_cleaned)| {
      let data_point_type_cleaned_address = data_point_type_cleaned.address.take().unwrap();

      if !is_data_point_type_supported(&data_point_type_cleaned_address, &data_point_type_cleaned) {
        return None;
      }

      data_point_type_cleaned.event_types =
        data_point_type_cleaned.event_types.into_iter().filter(|id| event_types_cleaned.contains_key(id)).collect();

      Some((data_point_type_cleaned_address, data_point_type_cleaned))
    })
    .collect();

  let event_type_ids: BTreeSet<_> = event_types_cleaned.iter().map(|(_, event_type)| &event_type.type_id).collect();

  let system_event_types_cleaned: BTreeMap<_, _> = system_event_types
    .into_iter()
    .filter_map(|(event_type_id, event_type)| {
      if event_type_ids.contains(&event_type_id) {
        return None;
      }

      let value_type = if event_type_id == "ecnsysDeviceIdent" {
        Some("DeviceId".to_owned())
      } else if event_type_id == "ecnsysDeviceIdentF0" {
        Some("DeviceIdF0".to_owned())
      } else if event_type_id == "ecnsysErrorBuffer"
        || event_type_id.starts_with("ecnsysFehlerhistorie")
        || event_type_id == "ecnsysControllerSerialNumber"
        || event_type_id == "ecnsysDeviceBoilerSerialNumber"
      {
        return None;
      } else {
        None
      };

      let event_type_cleaned = cleaned::EventType {
        access_mode: event_type.access_mode,
        address: Some(event_type.address),
        bit_length: if event_type.bit_length == 0 { None } else { Some(event_type.bit_length) },
        bit_position: Some(event_type.bit_position),
        block_factor: event_type.block_factor,
        block_length: Some(event_type.block_length),
        byte_length: Some(event_type.byte_length),
        byte_position: Some(event_type.byte_position),
        conversion: event_type.conversion.and_then(|conversion| {
          map_conversion(conversion, event_type.conversion_factor, event_type.conversion_offset)
        }),
        conversion_factor: None,
        conversion_offset: None,
        default_value: None,
        description: event_type.description.clone(),
        enum_type: None,
        fc_read: event_type.fc_read,
        fc_write: event_type.fc_write,
        filter_criterion: None,
        lower_border: None,
        mapping_type: None,
        name: event_type.name.clone(),
        option_list: Vec::new(),
        parameter: Some(event_type.parameter),
        priority: event_type.priority,
        reporting_criterion: None,
        sdk_data_type: Some(event_type.sdk_data_type),
        stepping: None,
        type_id: event_type_id.clone(),
        unit: None,
        upper_border: None,
        url: None,
        value_list: BTreeMap::new(),
        value_type,
      };

      if !is_event_type_supported(&event_type_id, &event_type_cleaned) {
        return None;
      }

      Some((event_type_id, event_type_cleaned))
    })
    .collect();
  save_json("system_event_types.cleaned.json", &system_event_types_cleaned)?;

  let devices_cleaned: BTreeMap<_, _> = data_point_types_cleaned
    .iter()
    .filter(|(_, data_point_type)| {
      // Remove devices without any supported event types.
      data_point_type
        .event_types
        .iter()
        .filter(|event_type_id| {
          let type_id = event_types_cleaned.get(&event_type_id).as_ref().unwrap().type_id.as_str();
          !matches!(type_id, "ecnsysEventType~Error" | "ecnsysEventType~ErrorIndex")
        })
        .count()
        > 0
    })
    .collect();
  save_json("devices.cleaned.json", &devices_cleaned)?;

  let translations_unique = UniqueMapping::create(translations_cleaned);
  save_json("translations.unique.json", &translations_unique)?;

  let error_mappings =
    translations_unique.mapping.iter().fold(BTreeMap::<String, _>::new(), |mut acc, (translation_id, mapping_id)| {
      lazy_static! {
        static ref ERROR_CODE_REGEX: Regex =
          Regex::new(r"^(?<id>viessmann\.errorcode(?:\.SMS)?(?:\.[^.]+)?)\.(?<value>[[:xdigit:]]{2})$").unwrap();
      }

      if let Some(captures) = ERROR_CODE_REGEX.captures(translation_id) {
        let id = captures.name("id").unwrap().as_str().to_string();
        let value = captures.name("value").unwrap().as_str();

        let h = acc.entry(id).or_insert_with(BTreeMap::new);
        h.insert(i32::from_str_radix(value, 16).unwrap(), mapping_id);
      }

      acc
    });

  let all_event_types = system_event_types_cleaned.values().chain(event_types_cleaned.values());
  let mappings = all_event_types.fold(BTreeMap::<String, BTreeMap<i32, _>>::new(), |mut acc, event_type| {
    let value_list = &event_type.value_list;

    if value_list.is_empty() {
      return acc;
    }

    acc.insert(
      event_type.type_id.clone(),
      value_list
        .iter()
        .map(|(value, translation_id)| (value.clone(), translations_unique.mapping.get(translation_id).unwrap()))
        .collect(),
    );

    acc
  });

  let mappings_unique = UniqueMapping::create(error_mappings.into_iter().chain(mappings.into_iter()).collect());
  save_json("mappings.unique.json", &mappings_unique)?;

  let devices_used: BTreeMap<_, _> = {
    let default_error_mapping = *mappings_unique.mapping.get("viessmann.errorcode").unwrap();

    #[derive(Debug, Serialize)]
    struct DeviceUsed {
      pub commands: BTreeSet<u32>,
      pub error_mapping: usize,
      pub f0: Option<u16>,
      pub f0_till: Option<u16>,
      pub id: u16,
      pub id_ext: Option<u16>,
      pub id_ext_till: Option<u16>,
    }

    devices_cleaned
      .into_iter()
      .map(|(device_id, device)| {
        (
          device_id,
          DeviceUsed {
            commands: device.event_types.clone(),
            error_mapping: mappings_unique
              .mapping
              .get(&format!("viessmann.errorcode.{device_id}"))
              .cloned()
              .unwrap_or(default_error_mapping),
            f0: device.f0,
            f0_till: device.f0_till,
            id: device.identification.as_deref().map(|n| u16::from_str_radix(n, 16).unwrap()).unwrap(),
            id_ext: device.identification_extension.as_deref().map(|n| u16::from_str_radix(n, 16).unwrap()),
            id_ext_till: device.identification_extension_till.as_deref().map(|n| u16::from_str_radix(n, 16).unwrap()),
          },
        )
      })
      .collect()
  };
  save_json("devices.used.json", &devices_used)?;

  let event_types_used = {
    let used_event_type_ids = devices_used.values().fold(BTreeSet::new(), |mut acc, device| {
      acc.extend(device.commands.iter().copied());
      acc
    });

    event_types_cleaned.into_iter().filter(|(event_type_id, _)| used_event_type_ids.contains(&event_type_id)).fold(
      BTreeMap::new(),
      |mut acc, (id, event_type)| {
        let event_type_id = event_type.type_id.clone();
        acc.insert(id, Command::from_event_type(event_type_id, event_type, &mappings_unique));
        acc
      },
    )
  };
  save_json("event_types.used.json", &event_types_used)?;

  let system_event_types_used: BTreeMap<_, _> = system_event_types_cleaned
    .into_iter()
    .map(|(event_type_id, event_type)| {
      let id = event_type_id.as_str();
      let id = id.strip_prefix("ecnsysEventType~").or_else(|| id.strip_prefix("ecnsys")).unwrap_or(id);
      let id = id.replace("LON", "_LON_");
      let id = id.replace("BHKW", "_CHP_"); // Blockheizkraftwerk -> Combined Heat & Power
      let id = snake_case(&id);
      let id = id
        .split("_")
        .map(|part| match part {
          "ident" => "id",
          "fehlerhistorie" => "error_history",
          "anlagennummer" => "system_number",
          "teilnehmernummer" => "subscriber_number",
          "modul" => "module",
          part => part,
        })
        .collect::<Vec<_>>()
        .join("_");

      (id, Command::from_event_type(event_type_id, event_type, &mappings_unique))
    })
    .collect();
  save_json("system_event_types.used.json", &system_event_types_used)?;

  let mappings_used: BTreeMap<_, _> = {
    let device_error_mappings = devices_used.into_values().map(|v| v.error_mapping);
    let command_mappings = event_types_used.into_values().filter_map(|v| v.mapping);

    device_error_mappings.chain(command_mappings).map(|k| (k, mappings_unique.translations.get(&k).unwrap())).collect()
  };
  save_json("mappings.used.json", &mappings_used)?;

  let translations_used: BTreeMap<_, _> = mappings_used
    .into_values()
    .flat_map(|translations| translations.values())
    .map(|translation_id| (translation_id, translations_unique.translations.get(translation_id).unwrap()))
    .collect();
  save_json("translations.used.json", &translations_used)?;

  Ok(())
}
