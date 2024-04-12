use std::str;

use nom::{
  bytes::complete::take,
  combinator::{map, map_res},
  sequence::{pair, preceded},
  IResult,
};

use crate::{
  combinator::into_failure,
  enumeration::PrimitiveType,
  error::{error_position, Error},
};

/// 2.1.1.6 `LengthPrefixedString`
#[derive(Debug, Clone, PartialEq)]
pub struct LengthPrefixedString<'s>(pub(crate) &'s str);

impl<'i> LengthPrefixedString<'i> {
  fn parse_len(input: &[u8]) -> IResult<&[u8], u32> {
    let parse_len = |mut input| -> IResult<(&[u8], usize), u32> {
      use nom::bits::complete::{bool, tag, take};

      let mut len = 0;

      let (mut parse_next, mut len_part): (bool, u8);
      (input, (parse_next, len_part)) = pair(bool, take(7usize))(input)?;
      len |= len_part as u32;

      if parse_next {
        (input, (parse_next, len_part)) = pair(bool, take(7usize))(input)?;
        len |= (len_part as u32) << 7;

        if parse_next {
          (input, (parse_next, len_part)) = pair(bool, take(7usize))(input)?;
          len |= (len_part as u32) << 14;

          if parse_next {
            (input, (parse_next, len_part)) = pair(bool, take(7usize))(input)?;
            len |= (len_part as u32) << 21;

            if parse_next {
              (input, len_part) = preceded(tag(0b00000, 5usize), take(3usize))(input)?;
              len |= (len_part as u32) << 28;
            }
          }
        }
      }

      Ok((input, len))
    };

    match parse_len((input, 0)) {
      Ok(((input, 0), len)) => Ok((input, len)),
      Ok(_) => Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Verify))),
      Err(err) => Err(err.map_input(|(input, _)| input)),
    }
  }

  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, Error<'i>> {
    Self::parse_len(input)
      .and_then(|(input, len)| map(map_res(take(len), str::from_utf8), Self)(input))
      .map_err(into_failure)
      .map_err(|err| err.map(|err| error_position!(err.input, ExpectedPrimitive(PrimitiveType::String))))
  }

  #[inline]
  pub fn as_str(&self) -> &'i str {
    self.0
  }
}

impl<'s> From<&'s str> for LengthPrefixedString<'s> {
  #[inline]
  fn from(s: &'s str) -> Self {
    Self(s)
  }
}

impl<'s> From<&'s String> for LengthPrefixedString<'s> {
  #[inline]
  fn from(s: &'s String) -> Self {
    Self(s.as_str())
  }
}

impl<'s> From<LengthPrefixedString<'s>> for &'s str {
  #[inline]
  fn from(val: LengthPrefixedString<'s>) -> Self {
    val.0
  }
}

#[cfg(test)]
mod tests {
  use super::LengthPrefixedString;

  #[test]
  fn length_127() {
    let string = "a".repeat(127);

    let mut input = vec![0b01111111];
    input.extend(string.as_bytes());

    assert_eq!(LengthPrefixedString::parse(&input), Ok(([].as_slice(), LengthPrefixedString::from(&string))));
  }
  #[test]
  fn length_16383() {
    let string = "a".repeat(16383);

    let mut input = vec![0b11111111, 0b01111111];
    input.extend(string.as_bytes());

    assert_eq!(LengthPrefixedString::parse(&input), Ok(([].as_slice(), LengthPrefixedString::from(&string))));
  }

  #[test]
  fn length_2097151() {
    let string = "a".repeat(2097151);

    let mut input = vec![0b11111111, 0b11111111, 0b01111111];
    input.extend(string.as_bytes());

    assert_eq!(LengthPrefixedString::parse(&input), Ok(([].as_slice(), LengthPrefixedString::from(&string))));
  }

  #[test]
  fn length_268435455() {
    let string = "a".repeat(268435455);

    let mut input = vec![0b11111111, 0b11111111, 0b11111111, 0b01111111];
    input.extend(string.as_bytes());

    assert_eq!(LengthPrefixedString::parse(&input), Ok(([].as_slice(), LengthPrefixedString::from(&string))));
  }

  #[ignore = "needs too much memory"]
  #[test]
  fn length_2147483647() {
    let string = "a".repeat(2147483647);

    let mut input = vec![0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b00000111];
    input.extend(string.as_bytes());

    assert_eq!(LengthPrefixedString::parse(&input), Ok(([].as_slice(), LengthPrefixedString::from(&string))));
  }
}
