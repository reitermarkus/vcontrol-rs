use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, de::DeserializeOwned};
use serde_yaml;
use phf_codegen;

#[path = "src/raw_type.rs"]
mod raw_type;
use raw_type::RawType;

fn escape_const_name(s: &str) -> String {
  s.to_uppercase().replace(".", "_").replace("|", "_").replace(" ", "_").replace("-", "_")
}

#[track_caller]
fn load_yaml<T: DeserializeOwned>(file_name: &str) -> T {
  let path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("codegen").join(file_name);
  let file = BufReader::new(File::open(&path).expect(&format!("Error opening {:?}", path)));
  serde_yaml::from_reader(file).unwrap()
}

fn output_file(file_name: &str) -> BufWriter<File> {
  let path = Path::new(&env::var("OUT_DIR").unwrap()).join(file_name);
  BufWriter::new(File::create(&path).unwrap())
}

fn generate_translations() {
  println!("Generating translations.");

  let translations: BTreeMap<String, String> = load_yaml("used_translations.yml");

  let mut file = output_file("translations.rs");

  for (k, v) in translations {
    writeln!(file, "const TRANSLATION_{}: &'static str = {:?};", escape_const_name(&k), v).unwrap();
  }
}

fn generate_mappings() {
  println!("Generating mappings.");

  let mappings: BTreeMap<String, BTreeMap<i32, String>> = load_yaml("used_mappings.yml");

  let mut file = output_file("mappings.rs");

  writeln!(file, r#"include!(concat!(env!("OUT_DIR"), "/translations.rs"));"#).unwrap();

  for (k, mapping) in mappings {
    let mut map = phf_codegen::Map::new();

    for (k, v) in mapping {
      map.entry(k, &format!("TRANSLATION_{}", escape_const_name(&v)));
    }

    // let v = v.as_str().unwrap();
    writeln!(file, "const MAPPING_{}: ::phf::Map<i32, &'static str> = {};", escape_const_name(&k), map.build()).unwrap();
  }
}

fn generate_commands() {
  println!("Generating commands.");

  let mappings: BTreeMap<String, Command> = load_yaml("used_commands.yml");

  let mut file = output_file("commands.rs");

  writeln!(file, r#"include!(concat!(env!("OUT_DIR"), "/mappings.rs"));"#).unwrap();

  for (command_name, command) in mappings {
    writeln!(file, "const COMMAND_{}: Command = {:?};", escape_const_name(&command_name), command).unwrap();
  }
}

fn generate_devices() {
  println!("Generating devices.");

  let mappings: BTreeMap<String, Device> = load_yaml("used_devices.yml");

  let mut file = output_file("devices.rs");

  writeln!(file, r#"include!(concat!(env!("OUT_DIR"), "/commands.rs"));"#).unwrap();

  let mut device_map = phf_codegen::Map::<u64>::new();
  for (device_id, device) in &mappings {
    let id = ((device.id as u64) << 32) + ((device.id_ext as u64) << 16) + (device.id_ext_till as u64);
    device_map.entry(id, &format!("&{}", escape_const_name(&device_id)));
  }
  writeln!(&mut file, r#"
    /// A map of all supported devices.
    pub const DEVICES: ::phf::Map<u64, &'static Device> = {};
  "#, device_map.build()).unwrap();


  for (device_id, device) in mappings {
    let mut map = phf_codegen::Map::<&str>::new();


    for command_name in device.commands.iter() {
      map.entry(command_name, &format!("&COMMAND_{}", escape_const_name(&command_name)));
    }

    writeln!(&mut file, "const {}_COMMANDS: ::phf::Map<&'static str, &'static Command> = {};", escape_const_name(&device_id), map.build()).unwrap();

    writeln!(file, r#"
      const {}: Device = Device {{ name: {}, commands: &{}_COMMANDS, errors: &MAPPING_{} }};
    "#, escape_const_name(&device_id), format!("{:?}", device_id), escape_const_name(&device_id), escape_const_name(&device.error_mapping)).unwrap();
  }
}

fn main() {
  generate_translations();
  generate_mappings();
  generate_commands();
  generate_devices();

  let mut file = output_file("codegen.rs");
  writeln!(file, r#"include!(concat!(env!("OUT_DIR"), "/devices.rs"));"#).unwrap();
}

#[derive(Debug, Deserialize)]
pub struct Device {
  id: u16,
  id_ext: u16,
  id_ext_till: u16,
  commands: Vec<String>,
  error_mapping: String
}

/// A command which can be executed on an Optolink connection.
#[derive(Deserialize)]
pub struct Command {
  addr: u16,
  mode: AccessMode,
  data_type: DataType,
  raw_type: RawType,
  block_len: Option<usize>,
  byte_len: Option<usize>,
  byte_pos: Option<usize>,
  bit_pos: usize,
  bit_len: usize,
  factor: Option<f64>,
  mapping: Option<String>,
}

impl fmt::Debug for Command {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let block_len = self.block_len.or_else(|| self.raw_type.size()).unwrap();
    let byte_len = self.byte_len.or_else(|| self.raw_type.size()).unwrap();
    let byte_pos = self.byte_pos.unwrap_or(0);

    let mapping = if let Some(mapping) = &self.mapping {
      format!("Some(MAPPING_{})", escape_const_name(&mapping))
    } else {
      "None".into()
    };

    f.debug_struct("Command")
       .field("addr", &format_args!("0x{:04X}", self.addr))
       .field("mode", &format_args!("crate::AccessMode::{:?}", self.mode))
       .field("data_type", &format_args!("crate::DataType::{:?}", self.data_type))
       .field("raw_type", &format_args!("crate::RawType::{:?}", self.raw_type))
       .field("block_len", &block_len)
       .field("byte_len", &byte_len)
       .field("byte_pos", &byte_pos)
       .field("bit_len", &self.bit_len)
       .field("bit_pos", &self.bit_pos)
       .field("factor", &self.factor)
       .field("mapping", &format_args!("{}", mapping))
       .finish()
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessMode {
  Read,
  Write,
  ReadWrite,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataType {
  Int,
  Double,
  String,
  Array,
  SysTime,
  CycleTimes,
  Error,
}
