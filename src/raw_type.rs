use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RawType {
  I8,
  I16,
  I32,
  U8,
  U16,
  U32,
  Array,
}

impl RawType {
  /// Return size for types with static size.
  pub fn size(&self) -> Option<usize> {
    Some(match self {
      Self::I8 => 1,
      Self::I16 => 2,
      Self::I32 => 4,
      Self::U8 => 1,
      Self::U16 => 2,
      Self::U32 => 4,
      _ => return None,
    })
  }
}
