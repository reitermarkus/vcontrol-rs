use std::num::NonZeroU32;

use nom::{combinator::cond, multi::count, IResult};

use crate::{
  combinator::{length, object_id},
  common::AdditionalTypeInfo,
  enumeration::{BinaryArrayType, BinaryType},
  error::Error,
  record::RecordType,
};

/// 2.4.3.1 `BinaryArray`
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryArray<'i> {
  pub object_id: NonZeroU32,
  pub binary_array_type_enum: BinaryArrayType,
  pub lengths: Vec<usize>,
  pub lower_bounds: Option<Vec<usize>>,
  pub type_enum: BinaryType,
  pub additional_type_info: Option<AdditionalTypeInfo<'i>>,
}

impl<'i> BinaryArray<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self, Error<'_>> {
    let (input, _) = RecordType::BinaryArray.parse(input)?;

    let (input, object_id) = object_id(input)?;
    let (input, binary_array_type_enum) = BinaryArrayType::parse(input)?;
    let (input, rank) = length(input)?;

    let (input, lengths) = count(length, rank)(input)?;
    let (input, lower_bounds) = cond(
      matches!(
        binary_array_type_enum,
        BinaryArrayType::SingleOffset | BinaryArrayType::JaggedOffset | BinaryArrayType::RectangularOffset
      ),
      count(length, rank),
    )(input)?;
    let (input, type_enum) = BinaryType::parse(input)?;
    let (input, additional_type_info) = AdditionalTypeInfo::parse(input, type_enum)?;

    Ok((input, Self { object_id, binary_array_type_enum, lengths, lower_bounds, type_enum, additional_type_info }))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> NonZeroU32 {
    self.object_id
  }
}
