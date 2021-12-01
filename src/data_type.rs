use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum DataType {
  String,
  Int,
  Double,
  DateTime,
  CircuitTimes,
  ErrorIndex,
  Error,
  Byte,
  ByteArray,
}
