use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessMode {
  Read,
  Write,
  ReadWrite,
}

impl AccessMode {
  #[allow(unused)]
  pub fn is_read(self) -> bool {
    matches!(self, AccessMode::Read | AccessMode::ReadWrite)
  }

  #[allow(unused)]
  pub fn is_write(self) -> bool {
    matches!(self, AccessMode::Write | AccessMode::ReadWrite)
  }
}
