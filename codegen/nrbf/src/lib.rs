#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Int32(i32),
  Boolean(bool),
  Double(f64),
  String(String),
}

pub fn parse(bytes: &[u8]) -> Result<Value, &'static str> {
  Err("")
}
