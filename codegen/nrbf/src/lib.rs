use nom::{
  branch::alt,
  bytes::complete::tag,
  combinator::{cond, fail, map, opt, value},
  multi::{many0, many_m_n},
  number::complete::le_i32,
  IResult, Parser, ToUsize,
};

mod binary_library;
pub use binary_library::BinaryLibrary;
pub mod data_type;
use data_type::*;
pub mod enumeration;
use enumeration::*;
pub mod method_invocation;
use method_invocation::*;
mod message_end;
pub use message_end::MessageEnd;
mod null_object;
pub use null_object::{NullObject, ObjectNull, ObjectNullMultiple, ObjectNullMultiple256};
mod serialization_header;
pub use serialization_header::SerializationHeader;

#[derive(Debug, Clone, PartialEq)]
pub struct ClassInfo<'i> {
  pub object_id: i32,
  pub name: LengthPrefixedString<'i>,
  pub member_names: Vec<LengthPrefixedString<'i>>,
}

impl<'i> ClassInfo<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, object_id) = le_i32(input)?;
    let (input, name) = LengthPrefixedString::parse(input)?;
    let (input, member_count) = le_i32(input)?;

    let member_count = usize::try_from(member_count).unwrap();

    let (input, member_names) = many_m_n(member_count, member_count, LengthPrefixedString::parse)(input)?;

    Ok((input, Self { object_id, name, member_names }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemberPrimitiveUnTyped {
  Boolean(Boolean),
  Byte(Byte),
  Char(Char),
  Decimal(Decimal),
  Double(Double),
  Int16(Int16),
  Int32(Int32),
  Int64(Int64),
  SByte(Int8),
  Single(Single),
  TimeSpan(TimeSpan),
  DateTime(DateTime),
  UInt16(UInt16),
  UInt32(UInt32),
  UInt64(UInt64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemberPrimitiveTyped {
  Boolean(Boolean),
  Byte(Byte),
  Char(Char),
  Decimal(Decimal),
  Double(Double),
  Int16(Int16),
  Int32(Int32),
  Int64(Int64),
  SByte(Int8),
  Single(Single),
  TimeSpan(TimeSpan),
  DateTime(DateTime),
  UInt16(UInt16),
  UInt32(UInt32),
  UInt64(UInt64),
}

impl MemberPrimitiveTyped {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = tag([8])(input)?;

    let (input, primitive_type) = PrimitiveType::parse(input)?;
    let (input, primitive_untyped) = MemberPrimitiveUnTyped::parse(input, primitive_type)?;

    let primitive_typed = match primitive_untyped {
      MemberPrimitiveUnTyped::Boolean(v) => Self::Boolean(v),
      MemberPrimitiveUnTyped::Byte(v) => Self::Byte(v),
      MemberPrimitiveUnTyped::Char(v) => Self::Char(v),
      MemberPrimitiveUnTyped::Decimal(v) => Self::Decimal(v),
      MemberPrimitiveUnTyped::Double(v) => Self::Double(v),
      MemberPrimitiveUnTyped::Int16(v) => Self::Int16(v),
      MemberPrimitiveUnTyped::Int32(v) => Self::Int32(v),
      MemberPrimitiveUnTyped::Int64(v) => Self::Int64(v),
      MemberPrimitiveUnTyped::SByte(v) => Self::SByte(v),
      MemberPrimitiveUnTyped::Single(v) => Self::Single(v),
      MemberPrimitiveUnTyped::TimeSpan(v) => Self::TimeSpan(v),
      MemberPrimitiveUnTyped::DateTime(v) => Self::DateTime(v),
      MemberPrimitiveUnTyped::UInt16(v) => Self::UInt16(v),
      MemberPrimitiveUnTyped::UInt32(v) => Self::UInt32(v),
      MemberPrimitiveUnTyped::UInt64(v) => Self::UInt64(v),
    };

    Ok((input, primitive_typed))
  }
}

impl MemberPrimitiveUnTyped {
  pub fn parse(input: &[u8], primitive_type: PrimitiveType) -> IResult<&[u8], Self> {
    match primitive_type {
      PrimitiveType::Boolean => map(Boolean::parse, Self::Boolean)(input),
      PrimitiveType::Byte => map(Byte::parse, Self::Byte)(input),
      PrimitiveType::Char => map(Char::parse, Self::Char)(input),
      PrimitiveType::Decimal => map(Decimal::parse, Self::Decimal)(input),
      PrimitiveType::Double => map(Double::parse, Self::Double)(input),
      PrimitiveType::Int16 => map(Int16::parse, Self::Int16)(input),
      PrimitiveType::Int32 => map(Int32::parse, Self::Int32)(input),
      PrimitiveType::Int64 => map(Int64::parse, Self::Int64)(input),
      PrimitiveType::SByte => map(Int8::parse, Self::SByte)(input),
      PrimitiveType::Single => map(Single::parse, Self::Single)(input),
      PrimitiveType::TimeSpan => map(TimeSpan::parse, Self::TimeSpan)(input),
      PrimitiveType::DateTime => map(DateTime::parse, Self::DateTime)(input),
      PrimitiveType::UInt16 => map(UInt16::parse, Self::UInt16)(input),
      PrimitiveType::UInt32 => map(UInt32::parse, Self::UInt32)(input),
      PrimitiveType::UInt64 => map(UInt64::parse, Self::UInt64)(input),
      PrimitiveType::Null => fail(input),
      PrimitiveType::String => fail(input),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AdditionalTypeInfo<'i> {
  Primitive(PrimitiveType),
  SystemClass(LengthPrefixedString<'i>),
  Class(ClassTypeInfo<'i>),
}

impl<'i> AdditionalTypeInfo<'i> {
  pub fn parse_many(mut input: &'i [u8], binary_type_enums: &[BinaryType]) -> IResult<&'i [u8], Vec<Option<Self>>> {
    let mut additional_infos = vec![];

    for binary_type_enum in binary_type_enums {
      let additional_info;
      (input, additional_info) = Self::parse(input, *binary_type_enum)?;
      additional_infos.push(additional_info);
    }

    Ok((input, additional_infos))
  }

  pub fn parse(mut input: &'i [u8], binary_type_enum: BinaryType) -> IResult<&'i [u8], Option<Self>> {
    let additional_info = match binary_type_enum {
      BinaryType::Primitive => {
        let primitive_type;
        (input, primitive_type) = PrimitiveType::parse(input)?;
        Some(Self::Primitive(primitive_type))
      },
      BinaryType::String => None,
      BinaryType::Object => None,
      BinaryType::SystemClass => {
        let class_name;
        (input, class_name) = LengthPrefixedString::parse(input)?;
        Some(Self::SystemClass(class_name))
      },
      BinaryType::Class => {
        let class_type_info;
        (input, class_type_info) = ClassTypeInfo::parse(input)?;
        Some(Self::Class(class_type_info))
      },
      BinaryType::ObjectArray => None,
      BinaryType::StringArray => None,
      BinaryType::PrimitiveArray => {
        let primitive_type;
        (input, primitive_type) = PrimitiveType::parse(input)?;
        Some(Self::Primitive(primitive_type))
      },
    };

    Ok((input, additional_info))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberTypeInfo<'i> {
  pub binary_type_enums: Vec<BinaryType>,
  pub additional_infos: Vec<Option<AdditionalTypeInfo<'i>>>,
}

impl<'i> MemberTypeInfo<'i> {
  pub fn parse(input: &'i [u8], class_info: &ClassInfo<'_>) -> IResult<&'i [u8], Self> {
    let count = class_info.member_names.len();

    let (input, binary_type_enums) = many_m_n(count, count, BinaryType::parse)(input)?;
    let (input, additional_infos) = AdditionalTypeInfo::parse_many(input, &binary_type_enums)?;

    Ok((input, Self { binary_type_enums, additional_infos }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassWithId {
  pub object_id: i32,
  pub metadata_id: i32,
}

impl ClassWithId {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = tag([1])(input)?;

    let (input, object_id) = le_i32(input)?;
    let (input, metadata_id) = le_i32(input)?;

    Ok((input, Self { object_id, metadata_id }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassWithMembers<'i> {
  pub class_info: ClassInfo<'i>,
  pub library_id: i32,
}

impl<'i> ClassWithMembers<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = tag([3])(input)?;

    let (input, class_info) = ClassInfo::parse(input)?;
    let (input, library_id) = le_i32(input)?;

    Ok((input, Self { class_info, library_id }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassWithMembersAndTypes<'i> {
  pub class_info: ClassInfo<'i>,
  pub member_type_info: MemberTypeInfo<'i>,
  pub library_id: i32,
}

impl<'i> ClassWithMembersAndTypes<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = tag([5])(input)?;

    let (input, class_info) = ClassInfo::parse(input)?;
    let (input, member_type_info) = MemberTypeInfo::parse(input, &class_info)?;
    let (input, library_id) = le_i32(input)?;

    Ok((input, Self { class_info, member_type_info, library_id }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SystemClassWithMembers<'i> {
  pub class_info: ClassInfo<'i>,
}

impl<'i> SystemClassWithMembers<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = tag([2])(input)?;

    let (input, class_info) = ClassInfo::parse(input)?;

    Ok((input, Self { class_info }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SystemClassWithMembersAndTypes<'i> {
  pub class_info: ClassInfo<'i>,
  pub member_type_info: MemberTypeInfo<'i>,
}

impl<'i> SystemClassWithMembersAndTypes<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = tag([4])(input)?;

    let (input, class_info) = ClassInfo::parse(input)?;
    let (input, member_type_info) = MemberTypeInfo::parse(input, &class_info)?;

    Ok((input, Self { class_info, member_type_info }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryObjectString<'s> {
  pub object_id: i32,
  pub value: LengthPrefixedString<'s>,
}

impl<'i> BinaryObjectString<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = tag([6])(input)?;

    let (input, object_id) = le_i32(input)?;
    let (input, value) = LengthPrefixedString::parse(input)?;

    Ok((input, Self { object_id, value }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberReference {
  pub id_ref: Int32,
}

impl MemberReference {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = RecordType::MemberReference.parse(input)?;

    let (input, id_ref) = Int32::parse_positive(input)?;

    Ok((input, Self { id_ref }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayInfo {
  pub object_id: Int32,
  pub length: Int32,
}

impl ArrayInfo {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, object_id) = Int32::parse_positive(input)?;
    let (input, length) = Int32::parse_positive_or_zero(input)?;

    Ok((input, Self { object_id, length }))
  }

  #[inline]
  pub(crate) fn len(&self) -> usize {
    (i32::from(self.length) as u32).to_usize()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArraySingleObject<'i> {
  pub array_info: ArrayInfo,
  pub member_references: Vec<MemberReference2<'i>>,
}

impl<'i> ArraySingleObject<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = RecordType::ArraySingleObject.parse(input)?;

    let (input, array_info) = ArrayInfo::parse(input)?;
    let length = array_info.len();
    let (input, member_references) = many_m_n(length, length, MemberReference2::parse)(input)?;

    Ok((input, Self { array_info, member_references }))
  }
}

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
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArraySingleString<'i> {
  pub array_info: ArrayInfo,
  pub members: Vec<MemberReference3<'i>>,
}

impl<'i> ArraySingleString<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = RecordType::ArraySingleString.parse(input)?;

    let (mut input, array_info) = ArrayInfo::parse(input)?;

    let mut members = vec![];

    let mut len_remaining = array_info.len();
    while len_remaining > 0 {
      let (member, count);
      (input, (member, count)) = alt((
        map(BinaryObjectString::parse, |s| (MemberReference3::BinaryObjectString(s), 1)),
        map(MemberReference::parse, |m| (MemberReference3::MemberReference(m), 1)),
        map(NullObject::parse, |null_object| {
          let null_count = match null_object {
            NullObject::ObjectNull(_) => 1,
            NullObject::ObjectNullMultiple(ref n) => n.null_count(),
            NullObject::ObjectNullMultiple256(ref n) => n.null_count(),
          };

          (MemberReference3::NullObject(null_object), null_count)
        }),
      ))(input)?;
      members.push(member);
      len_remaining -= count;
    }

    Ok((input, Self { array_info, members }))
  }
}

/// 2.4.1.1 `BinaryArrayTypeEnumeration`
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryArrayType {
  Single,
  Jagged,
  Rectangular,
  SingleOffset,
  JaggedOffset,
  RectangularOffset,
}

impl BinaryArrayType {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    alt((
      value(Self::Single, tag([0])),
      value(Self::Jagged, tag([1])),
      value(Self::Rectangular, tag([2])),
      value(Self::SingleOffset, tag([3])),
      value(Self::JaggedOffset, tag([4])),
      value(Self::RectangularOffset, tag([5])),
    ))(input)
  }
}

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
  pub(self) fn parse_member(
    input: &'i [u8],
    type_enum: BinaryType,
    additional_type_info: Option<&AdditionalTypeInfo<'i>>,
  ) -> IResult<&'i [u8], MemberReference2<'i>> {
    match (type_enum, additional_type_info) {
      (BinaryType::Primitive, Some(AdditionalTypeInfo::Primitive(primitive_type))) => map(
        |input| MemberPrimitiveUnTyped::parse(input, *primitive_type),
        |value| MemberReference2 {
          binary_library: None,
          member_reference: MemberReference3::MemberPrimitiveUnTyped(value),
        },
      )(input),
      (BinaryType::String, None) => map(BinaryObjectString::parse, |value| MemberReference2 {
        binary_library: None,
        member_reference: MemberReference3::BinaryObjectString(value),
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum Class<'i> {
  ClassWithId(ClassWithId),
  ClassWithMembers(ClassWithMembers<'i>),
  ClassWithMembersAndTypes(ClassWithMembersAndTypes<'i>),
  SystemClassWithMembers(SystemClassWithMembers<'i>),
  SystemClassWithMembersAndTypes(SystemClassWithMembersAndTypes<'i>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Array<'i> {
  ArraySingleObject(ArraySingleObject<'i>),
  ArraySinglePrimitive(ArraySinglePrimitive),
  ArraySingleString(ArraySingleString<'i>),
  BinaryArray(BinaryArray<'i>),
}

impl<'i> Array<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    alt((
      map(ArraySingleObject::parse, Self::ArraySingleObject),
      map(ArraySinglePrimitive::parse, Self::ArraySinglePrimitive),
      map(ArraySingleString::parse, Self::ArraySingleString),
      map(BinaryArray::parse, Self::BinaryArray),
    ))(input)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arrays<'i> {
  pub binary_library: Option<BinaryLibrary<'i>>,
  pub array: Array<'i>,
}

impl<'i> Arrays<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;
    let (input, array) = Array::parse(input)?;

    Ok((input, Self { binary_library, array }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Referenceable<'i> {
  Classes(Classes<'i>),
  Arrays(Arrays<'i>),
  BinaryObjectString(BinaryObjectString<'i>),
}

impl<'i> Referenceable<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    alt((
      map(Classes::parse, Self::Classes),
      map(Arrays::parse, Self::Arrays),
      map(BinaryObjectString::parse, Self::BinaryObjectString),
    ))(input)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Classes<'i> {
  pub binary_library: Option<BinaryLibrary<'i>>,
  pub class: Class<'i>,
  pub member_references: Vec<MemberReference2<'i>>,
}

impl<'i> Classes<'i> {
  fn parse_member_references(
    mut input: &'i [u8],
    member_type_info: &MemberTypeInfo<'i>,
  ) -> IResult<&'i [u8], Vec<MemberReference2<'i>>> {
    let mut member_references = vec![];

    for (binary_type_enum, additional_info) in
      member_type_info.binary_type_enums.iter().zip(member_type_info.additional_infos.iter())
    {
      let member;
      (input, member) = BinaryArray::parse_member(input, *binary_type_enum, additional_info.as_ref())?;
      member_references.push(member);
    }

    Ok((input, member_references))
  }

  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;

    let (input, class) = alt((
      map(ClassWithId::parse, Class::ClassWithId),
      map(ClassWithMembers::parse, Class::ClassWithMembers),
      map(ClassWithMembersAndTypes::parse, Class::ClassWithMembersAndTypes),
      map(SystemClassWithMembers::parse, Class::SystemClassWithMembers),
      map(SystemClassWithMembersAndTypes::parse, Class::SystemClassWithMembersAndTypes),
    ))(input)?;

    let (input, member_references) = match class {
      Class::ClassWithId(ref _class) => many0(MemberReference2::parse)(input)?,
      Class::ClassWithMembers(ref _class) => many0(MemberReference2::parse)(input)?,
      Class::ClassWithMembersAndTypes(ref class) => Self::parse_member_references(input, &class.member_type_info)?,
      Class::SystemClassWithMembers(ref _class) => many0(MemberReference2::parse)(input)?,
      Class::SystemClassWithMembersAndTypes(ref class) => {
        Self::parse_member_references(input, &class.member_type_info)?
      },
    };

    Ok((input, Self { binary_library, class, member_references }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemberReference3<'i> {
  MemberPrimitiveUnTyped(MemberPrimitiveUnTyped),
  MemberPrimitiveTyped(MemberPrimitiveTyped),
  MemberReference(MemberReference),
  BinaryObjectString(BinaryObjectString<'i>),
  NullObject(NullObject),
  Classes(Classes<'i>),
}

impl<'i> MemberReference3<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    alt((
      map(MemberPrimitiveTyped::parse, Self::MemberPrimitiveTyped),
      map(MemberReference::parse, Self::MemberReference),
      map(BinaryObjectString::parse, Self::BinaryObjectString),
      map(NullObject::parse, Self::NullObject),
      map(Classes::parse, Self::Classes),
    ))(input)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberReference2<'i> {
  pub binary_library: Option<BinaryLibrary<'i>>,
  pub member_reference: MemberReference3<'i>,
}

impl<'i> MemberReference2<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;
    let (input, member_reference) = MemberReference3::parse(input)?;

    Ok((input, Self { binary_library, member_reference }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArray<'i> {
  pub binary_library: Option<BinaryLibrary<'i>>,
  pub call_array: MethodCallArray<'i>,
}

impl<'i> CallArray<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;
    let (input, call_array) = MethodCallArray::parse(input)?;

    Ok((input, Self { binary_library, call_array }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodCall<'i> {
  pub binary_library: Option<BinaryLibrary<'i>>,
  pub binary_method_call: BinaryMethodCall<'i>,
  pub call_array: Option<CallArray<'i>>,
}

impl<'i> MethodCall<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;
    let (input, binary_method_call) = BinaryMethodCall::parse(input)?;
    let (input, call_array) = opt(CallArray::parse)(input)?;

    Ok((input, Self { binary_library, binary_method_call, call_array }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnCallArray<'i> {
  pub binary_library: Option<BinaryLibrary<'i>>,
  pub return_call_array: MethodReturnCallArray<'i>,
  pub member_references: Vec<MemberReference2<'i>>,
}

impl<'i> ReturnCallArray<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;
    let (input, return_call_array) = MethodReturnCallArray::parse(input)?;
    let length = return_call_array.0.array_info.len();
    let (input, member_references) = many_m_n(length, length, MemberReference2::parse)(input)?;

    Ok((input, Self { binary_library, return_call_array, member_references }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodReturn<'i> {
  pub binary_library: Option<BinaryLibrary<'i>>,
  pub binary_method_return: BinaryMethodReturn<'i>,
  pub return_call_array: Option<ReturnCallArray<'i>>,
}

impl<'i> MethodReturn<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;
    let (input, binary_method_return) = BinaryMethodReturn::parse(input)?;
    let (input, return_call_array) = opt(ReturnCallArray::parse)(input)?;

    Ok((input, Self { binary_library, binary_method_return, return_call_array }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Record<'i> {
  SerializationHeader(SerializationHeader),
  BinaryLibrary(BinaryLibrary<'i>),
  MethodReturn(MethodReturn<'i>),
  MethodCall(MethodCall<'i>),
  MemberPrimitiveUnTyped(MemberPrimitiveUnTyped),
  MemberPrimitiveTyped(MemberPrimitiveTyped),
  BinaryObjectString(BinaryObjectString<'i>),
  MessageEnd(MessageEnd),
  Referenceable(Referenceable<'i>),
}

impl<'i> Record<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Vec<Self>> {
    let (input, _) = SerializationHeader::parse(input)?;

    let (input, records) = many0(alt((
      map(Referenceable::parse, Self::Referenceable),
      alt((map(MethodCall::parse, Self::MethodCall), map(MethodReturn::parse, Self::MethodReturn))),
    )))(input)?;

    let (input, _) = MessageEnd::parse(input)?;

    Ok((input, records))
  }
}

pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<Record<'_>>> {
  Record::parse(input)
}
