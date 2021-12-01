use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, de::DeserializeOwned};
use serde_yaml;
use phf_codegen;

#[path = "src/access_mode.rs"]
mod access_mode;
use access_mode::AccessMode;

#[path = "src/device_ident_range.rs"]
mod device_ident_range;
use device_ident_range::DeviceIdentRange;

#[path = "src/data_type.rs"]
mod data_type;
use data_type::DataType;

#[path = "src/parameter.rs"]
mod parameter;
use parameter::Parameter;

fn escape_const_name(s: &str) -> String {
  s.to_uppercase().replace(".", "_").replace("|", "_").replace(" ", "_").replace("-", "_").replace("~", "_").replace("%", "PERCENT")
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

  let mut device_map = phf_codegen::Map::<DeviceIdentRange>::new();
  for (device_id, device) in &mappings {
    let id_range = DeviceIdentRange {
      id: device.id,
      hardware_index_range: ((device.id_ext >> 8) as u8)..=((device.id_ext_till >> 8) as u8),
      software_index_range: ((device.id_ext & 0xff) as u8)..=((device.id_ext_till & 0xff) as u8),
      f0_range: (device.f0)..=(device.f0_till),
    };
    device_map.entry(id_range, &format!("&{}", escape_const_name(&device_id)));

    let mut map = phf_codegen::Map::<&str>::new();
    for command_name in device.commands.iter() {
      map.entry(command_name, &format!("&COMMAND_{}", escape_const_name(&command_name)));
    }
    writeln!(&mut file, "const {}_COMMANDS: ::phf::Map<&'static str, &'static Command> = {};", escape_const_name(&device_id), map.build()).unwrap();

    writeln!(file, r#"
      const {}: Device = Device {{
        name: {},
        commands: &{}_COMMANDS,
        errors: &MAPPING_{},
      }};
    "#, escape_const_name(&device_id), format!("{:?}", device_id), escape_const_name(&device_id), escape_const_name(&device.error_mapping)).unwrap();
  }
  writeln!(&mut file, r#"
    /// A map of all supported devices.
    pub const DEVICES: ::phf::Map<DeviceIdentRange, &'static Device> = {};
  "#, device_map.build()).unwrap();
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
  f0: u16,
  f0_till: u16,
  commands: Vec<String>,
  error_mapping: String
}

/// A command which can be executed on an Optolink connection.
#[derive(Deserialize)]
pub struct Command {
  addr: u16,
  mode: AccessMode,
  data_type: DataType,
  parameter: Parameter,
  block_len: usize,
  byte_len: usize,
  byte_pos: usize,
  bit_pos: usize,
  bit_len: usize,
  conversion: String,
  conversion_factor: Option<f64>,
  conversion_offset: Option<f64>,
  lower_border: Option<f64>,
  upper_border: Option<f64>,
  unit: Option<String>,
  mapping: Option<String>,
}

impl fmt::Debug for Command {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mapping = if let Some(mapping) = &self.mapping {
      format!("Some(MAPPING_{})", escape_const_name(&mapping))
    } else {
      "None".into()
    };

    let conversion = if self.conversion == "MulOffset" {
      #[derive(Debug)]
      struct MulOffset {
        factor: f64,
        offset: f64,
      }

      format!("{:?}", MulOffset {
        factor: self.conversion_factor.unwrap_or(1.0),
        offset: self.conversion_offset.unwrap_or(0.0),
      })
    } else {
      self.conversion.to_owned()
    };

    f.debug_struct("Command")
       .field("addr", &format_args!("0x{:04X}", self.addr))
       .field("mode", &format_args!("crate::AccessMode::{:?}", self.mode))
       .field("data_type", &format_args!("crate::DataType::{:?}", self.data_type))
       .field("parameter", &format_args!("crate::Parameter::{:?}", self.parameter))
       .field("block_len", &self.block_len)
       .field("byte_len", &self.byte_len)
       .field("byte_pos", &self.byte_pos)
       .field("bit_len", &self.bit_len)
       .field("bit_pos", &self.bit_pos)
       .field("conversion", &format_args!("crate::Conversion::{}", conversion))
       .field("unit", &self.unit)
       .field("mapping", &format_args!("{}", mapping))
       .finish()
  }
}
