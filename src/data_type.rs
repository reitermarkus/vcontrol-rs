use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
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
