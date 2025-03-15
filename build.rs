use std::{
  collections::BTreeMap,
  env, fmt,
  fs::File,
  io::{self, BufReader, BufWriter, Write},
  path::Path,
};

use anyhow::Context;
use serde::{Deserialize, de::DeserializeOwned};

#[path = "src/access_mode.rs"]
mod access_mode;
use access_mode::AccessMode;

#[path = "src/device/device_id_range.rs"]
mod device_id_range;
use device_id_range::DeviceIdRange;

#[path = "src/data_type.rs"]
mod data_type;
use data_type::DataType;

#[path = "src/parameter.rs"]
mod parameter;
use parameter::Parameter;

#[path = "src/conversion.rs"]
mod conversion;
use conversion::Conversion;

fn escape_const_name(s: &str) -> String {
  s.to_uppercase().replace(['.', '|', ' ', '-', '~'], "_").replace('%', "PERCENT")
}

#[track_caller]
fn load_yaml<T: DeserializeOwned>(file_name: &str) -> anyhow::Result<T> {
  let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");
  let path = Path::new(&cargo_manifest_dir).join("codegen").join(file_name);
  let file = BufReader::new(File::open(&path).with_context(|| format!("Error opening {:?}", path))?);
  Ok(serde_yaml::from_reader(file)?)
}

fn output_file(file_name: &str) -> io::Result<BufWriter<File>> {
  let path = Path::new(&env::var("OUT_DIR").expect("OUT_DIR is not set")).join(file_name);
  Ok(BufWriter::new(File::create(path)?))
}

fn generate_translations() -> anyhow::Result<()> {
  println!("Generating translations.");

  let translations: BTreeMap<u16, String> = load_yaml("translations.used.yml")?;

  let mut file = output_file("translations.rs")?;

  for (k, v) in translations {
    writeln!(file, "const TRANSLATION_{}: &str = {:?};", k, v)?;
  }

  Ok(())
}

