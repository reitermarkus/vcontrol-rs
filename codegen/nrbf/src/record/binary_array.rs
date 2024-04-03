use nom::{
  combinator::{cond, fail, map},
  multi::many_m_n,
  IResult, Parser, ToUsize,
};

use crate::{
  common::AdditionalTypeInfo,
  data_type::Int32,
  enumeration::{BinaryArrayType, BinaryType},
  grammar::{MemberReference2, MemberReferenceInner},
  record::{BinaryObjectString, MemberPrimitiveUnTyped, RecordType},
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
  pub members: Vec<MemberReference2<'i>>,
}

impl<'i> BinaryArray<'i> {
  pub(crate) fn parse_member(
    input: &'i [u8],
    type_enum: BinaryType,
    additional_type_info: Option<&AdditionalTypeInfo<'i>>,
  ) -> IResult<&'i [u8], MemberReference2<'i>> {
    match (type_enum, additional_type_info) {
      (BinaryType::Primitive, Some(AdditionalTypeInfo::Primitive(primitive_type))) => map(
        |input| MemberPrimitiveUnTyped::parse(input, *primitive_type),
        |value| MemberReference2 {
          binary_library: None,
          member_reference: MemberReferenceInner::MemberPrimitiveUnTyped(value),
        },
      )(input),
      (BinaryType::String, None) => map(BinaryObjectString::parse, |value| MemberReference2 {
        binary_library: None,
        member_reference: MemberReferenceInner::BinaryObjectString(value),
      })(input),
      (BinaryType::Object, None) => MemberReference2::parse(input),
      (BinaryType::SystemClass, Some(_class_name)) => MemberReference2::parse(input),
      (BinaryType::Class, Some(_class_type_info)) => MemberReference2::parse(input),
      (BinaryType::ObjectArray, None) => MemberReference2::parse(input),
      (BinaryType::StringArray, None) => MemberReference2::parse(input),
      (BinaryType::PrimitiveArray, Some(_additional_type_info)) => MemberReference2::parse(input),
      _ => unreachable!(),
    }
  }

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

    let member_count = match binary_array_type_enum {
      BinaryArrayType::Single | BinaryArrayType::SingleOffset => lengths.first().map(|n| i32::from(*n) as u32),
      BinaryArrayType::Rectangular | BinaryArrayType::RectangularOffset => {
        lengths.iter().try_fold(1u32, |acc, n| acc.checked_mul(i32::from(*n) as u32))
      },
      BinaryArrayType::Jagged | BinaryArrayType::JaggedOffset => lengths.first().map(|n| i32::from(*n) as u32),
    };
    let member_count = match member_count {
      Some(member_count) => member_count.to_usize(),
      None => return fail(input),
    };
    let (input, members) = many_m_n(member_count, member_count, |input| {
      Self::parse_member(input, type_enum, additional_type_info.as_ref())
    })(input)?;

    Ok((
      input,
      Self { object_id, binary_array_type_enum, rank, lengths, lower_bounds, type_enum, additional_type_info, members },
    ))
  }

  #[inline]
  pub(crate) fn object_id(&self) -> Int32 {
    self.object_id
  }
}
