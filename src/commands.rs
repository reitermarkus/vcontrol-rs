use crate::Command;

/// An iterator over command name and command pairs.
#[derive(Debug, Clone)]
pub struct Commands {
  iter: phf::map::Entries<'static, &'static str, &'static Command>,
}

impl Iterator for Commands {
  type Item = (&'static str, &'static Command);

  fn next(&mut self) -> Option<Self::Item> {
    self.iter.next().map(|(k, v)| (*k, *v))
  }
}

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
pub fn system_commands() -> Commands {
  Commands { iter: SYSTEM_COMMANDS.entries() }
}
