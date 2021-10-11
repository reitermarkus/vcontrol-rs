use std::env;
use std::fs::File;
use std::io::{Read, BufReader, BufWriter, Write};
use std::path::Path;
use std::collections::HashMap;
use std::fmt;

use serde::Deserialize;
use serde_yaml;
use yaml_merge_keys;
use phf_codegen;

#[path = "src/raw_type.rs"]
mod raw_type;
use raw_type::RawType;

#[path = "src/types/mod.rs"]
mod types;
use self::types::*;

fn main() {
  let device = "VBC550P";

  let config_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("config").join(format!("{}.yml", device));

  let file = File::open(config_path).unwrap();

  let mut content = String::new();
  BufReader::new(file).read_to_string(&mut content).unwrap();

  let yaml = serde_yaml::from_str::<serde_yaml::Value>(&content).unwrap();
  let merged_yaml = yaml_merge_keys::merge_keys_serde(yaml).unwrap();
  let config: Configuration = serde_yaml::from_value(merged_yaml).unwrap();

  let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
  let mut file = BufWriter::new(File::create(&path).unwrap());

  let protocol = config.device.protocol;

  let mut map = phf_codegen::Map::<&str>::new();

  for (name, command) in config.commands.iter() {
    map.entry(name, &format!("{:?}", command));
  }

  writeln!(&mut file, "static {}_COMMANDS: phf::Map<&'static str, Command> = {};", device, map.build()).unwrap();

  write!(&mut file, "
    #[derive(Debug)]
    pub enum {} {{}}

    impl Device for {} {{
      type Protocol = {};

      #[inline(always)]
      fn map() -> &'static phf::Map<&'static str, Command> {{
        &{}_COMMANDS
      }}
    }}
  ", device, device, protocol, device).unwrap();
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
  pub device: Device,
  pub commands: HashMap<String, Command>,
}

#[derive(Debug, Deserialize)]
pub struct Device {
  protocol: String
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
  bit_pos: Option<usize>,
  bit_len: Option<usize>,
  factor: Option<f64>,
  mapping: Option<HashMap<Vec<u8>, String>>,
}

impl fmt::Debug for Command {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    println!("{:0X}", self.addr);
    let block_len = self.block_len.or_else(|| self.raw_type.size()).unwrap();
    let byte_len = self.byte_len.or_else(|| self.raw_type.size()).unwrap();
    let byte_pos = self.byte_pos.unwrap_or(0);

    let mapping = if let Some(mapping) = &self.mapping {
      let mut map = phf_codegen::Map::new();

      for (k, v) in mapping {
        map.entry(Bytes::from_bytes(k), &format!("{:?}", v));
      }

      format!("Some({})", map.build())
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
  I8,
  I16,
  I32,
  U8,
  U16,
  U32,
  F32,
  F64,
  String,
  Array,
  SysTime,
  CycleTime,
  Error,
}

impl DataType {
  pub fn size(&self) -> usize {
    match self {
      Self::I8 => std::mem::size_of::<i8>(),
      Self::I16 => std::mem::size_of::<i16>(),
      Self::I32 => std::mem::size_of::<i32>(),
      Self::U8 => std::mem::size_of::<u8>(),
      Self::U16 => std::mem::size_of::<u16>(),
      Self::U32 => std::mem::size_of::<u32>(),
      Self::String => std::mem::size_of::<String>(),
      Self::SysTime => std::mem::size_of::<SysTime>(),
      Self::CycleTime => std::mem::size_of::<CycleTime>(),
      Self::Error => std::mem::size_of::<Error>(),
      _ => panic!("unit has dynamic size: {:?}", self),
    }
  }
}
