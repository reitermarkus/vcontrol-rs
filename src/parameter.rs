use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum Parameter {
  Byte = 1,
  SByte,
  Int,
  SInt,
  Int4,
  SInt4,
  IntHighByteFirst,
  SIntHighByteFirst,
  Int4HighByteFirst,
  SInt4HighByteFirst,
  Array,
  String,
  StringNt,
  StringCr,
}
