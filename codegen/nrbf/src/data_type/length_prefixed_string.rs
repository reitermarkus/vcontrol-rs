use std::str;

use nom::{
  bytes::complete::take,
  combinator::{map, map_res},
  number::complete::u8,
  sequence::{pair, preceded},
  IResult,
};

/// 2.1.1.6 `LengthPrefixedString`
#[derive(Debug, Clone, PartialEq)]
pub struct LengthPrefixedString<'s>(&'s str);

impl<'i> LengthPrefixedString<'i> {
  fn parse_len(mut input: (&[u8], usize)) -> IResult<(&[u8], usize), u32> {
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
  }

  pub fn parse(mut input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let ((input, _), len) = Self::parse_len((input, 0)).map_err(|err| err.map_input(|(input, _)| input))?;
    map(map_res(take(len), str::from_utf8), Self)(input)
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

impl<'s> Into<&'s str> for LengthPrefixedString<'s> {
  #[inline]
  fn into(self) -> &'s str {
    self.0
  }
}

#[cfg(test)]
mod tests {
  use std::iter::repeat;

  use super::LengthPrefixedString;

  #[test]
  fn length_127() {
    let string = repeat('a').take(127).collect::<String>();

    let mut input = vec![0b01111111];
    input.extend(string.as_bytes());

    assert_eq!(LengthPrefixedString::parse(&input), Ok(([].as_slice(), LengthPrefixedString::from(&string))));
  }
  #[test]
  fn length_16383() {
    let string = repeat('a').take(16383).collect::<String>();

    let mut input = vec![0b11111111, 0b01111111];
    input.extend(string.as_bytes());

    assert_eq!(LengthPrefixedString::parse(&input), Ok(([].as_slice(), LengthPrefixedString::from(&string))));
  }

  #[test]
  fn length_2097151() {
    let string = repeat('a').take(2097151).collect::<String>();

    let mut input = vec![0b11111111, 0b11111111, 0b01111111];
    input.extend(string.as_bytes());

    assert_eq!(LengthPrefixedString::parse(&input), Ok(([].as_slice(), LengthPrefixedString::from(&string))));
  }

  #[test]
  fn length_268435455() {
    let string = repeat('a').take(268435455).collect::<String>();

    let mut input = vec![0b11111111, 0b11111111, 0b11111111, 0b01111111];
    input.extend(string.as_bytes());

    assert_eq!(LengthPrefixedString::parse(&input), Ok(([].as_slice(), LengthPrefixedString::from(&string))));
  }

  #[ignore = "needs too much memory"]
  #[test]
  fn length_2147483647() {
    let string = repeat('a').take(2147483647).collect::<String>();

    let mut input = vec![0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b00000111];
    input.extend(string.as_bytes());

    assert_eq!(LengthPrefixedString::parse(&input), Ok(([].as_slice(), LengthPrefixedString::from(&string))));
  }
}
