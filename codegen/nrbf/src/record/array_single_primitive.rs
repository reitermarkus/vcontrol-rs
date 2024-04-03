use nom::{multi::many_m_n, IResult, Parser};
#[cfg(feature = "serde")]
use serde::ser::{Serialize, SerializeSeq, Serializer};

use crate::{
  common::ArrayInfo,
  data_type::Int32,
  enumeration::PrimitiveType,
  record::{MemberPrimitiveUnTyped, RecordType},
};

/// 2.4.3.3 `ArraySinglePrimitive`
#[derive(Debug, Clone, PartialEq)]
pub struct ArraySinglePrimitive {
  pub array_info: ArrayInfo,
  pub members: Vec<MemberPrimitiveUnTyped>,
}

impl ArraySinglePrimitive {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = RecordType::ArraySinglePrimitive.parse(input)?;

    let (input, array_info) = ArrayInfo::parse(input)?;
    let (input, primitive_type) = PrimitiveType::parse(input)?;
    let length = array_info.len();
    let (input, members) =
      many_m_n(length, length, |input| MemberPrimitiveUnTyped::parse(input, primitive_type))(input)?;

    Ok((input, Self { array_info, members }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.array_info.object_id()
  }
}

#[cfg(feature = "serde")]
impl Serialize for ArraySinglePrimitive {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut array = serializer.serialize_seq(Some(self.array_info.len()))?;
    for member in &self.members {
      array.serialize_element(member)?;
    }
    array.end()
  }
}
