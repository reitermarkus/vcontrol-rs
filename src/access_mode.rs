use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessMode {
  Read,
  Write,
  ReadWrite,
}

impl AccessMode {
  pub fn is_read(self) -> bool {
    match self {
      AccessMode::Read | AccessMode::ReadWrite => true,
      _ => false,
    }
  }

  pub fn is_write(self) -> bool {
    match self {
      AccessMode::Write | AccessMode::ReadWrite => true,
      _ => false,
    }
  }
}