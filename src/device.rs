use phf;

use crate::{Error, Command, Optolink, Protocol, Value};

#[allow(clippy::unreadable_literal)]
mod codegen {
  use super::*;
  use crate::protocol::*;

  include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}

pub use self::codegen::*;

/// Representation of a heating system device.
#[derive(Debug)]
pub struct Device {
  name: &'static str,
  commands: &'static phf::Map<&'static str, &'static Command>,
}

impl Device {
  /// Get the name of the device.
  pub fn name(&self) -> &'static str {
    self.name
  }

  /// Get all supported commands for the device.
  pub fn commands(&self) -> &'static phf::Map<&'static str, &'static Command> {
    self.commands
  }

  /// Get a specific command for the device, if it is supported.
  pub fn command(&self, name: &str) -> Option<&'static Command> {
    self.commands.get(name).map(|c| *c)
  }
}
