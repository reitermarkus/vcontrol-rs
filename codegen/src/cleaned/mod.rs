use std::collections::BTreeMap;

use lazy_static::lazy_static;
use regex::Regex;
use stringcase::snake_case;

mod data_point_type;
pub use data_point_type::DataPointType;
mod event_type;
pub use event_type::{Conversion, ConversionInner, EventType, EventValueType};

pub fn parse_function(f: String) -> Option<String> {
  match f.as_str() {
    "" | "undefined" => None,
    _ => Some(snake_case(&f)),
  }
}

lazy_static! {
  static ref WHITESPACE: Regex = Regex::new(r"\s+").unwrap();
}

pub fn simplify_translation_text(text: &str) -> String {
  WHITESPACE.replace_all(text, " ").trim().to_owned()
}

pub fn parse_description(
  text: &str,
  translations: &BTreeMap<String, String>,
  reverse_translations: &BTreeMap<String, &String>,
) -> Option<String> {
  match text.trim() {
    "" => None,
    v if v.starts_with("@@") => Some(v.to_owned()),
    v => {
      let k = simplify_translation_text(v);
      if let Some(translation_id) = reverse_translations.get(&k) {
        Some(format!("@@{translation_id}"))
      } else {
        if translations.contains_key(v) {
          Some(format!("@@{v}"))
        } else {
          let k2 = format!("viessmann.eventtype.{v}.description");
          if translations.contains_key(&k2) { Some(format!("@@{k2}")) } else { None }
        }
      }
    },
  }
}

pub fn parse_default_value(default_value: &str) -> Option<serde_json::Value> {
  match default_value {
    "" => None,
    value => {
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
          Some(serde_json::Value::String(format!("{year:04}-{month:02}-{day:02}")))
        } else if let Some(captures) = DATE_TIME_REGEX.captures(&value) {
          let day = captures["day"].parse::<u8>().unwrap();
          let month = captures["month"].parse::<u8>().unwrap();
          let year = captures["year"].parse::<u16>().unwrap();
          let hour = captures["hour"].parse::<u8>().unwrap();
          let minute = captures["minute"].parse::<u8>().unwrap();
          let second = captures["second"].parse::<u8>().unwrap();
          Some(serde_json::Value::String(format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}")))
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
          Some(serde_json::Value::String(value))
        }
      }
    },
  }
}

pub fn parse_unit(unit: &str) -> Option<&'static str> {
  Some(match unit.strip_prefix("ecnUnit.").unwrap_or(unit) {
    "Minuten" => "min",
    "Grad C" => "°C",
    "Prozent" => "%",
    "K" => "K",
    "Sekunden" | "Sek." => "s",
    "Stunden" => "h",
    "Prozent pro K" => "%/K",
    "Bar" => "bar",
    "Ohm" => "Ω",
    "K Sec" => "K/s",
    "K Min" => "K/min",
    "K pro h" => "K/h",
    "Monate" => "mo",
    "kW" | "KW" | "kW_10" => "kW",
    "MWh" => "MWh",
    "kWh" => "kWh",
    "l pro min" => "l/min",
    "l pro h" => "l/h",
    "m3 pro h" | "cbm pro h" => "m³/h",
    "m3" => "m³",
    "kWh pro m3" => "kWh/m³",
    "Tage" => "d",
    "Liter" => "l",
    "kg" => "kg",
    "rps" => "rev/s",
    "rps pro s" => "rev/s²",
    "U pro min" => "rev/min",
    "Grad C pro Min" => "°C/min",
    "Tonnen" => "t",
    "mBar" => "mbar",
    "dBm" => "dBm",
    "Bar (absolut)" => "bara",
    "ct_pro_kwh" => "c/kWh",
    "g_pro_sec" => "g/s",
    "kg_pro_h" => "kg/h",
    "h" => "h",
    "V" => "V",
    "mV" => "mV",
    "A" => "A",
    "Hz" => "Hz",
    "W" => "W",
    "Wh" => "Wh",
    "VA" => "VA",
    "VAr" => "VAr",
    "Ah" => "Ah",
    "kJ" => "kJ",
    "MJ" => "MJ",
    "GJ" => "GJ",
    "ppm" => "ppm",
    "" | "Minus" | "Pts" | "Pkt" | "sech" => return None,
    unit => unreachable!("Unknown unit: {unit}"),
  })
}
