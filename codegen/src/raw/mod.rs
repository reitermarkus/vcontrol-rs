use std::collections::BTreeMap;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, de::IgnoredAny};
use serde_with::{StringWithSeparator, base64::Base64, formats::SemicolonSeparator, serde_as};
use stringcase::snake_case;

mod data_point_type;
pub use data_point_type::DataPointType;
mod event_type;
pub use event_type::EventTypes;
mod event_value_type;
pub use event_value_type::EventValueType;
mod sys_event_type;
pub use sys_event_type::SysEventType;
mod table_extension;
pub use table_extension::TableExtension;
mod table_extension_value;
pub use table_extension_value::TableExtensionValue;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DefaultCulture {
  #[serde(rename = "@CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "@DefaultCultureId")]
  #[allow(unused)]
  pub default_culture_id: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DefaultCultures {
  #[serde(rename = "DefaultCulture")]
  #[allow(unused)]
  pub default_culture: DefaultCulture,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Culture {
  #[serde(rename = "@Id")]
  pub id: u8,
  #[serde(rename = "@CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "@Name")]
  pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cultures {
  #[serde(default, rename = "Culture")]
  pub culture: Vec<Culture>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextResource {
  #[serde(rename = "@CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "@CultureId")]
  pub culture_id: u8,
  #[serde(rename = "@Label")]
  pub label: String,
  #[serde(rename = "@Value")]
  pub value: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextResources {
  #[serde(rename = "TextResource")]
  pub text_resource: Vec<TextResource>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentElement {
  #[allow(unused)]
  #[serde(rename = "@xmlns")]
  pub xmlns: String,
  #[serde(rename = "DefaultCultures")]
  #[allow(unused)]
  pub default_cultures: DefaultCultures,
  #[serde(rename = "Cultures")]
  pub cultures: Cultures,
  #[serde(rename = "TextResources")]
  pub text_resources: TextResources,
}

lazy_static! {
  static ref WHITESPACE: Regex = Regex::new(r"\s+").unwrap();
}

pub fn parse_translation_text(text: String) -> String {
  let text = text.trim();
  let text = WHITESPACE.replace_all(&text, " ");
  let text = text
    .replace("##ecnnewline##", "\n")
    .replace("##ecntab##", "\t")
    .replace("##ecnsemicolon##", ";")
    .replace("##nl##", "\n");

  text.lines().map(str::trim).collect::<Vec<_>>().join("\n")
}

pub fn clean_enum_text<'a, 'b, 'c>(translation_id: Option<&'a str>, index: Option<&'b str>, text: String) -> String {
  lazy_static! {
    static ref INDEX: Regex = Regex::new(r"^(?<name>.*)~(?<index>\d+)$").unwrap();
  }

  let index = if let Some(captures) = translation_id.and_then(|translation_id| INDEX.captures(translation_id)) {
    if captures.name("name").map(|m| m.as_str()) == Some("viessmann.eventvaluetype.K73_KonfiZpumpeIntervallFreigabe") {
      // False positive: <index> per hour
      None
    } else {
      captures.name("index").map(|m| m.as_str())
    }
  } else {
    index
  };

  let text = if let Some(index) = index {
    let index = regex::escape(index);

    let text =
      Regex::new(&format!(r"^(?:(?:{index}|\(0*{index}\))(?:\s*:|\s+-)?\s+)([^\s]+)")).unwrap().replace(&text, "$1");
    let text = Regex::new(r"^-*$").unwrap().replace(&text, "");
    text.into_owned()
  } else {
    text
  };

  text.trim().to_owned()
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcnCulture {
  #[serde(rename = "@id")]
  #[allow(unused)]
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  #[allow(unused)]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
  #[allow(unused)]
  pub msdata_row_order: IgnoredAny, // u16

  #[serde(rename = "Id")]
  pub id: u8,
  #[serde(rename = "CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "Name")]
  pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcnDataPointType {
  #[serde(rename = "@id")]
  #[allow(unused)]
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  #[allow(unused)]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
  #[allow(unused)]
  pub msdata_row_order: IgnoredAny, // u16

  #[serde(rename = "Id")]
  pub id: u32,
  #[serde(rename = "CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "Name")]
  pub name: String,
  #[serde(rename = "Description")]
  pub description: String,
  #[serde(rename = "StatusEventTypeId")]
  pub status_event_type_id: u8,
  #[serde(rename = "Address")]
  pub address: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcnDataPointTypeEventTypeLink {
  #[serde(rename = "@id")]
  #[allow(unused)]
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  #[allow(unused)]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
  #[allow(unused)]
  pub msdata_row_order: IgnoredAny, // u16

  #[serde(rename = "CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "DataPointTypeId")]
  pub data_point_type_id: u32,
  #[serde(rename = "EventTypeId")]
  pub event_type_id: u32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcnDeviceType {
  #[serde(rename = "@id")]
  #[allow(unused)]
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  #[allow(unused)]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
  #[allow(unused)]
  pub msdata_row_order: IgnoredAny, // u16

  #[serde(rename = "Id")]
  pub id: u8,
  #[serde(rename = "CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "Name")]
  pub name: String,
  #[serde(rename = "Manufacturer")]
  pub manufacturer: String,
  #[serde(rename = "Description")]
  pub description: String,
  #[serde(rename = "StatusDataPointTypeId")]
  pub status_data_point_type_id: u8,
  #[serde(rename = "TechnicalIdentificationAddress")]
  pub technical_identification_address: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcnDeviceTypeDataPointTypeLink {
  #[serde(rename = "@id")]
  #[allow(unused)]
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  #[allow(unused)]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
  #[allow(unused)]
  pub msdata_row_order: IgnoredAny, // u16

  #[serde(rename = "CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "DeviceTypeId")]
  pub device_type_id: u8,
  #[serde(rename = "DataPointTypeId")]
  pub data_point_type_id: u16,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcnEventType {
  #[serde(rename = "@id")]
  #[allow(unused)]
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  #[allow(unused)]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
  #[allow(unused)]
  pub msdata_row_order: IgnoredAny, // u16

  #[serde(rename = "Id")]
  pub id: u32,
  #[serde(rename = "CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "EnumType")]
  pub enum_type: bool,
  #[serde(rename = "Name")]
  pub name: String,
  #[serde(rename = "Address")]
  pub address: String,
  #[serde(rename = "Conversion")]
  pub conversion: String,
  #[serde(rename = "Description")]
  pub description: String,
  #[serde(rename = "Priority")]
  pub priority: u8,
  #[serde(rename = "Filtercriterion")]
  pub filter_criterion: bool,
  #[serde(rename = "Reportingcriterion")]
  pub reporting_criterion: bool,
  #[serde(rename = "Type")]
  pub access_mode: u8,
  #[serde(rename = "URL")]
  pub url: String,
  #[serde(rename = "DefaultValue")]
  pub default_value: String,
  #[serde(rename = "ConfigSetId")]
  #[allow(unused)]
  pub config_set_id: Option<u8>,
  #[serde(rename = "ConfigSetParameterId")]
  #[allow(unused)]
  pub config_set_parameter_id: Option<u16>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcnEventTypeEventValueTypeLink {
  #[serde(rename = "@id")]
  #[allow(unused)]
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  #[allow(unused)]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
  #[allow(unused)]
  pub msdata_row_order: IgnoredAny, // u16

  #[serde(rename = "CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "EventTypeId")]
  pub event_type_id: u32,
  #[serde(rename = "EventValueId")]
  pub event_value_id: u16,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcnEventValueType {
  #[serde(rename = "@id")]
  #[allow(unused)]
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  #[allow(unused)]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
  #[allow(unused)]
  pub msdata_row_order: IgnoredAny, // u16

  #[serde(rename = "Id")]
  pub id: u16,
  #[serde(rename = "CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "Name")]
  pub name: String,
  #[serde(rename = "EnumAddressValue")]
  pub enum_address_value: Option<i32>,
  #[serde(rename = "EnumReplaceValue")]
  pub enum_replace_value: String,
  #[serde(rename = "StatusTypeId")]
  pub status_type_id: u8,
  #[serde(rename = "Unit")]
  pub unit: String,
  #[serde(rename = "DataType")]
  pub data_type: String,
  #[serde(rename = "Length")]
  pub length: Option<u8>,
  #[serde(rename = "Stepping")]
  pub stepping: Option<f64>,
  #[serde(rename = "ValuePrecision")]
  pub value_precision: Option<u16>,
  #[serde(rename = "LowerBorder")]
  pub lower_border: Option<f64>,
  #[serde(rename = "UpperBorder")]
  pub upper_border: Option<f64>,
  #[serde(rename = "Description")]
  pub description: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcnTableExtensionValue {
  #[serde(rename = "@id")]
  #[allow(unused)]
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  #[allow(unused)]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
  #[allow(unused)]
  pub msdata_row_order: IgnoredAny, // u16

  #[serde(rename = "Id")]
  pub id: u32,
  #[serde(rename = "CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "RefId")]
  pub ref_id: u16,
  #[serde(rename = "PkValue")]
  #[serde_as(as = "StringWithSeparator::<SemicolonSeparator, String>")]
  pub pk_value: Vec<String>,
  #[serde(rename = "InternalValue")]
  #[serde_as(as = "Base64")]
  pub internal_value: Vec<u8>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcnVersion {
  #[serde(rename = "@id")]
  #[allow(unused)]
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  #[allow(unused)]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
  #[allow(unused)]
  pub msdata_row_order: IgnoredAny, // u16

  #[serde(rename = "Id")]
  pub id: u8,
  #[serde(rename = "CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "Name")]
  pub name: String,
  #[serde(rename = "Value")]
  pub value: String,
  #[serde(rename = "Description")]
  pub description: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcnTableExtension {
  #[serde(rename = "@id")]
  #[allow(unused)]
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  #[allow(unused)]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
  #[allow(unused)]
  pub msdata_row_order: IgnoredAny, // u16

  #[serde(rename = "Id")]
  pub id: u16,
  #[serde(rename = "CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "TableName")]
  pub table_name: String,
  #[serde(rename = "FieldName")]
  pub field_name: String,
  #[serde(rename = "Label")]
  #[allow(unused)]
  pub label: IgnoredAny,
  #[serde(rename = "PkFields")]
  #[serde_as(as = "StringWithSeparator::<SemicolonSeparator, String>")]
  pub pk_fields: Vec<String>,
  #[serde(rename = "InternalDefaultValue")]
  #[serde_as(as = "Base64")]
  pub internal_default_value: Vec<u8>,
  #[serde(rename = "InternalDataType")]
  pub internal_data_type: u8,
  #[serde(default, rename = "OptionsValue")]
  #[serde_as(as = "StringWithSeparator::<SemicolonSeparator, String>")]
  pub options_value: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct EcnDataSetInner {
  #[serde(rename = "ecnCulture")]
  pub ecn_culture: Vec<EcnCulture>,
  #[serde(rename = "ecnDatapointType")]
  pub ecn_datapoint_type: Vec<EcnDataPointType>,
  #[serde(rename = "ecnDataPointTypeEventTypeLink")]
  pub ecn_data_point_type_event_type_link: Vec<EcnDataPointTypeEventTypeLink>,
  #[serde(rename = "ecnDeviceType")]
  pub ecn_device_type: Vec<EcnDeviceType>,
  #[serde(rename = "ecnDeviceTypeDataPointTypeLink")]
  pub ecn_device_type_data_point_type_link: Vec<EcnDeviceTypeDataPointTypeLink>,
  #[serde(rename = "ecnEventType")]
  pub ecn_event_type: Vec<EcnEventType>,
  #[serde(rename = "ecnEventTypeEventValueTypeLink")]
  pub ecn_event_type_event_value_type_link: Vec<EcnEventTypeEventValueTypeLink>,
  #[serde(rename = "ecnEventValueType")]
  pub ecn_event_value_type: Vec<EcnEventValueType>,
  #[serde(rename = "ecnTableExtensionValue")]
  pub ecn_table_extension_value: Vec<EcnTableExtensionValue>,
  #[serde(rename = "ecnVersion")]
  pub ecn_version: Vec<EcnVersion>,
  #[serde(rename = "ecnTableExtension")]
  pub ecn_table_extension: Vec<EcnTableExtension>,
}

#[derive(Debug, Deserialize)]
pub struct ECNDataSetDiffGram {
  #[serde(rename = "ECNDataSet")]
  pub ecn_data_set: EcnDataSetInner,
}

#[derive(Debug, Deserialize)]
pub struct ECNDataSet {
  #[serde(rename = "diffgram")]
  pub diff_gram: ECNDataSetDiffGram,
}

#[derive(Debug, Deserialize)]
pub struct DocumentServerDataSetDiffGram {}

#[derive(Debug, Deserialize)]
pub struct DocumentServerDataSet {
  #[serde(rename = "diffgram")]
  #[allow(unused)]
  pub diff_gram: DocumentServerDataSetDiffGram,
}

#[derive(Debug, Deserialize)]
pub struct ImportExportDataHolder {
  #[serde(rename = "ECNDataSet")]
  pub ecn_data_set: ECNDataSet,
  #[serde(rename = "DocumentServerDataSet")]
  #[allow(unused)]
  pub document_server_data_set: DocumentServerDataSet,
}

pub fn parse_conversion(conversion: &str) -> Option<String> {
  match conversion {
    "NoConversion" | "" | "GWG_2010_Kennung~0x00F9" => None,
    "Div2" => Some("div2".to_owned()),
    "Div10" => Some("div10".to_owned()),
    "Div100" => Some("div100".to_owned()),
    "Div1000" => Some("div1000".to_owned()),
    "Mult2" => Some("mul2".to_owned()),
    "Mult5" => Some("mul5".to_owned()),
    "Mult10" => Some("mul10".to_owned()),
    "Mult100" => Some("mul100".to_owned()),
    "MultOffset" => Some("mul_offset".to_owned()),
    "MultOffsetBCD" => Some("mul_offset_bcd".to_owned()),
    "MultOffsetFloat" => Some("mul_offset_float".to_owned()),
    "DateBCD" => Some("date_bcd".to_owned()),
    "RotateBytes" => Some("rotate_bytes".to_owned()),
    "Time53" => Some("time53".to_owned()),
    "HexByte2UTF16Byte" => Some("hex_byte_to_utf16_byte".to_owned()),
    "HexByte2AsciiByte" => Some("hex_byte_to_ascii_byte".to_owned()),
    "HexByte2DecimalByte" => Some("hex_byte_to_decimal_byte".to_owned()),
    "HexByte2Version" => Some("hex_byte_to_version".to_owned()),
    "HexToFloat" => Some("hex_to_float".to_owned()),
    "Sec2Minute" => Some("sec_to_minute".to_owned()),
    "Sec2Hour" => Some("sec_to_hour".to_owned()),
    "Phone2BCD" => Some("phone_to_bcd".to_owned()),
    "Estrich" => Some("estrich".to_owned()),
    "VitocomNV" => Some("vitocom_nv".to_owned()),
    "Vitocom3NV" => Some("vitocom3_nv".to_owned()),
    "DatenpunktADDR" => Some("datenpunkt_addr".to_owned()),
    "Kesselfolge" => Some("kesselfolge".to_owned()),
    "LastBurnerCheck" => Some("last_burner_check".to_owned()),
    "LastCheckInterval" => Some("last_check_interval".to_owned()),
    "DateMBus" => Some("date_mbus".to_owned()),
    "DateTimeMBus" => Some("date_time_mbus".to_owned()),
    "DateTimeBCD" => Some("date_time_bcd".to_owned()),
    "DateTimeVitocom" => Some("date_time_vitocom".to_owned()),
    "FixedStringTerminalZeroes" => Some("fixed_string_terminal_zeroes".to_owned()),
    "Vitocom300SGEinrichtenKanalLON" => Some("vitocom300_sg_einrichten_kanal_lon".to_owned()),
    "Vitocom300SGEinrichtenKanalMBUS" => Some("vitocom300_sg_einrichten_kanal_mbus".to_owned()),
    "Vitocom300SGEinrichtenKanalWILO" => Some("vitocom300_sg_einrichten_kanal_wilo".to_owned()),
    "VitocomEingang" => Some("vitocom_eingang".to_owned()),
    "IPAddress" => Some("ip_address".to_owned()),
    "DayToDate" => Some("day_to_date".to_owned()),
    "DayMonthBCD" => Some("day_month_bcd".to_owned()),
    "BinaryToJson" => Some("binary_to_json".to_owned()),
    _ => unreachable!("unknown conversion: {conversion}"),
  }
}

pub fn parse_option_list(text: &str) -> Vec<String> {
  if text.is_empty() {
    return Vec::new();
  }

  text.split(";").map(snake_case).collect()
}

pub fn parse_value_list(text: &str) -> BTreeMap<u8, String> {
  if text.is_empty() {
    return BTreeMap::new();
  }

  text
    .split(";")
    .map(|v| v.split_once("=").unwrap())
    .map(|(k, v)| (k.parse::<u8>().unwrap(), clean_enum_text(None, Some(k), v.to_owned())))
    .collect()
}

pub fn strip_address<'s>(s: &'s str, address: &str) -> &'s str {
  s.strip_suffix(&format!("~{address}")).unwrap_or(s)
}