fn generate_mappings() -> anyhow::Result<()> {
  println!("Generating mappings.");

  let mappings: BTreeMap<u16, BTreeMap<i32, u16>> = load_yaml("mappings.used.yml")?;

  let mut file = output_file("mappings.rs")?;

  writeln!(file, r#"include!(concat!(env!("OUT_DIR"), "/translations.rs"));"#)?;

  for (k, mapping) in mappings {
    let mut map = phf_codegen::Map::new();

    for (k, v) in mapping {
      map.entry(k, &format!("TRANSLATION_{}", v));
    }

    writeln!(file, "\npub const MAPPING_{}: ::phf::Map<i32, &'static str> = {};", k, map.build())?;
  }

  Ok(())
}

fn generate_commands() -> anyhow::Result<BTreeMap<u16, String>> {
  println!("Generating commands.");

  let mut command_name_map = BTreeMap::new();
  let mappings: BTreeMap<u16, Command> = load_yaml("event_types.used.yml")?;

  let mut file = output_file("commands.rs")?;

  let mut max_payload_len = 0;

  for (command_id, command) in mappings {
    let command_name = &command.name;
    writeln!(file, "\npub const COMMAND_{}: crate::Command = {:?};", command_id, command)?;

    max_payload_len = max_payload_len.max(command.block_len);

    command_name_map.insert(command_id, command_name.clone());
  }

  writeln!(file, "\npub const MAX_PAYLOAD_LEN: usize = {max_payload_len};")?;

  Ok(command_name_map)
}

fn generate_system_commands() -> anyhow::Result<()> {
  println!("Generating system commands.");

  let commands: BTreeMap<String, Command> = load_yaml("system_event_types.used.yml")?;

  let mut file = output_file("system_commands.rs")?;

  let mut map = phf_codegen::Map::<&str>::new();

  writeln!(file, "\npub mod system {{")?;

  for (command_name, command) in &commands {
    let constant_name = command_name.to_uppercase();

    map.entry(command_name, &format!("&system::{}", constant_name));

    writeln!(file, "\npub const {}: crate::Command = {:#?};", constant_name, command)?;
  }

  writeln!(file, "\n}}")?;

  writeln!(
    file,
    "\npub(crate) const SYSTEM_COMMANDS: ::phf::Map<&'static str, &'static crate::Command> = {};",
    map.build()
  )?;

  Ok(())
}

fn generate_devices(command_name_map: &BTreeMap<u16, String>) -> anyhow::Result<()> {
  println!("Generating devices.");

  let mappings: BTreeMap<String, Device> = load_yaml("devices.used.yml")?;

  let mut file = output_file("devices.rs")?;

  let mut device_map = phf_codegen::Map::<DeviceIdRange>::new();
  for (device_id, device) in &mappings {
    let id_range = DeviceIdRange {
      group_id: ((device.id & 0xff00) >> 2) as u8,
      id: (device.id & 0x00ff) as u8,
      hardware_index: device.id_ext.map(|id_ext| (id_ext >> 8) as u8),
      hardware_index_till: device.id_ext_till.map(|id_ext_till| (id_ext_till >> 8) as u8),
      software_index: device.id_ext.map(|id_ext| (id_ext & 0xff) as u8),
      software_index_till: device.id_ext_till.map(|id_ext_till| (id_ext_till & 0xff) as u8),
      f0: device.f0,
      f0_till: device.f0_till,
    };
    device_map.entry(id_range, &format!("&{}", escape_const_name(device_id)));

    let mut map = phf_codegen::Map::<&str>::new();
    for command_id in device.commands.iter() {
      let command_name = command_name_map.get(command_id).unwrap();
      map.entry(command_name, &format!("&crate::commands::COMMAND_{}", command_id));
    }
    writeln!(
      file,
      "const {}_COMMANDS: ::phf::Map<&'static str, &'static crate::Command> = {};",
      escape_const_name(device_id),
      map.build()
    )?;

    writeln!(
      file,
      r#"
      pub const {}: Device = Device {{
        name: {:?},
        commands: &{}_COMMANDS,
        errors: &crate::mappings::MAPPING_{},
      }};
    "#,
      escape_const_name(device_id),
      device_id,
      escape_const_name(device_id),
      device.error_mapping
    )?;
  }

  writeln!(
    file,
    r#"    pub(crate) const DEVICES: ::phf::Map<DeviceIdRange, &'static Device> = {};"#,
    device_map.build()
  )?;

  Ok(())
}

fn main() -> anyhow::Result<()> {
  generate_translations()?;
  generate_mappings()?;
  let command_name_map = generate_commands()?;
  generate_system_commands()?;
  generate_devices(&command_name_map)?;

  Ok(())
}

#[derive(Debug, Deserialize)]
pub struct Device {
  id: u16,
  id_ext: Option<u16>,
  id_ext_till: Option<u16>,
  f0: Option<u16>,
  f0_till: Option<u16>,
  commands: Vec<u16>,
  error_mapping: u16,
}

/// A command which can be executed on an Optolink connection.
#[derive(Deserialize)]
pub struct Command {
  name: String,
  addr: u16,
  mode: AccessMode,
  data_type: DataType,
  parameter: Parameter,
  block_count: Option<usize>,
  block_len: usize,
  byte_len: usize,
  byte_pos: usize,
  bit_pos: usize,
  bit_len: Option<usize>,
  conversion: Option<Conversion>,
  lower_border: Option<f64>,
  upper_border: Option<f64>,
  unit: Option<String>,
  mapping: Option<String>,
}

impl fmt::Debug for Command {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mapping = if let Some(mapping) = &self.mapping {
      format!("Some(crate::mappings::MAPPING_{})", escape_const_name(mapping))
    } else {
      "None".into()
    };

    let conversion = if let Some(conversion) = &self.conversion {
      format!("Some(crate::conversion::Conversion::{:?})", conversion)
    } else {
      "None".into()
    };

    f.debug_struct("crate::Command")
      .field("addr", &format_args!("0x{:04X}", self.addr))
      .field("mode", &format_args!("crate::AccessMode::{:?}", self.mode))
      .field("data_type", &format_args!("crate::DataType::{:?}", self.data_type))
      .field("parameter", &format_args!("crate::Parameter::{:?}", self.parameter))
      .field("block_count", &self.block_count)
      .field("block_len", &self.block_len)
      .field("byte_len", &self.byte_len)
      .field("byte_pos", &self.byte_pos)
      .field("bit_len", &self.bit_len)
      .field("bit_pos", &self.bit_pos)
      .field("conversion", &format_args!("{}", conversion))
      .field("lower_bound", &self.lower_border)
      .field("upper_bound", &self.upper_border)
      .field("unit", &self.unit)
      .field("mapping", &format_args!("{}", mapping))
      .finish()
  }
}
