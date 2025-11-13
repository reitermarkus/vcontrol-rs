use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum DataType {
  DeviceId,
  DeviceIdF0,
  String,
  Int,
  Double,
  Date,
  DateTime,
  CircuitTimes,
  ErrorIndex,
  Error,
  Byte,
  ByteArray,
}
