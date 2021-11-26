use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum DataType {
  String,
  Int,
  Double,
  DateTime,
  CircuitTimes,
  Error,
  Byte,
  ByteArray,
}
