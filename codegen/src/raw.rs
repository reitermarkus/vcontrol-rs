use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use serde::de::IgnoredAny;
use serde_with::base64::Base64;
use serde_with::serde_as;

#[derive(Debug, Deserialize)]
pub struct DefaultCulture {
  #[serde(rename = "@CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "@DefaultCultureId")]
  #[allow(unused)]
  pub default_culture_id: u8,
}

#[derive(Debug, Deserialize)]
pub struct DefaultCultures {
  #[serde(rename = "DefaultCulture")]
  #[allow(unused)]
  pub default_culture: DefaultCulture,
}

#[derive(Debug, Deserialize)]
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
pub struct Cultures {
  #[serde(default, rename = "Culture")]
  pub culture: Vec<Culture>,
}

#[derive(Debug, Deserialize)]
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
pub struct TextResources {
  #[serde(rename = "TextResource")]
  pub text_resource: Vec<TextResource>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DocumentElement {
  #[allow(unused)]
  pub default_cultures: DefaultCultures,
  pub cultures: Cultures,
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

pub fn clean_enum_text<'a, 'b, 'c>(translation_id: &'a str, index: Option<&'b str>, text: String) -> String {
  lazy_static! {
    static ref INDEX: Regex = Regex::new(r"^(?<name>.*)~(?<index>\d+)$").unwrap();
  }

  let index = if let Some(captures) = INDEX.captures(translation_id) {
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

pub fn simplify_translation_text(text: &str) -> String {
  WHITESPACE.replace_all(text, " ").trim().to_owned()
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcnCulture {
  #[serde(rename = "@id")]
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
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
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
  pub msdata_row_order: IgnoredAny, // u16

  #[serde(rename = "Id")]
  pub id: u16,
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
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
  pub msdata_row_order: IgnoredAny, // u16

  #[serde(rename = "CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "DataPointTypeId")]
  pub data_point_type_id: u16,
  #[serde(rename = "EventTypeId")]
  pub event_type_id: u16,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcnDeviceType {
  #[serde(rename = "@id")]
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
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
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
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
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
  pub msdata_row_order: IgnoredAny, // u16

  #[serde(rename = "Id")]
  pub id: u16,
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
  pub type_: u8,
  #[serde(rename = "URL")]
  pub url: String,
  #[serde(rename = "DefaultValue")]
  pub default_value: String,
  #[serde(rename = "ConfigSetId")]
  pub config_set_id: Option<u8>,
  #[serde(rename = "ConfigSetParameterId")]
  pub config_set_parameter_id: Option<u16>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcnEventTypeEventValueTypeLink {
  #[serde(rename = "@id")]
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
  pub msdata_row_order: IgnoredAny, // u16

  #[serde(rename = "CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "EventTypeId")]
  pub event_type_id: u16,
  #[serde(rename = "EventValueId")]
  pub event_value_id: u16,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcnEventValueType {
  #[serde(rename = "@id")]
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
  pub msdata_row_order: IgnoredAny, // u16

  #[serde(rename = "Id")]
  pub id: u16,
  #[serde(rename = "CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "Name")]
  pub name: String,
  #[serde(default, rename = "EnumAddressValue")]
  pub enum_address_value: i32,
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
  pub stepping: Option<f32>,
  #[serde(rename = "ValuePrecision")]
  pub value_precision: Option<u16>,
  #[serde(rename = "LowerBorder")]
  pub lower_border: Option<f32>,
  #[serde(rename = "UpperBorder")]
  pub uppwer_border: Option<f32>,
  #[serde(rename = "Description")]
  pub description: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcnTableExtensionValue {
  #[serde(rename = "@id")]
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
  pub msdata_row_order: IgnoredAny, // u16

  #[serde(rename = "Id")]
  pub id: u32,
  #[serde(rename = "CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "RefId")]
  pub ref_id: u16,
  #[serde(rename = "PkValue")]
  pub pk_value: String,
  #[serde(rename = "InternalValue")]
  #[serde_as(as = "Base64")]
  pub internal_value: Vec<u8>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcnVersion {
  #[serde(rename = "@id")]
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
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
  pub diffgr_id: IgnoredAny, // String
  #[serde(rename = "@hasChanges")]
  pub diffgr_has_changes: IgnoredAny, // String
  #[serde(rename = "@rowOrder")]
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
  pub label: String,
  #[serde(rename = "PkFields")]
  pub pk_fields: String,
  #[serde(rename = "InternalDefaultValue")]
  #[serde_as(as = "Base64")]
  pub internal_default_value: Vec<u8>,
  #[serde(rename = "InternalDataType")]
  pub internal_data_type: u8,
  #[serde(default, rename = "OptionsValue")]
  pub options_value: String,
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
  pub diff_gram: DocumentServerDataSetDiffGram,
}

// Datapoint definitions.

#[derive(Debug, Deserialize)]
pub struct ImportExportDataHolder {
  #[serde(rename = "ECNDataSet")]
  pub ecn_data_set: ECNDataSet,
  #[serde(rename = "DocumentServerDataSet")]
  pub document_server_data_set: DocumentServerDataSet,
}
