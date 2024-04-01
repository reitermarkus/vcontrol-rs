use std::collections::BTreeMap;

use nom::{
  combinator::{opt, verify},
  IResult,
};

use crate::{
  data_type::{Int32, LengthPrefixedString},
  grammar::Class,
  record::BinaryLibrary,
};

#[derive(Debug, Default)]
pub struct BinaryParser<'i> {
  pub binary_libraries: BTreeMap<Int32, LengthPrefixedString<'i>>,
  pub classes: BTreeMap<Int32, Class<'i>>,
}

impl<'i> BinaryParser<'i> {
  pub fn parse_binary_library(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    let (input, binary_library) = opt(verify(BinaryLibrary::parse, |binary_library| {
      !self.binary_libraries.contains_key(&binary_library.library_id())
    }))(input)?;

    if let Some(binary_library) = binary_library {
      self.binary_libraries.insert(binary_library.library_id(), binary_library.library_name);
    }

    Ok((input, ()))
  }

  pub fn parse_class(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    let (input, class) = verify(Class::parse, |class| !self.classes.contains_key(&class.object_id()))(input)?;

    self.classes.insert(class.object_id(), class);

    Ok((input, ()))
  }
}
