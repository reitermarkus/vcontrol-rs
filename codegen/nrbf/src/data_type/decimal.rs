use std::str::FromStr;

use nom::IResult;

use crate::{
  data_type::LengthPrefixedString,
  enumeration::PrimitiveType,
  error::{error_position, Error},
};

/// 2.1.1.7 `Decimal`
#[derive(Debug, Clone, PartialEq)]
pub struct Decimal(pub rust_decimal::Decimal);

impl Decimal {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    let err_input = input;

    let (input, s) = LengthPrefixedString::parse(input)?;

    if let Ok(decimal) = rust_decimal::Decimal::from_str(s.as_str()) {
      Ok((input, Self(decimal)))
    } else {
      Err(nom::Err::Failure(error_position!(err_input, ExpectedPrimitive(PrimitiveType::Decimal))))
    }
  }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Decimal {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::de::Deserializer<'de>,
  {
    use serde::de::{self, Error, MapAccess, Unexpected};

    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
      type Value = rust_decimal::Decimal;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Decimal")
      }

      fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
      where
        A: MapAccess<'de>,
      {
        use serde::Deserialize;

        #[derive(Deserialize)]
        enum Value {
          Flags(i32),
          Hi32(u32),
          Lo64(u64),
        }

        let mut flags = None;
        let mut hi32 = None;
        let mut lo64 = None;
        let mut values = 0;

        while let Some((key, value)) = map.next_entry::<&str, Value>()? {
          match (key, value) {
            ("_flags", Value::Flags(v)) if flags.is_none() => {
              flags = Some(v);
            },
            ("_hi32", Value::Hi32(v)) if hi32.is_none() => {
              hi32 = Some(v);
            },
            ("_lo32", Value::Lo64(v)) if lo64.is_none() => {
              lo64 = Some(v);
            },
            _ => break,
          }

          values += 1;
        }

        if values == 3 {
          if let (Some(flags), Some(hi32), Some(lo64)) = (flags, hi32, lo64) {
            let mut n = (i128::from(hi32) << 64) | i128::from(lo64);
            let scale = (flags >> 16) as u8;

            if (flags & (0x80000000u32 as i32)) != 0 {
              n = -n;
            }

            return Ok(rust_decimal::Decimal::from_i128_with_scale(n, scale.into()))
          }
        }

        Err(Error::invalid_type(Unexpected::Map, &self))
      }
    }

    deserializer.deserialize_map(Visitor).map(Self)
  }
}
