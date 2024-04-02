use nom::{combinator::cond, multi::many_m_n, IResult, Parser, ToUsize};

use crate::{
  common::AdditionalTypeInfo,
  data_type::Int32,
  enumeration::{BinaryArrayType, BinaryType},
  record::RecordType,
};

/// 2.4.3.1 `BinaryArray`
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryArray<'i> {
  pub object_id: Int32,
  pub binary_array_type_enum: BinaryArrayType,
  pub rank: Int32,
  pub lengths: Vec<Int32>,
  pub lower_bounds: Option<Vec<Int32>>,
  pub type_enum: BinaryType,
  pub additional_type_info: Option<AdditionalTypeInfo<'i>>,
}

impl<'i> BinaryArray<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = RecordType::BinaryArray.parse(input)?;

    let (input, object_id) = Int32::parse_positive(input)?;
    let (input, binary_array_type_enum) = BinaryArrayType::parse(input)?;
    let (input, rank) = Int32::parse_positive_or_zero(input)?;

    let rank_usize = (i32::from(rank) as u32).to_usize();

    let (input, lengths) = many_m_n(rank_usize, rank_usize, Int32::parse_positive_or_zero)(input)?;
    let (input, lower_bounds) = cond(
      matches!(
        binary_array_type_enum,
        BinaryArrayType::SingleOffset | BinaryArrayType::JaggedOffset | BinaryArrayType::RectangularOffset
      ),
      many_m_n(rank_usize, rank_usize, Int32::parse_positive_or_zero),
    )(input)?;
    let (input, type_enum) = BinaryType::parse(input)?;
    let (input, additional_type_info) = AdditionalTypeInfo::parse(input, type_enum)?;

    Ok((
      input,
      Self { object_id, binary_array_type_enum, rank, lengths, lower_bounds, type_enum, additional_type_info },
    ))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.object_id
  }
}
