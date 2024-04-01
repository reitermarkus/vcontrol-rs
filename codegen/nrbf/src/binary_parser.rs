use std::collections::BTreeMap;

use nom::{
  combinator::{opt, verify},
  IResult,
};

use crate::{
  data_type::{Int32, LengthPrefixedString},
  record::BinaryLibrary,
};

#[derive(Debug, Default)]
pub struct BinaryParser<'i> {
  pub binary_libraries: BTreeMap<Int32, LengthPrefixedString<'i>>,
}

impl<'i> BinaryParser<'i> {
  pub fn parse_binary_library(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    let (input, binary_library) = opt(verify(BinaryLibrary::parse, |binary_library| {
      !self.binary_libraries.contains_key(&binary_library.library_id)
    }))(input)?;

    if let Some(binary_library) = binary_library {
      self.binary_libraries.insert(binary_library.library_id, binary_library.library_name);
    }

    Ok((input, ()))
  }
}
