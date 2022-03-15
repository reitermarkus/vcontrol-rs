use crate::Command;

mod codegen {
  include!(concat!(env!("OUT_DIR"), "/commands.rs"));
  include!(concat!(env!("OUT_DIR"), "/system_commands.rs"));
}

pub use self::codegen::*;

/// Get a system command by name.
pub fn system_command(name: impl AsRef<str>) -> Option<&'static Command> {
  SYSTEM_COMMANDS.get(name.as_ref()).map(|&c| c)
}

/// Iterate over system commands.
pub fn system_commands() -> &'static phf::Map<&'static str, &'static Command> {
  &SYSTEM_COMMANDS
}
