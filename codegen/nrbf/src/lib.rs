use std::str::{self, FromStr};

use nom::{
  branch::alt,
  bytes::complete::{tag, take},
  combinator::{cond, map, map_opt, map_res, opt, value, verify},
  complete::bool,
  multi::{many0, many_m_n},
  number::complete::{i8, le_f32, le_f64, le_i16, le_i32, le_i64, le_u16, le_u24, le_u32, le_u64, u8},
  sequence::{preceded, terminated},
  IResult,
};

mod binary_library;
pub use binary_library::BinaryLibrary;
pub mod data_type;
use data_type::*;
pub mod enumeration;
use enumeration::*;
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryTypeEnumeration {
  Primitive,
  String,
  Object,
  SystemClass,
  Class,
  ObjectArray,
  StringArray,
  PrimitiveArray,
}

impl BinaryTypeEnumeration {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    alt((
      value(Self::Primitive, tag([0])),
      value(Self::String, tag([1])),
      value(Self::Object, tag([2])),
      value(Self::SystemClass, tag([3])),
      value(Self::Class, tag([4])),
      value(Self::ObjectArray, tag([5])),
      value(Self::StringArray, tag([6])),
      value(Self::PrimitiveArray, tag([7])),
    ))(input)
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrimitiveTypeEnumeration {
  Boolean,
  Byte,
  Char,
  Decimal,
  Double,
  Int16,
  Int32,
  Int64,
  SByte,
  Single,
  TimeSpan,
  DateTime,
  UInt16,
  UInt32,
  UInt64,
}

impl PrimitiveTypeEnumeration {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    alt((
      value(Self::Boolean, tag([1])),
      value(Self::Byte, tag([2])),
      value(Self::Char, tag([3])),
      value(Self::Decimal, tag([5])),
      value(Self::Double, tag([6])),
      value(Self::Int16, tag([7])),
      value(Self::Int32, tag([8])),
      value(Self::Int64, tag([9])),
      value(Self::SByte, tag([10])),
      value(Self::Single, tag([11])),
      value(Self::TimeSpan, tag([12])),
      value(Self::DateTime, tag([13])),
      value(Self::UInt16, tag([14])),
      value(Self::UInt32, tag([15])),
      value(Self::UInt64, tag([16])),
    ))(input)
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

    let (input, primitive_type) = PrimitiveTypeEnumeration::parse(input)?;
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
  pub fn parse(input: &[u8], primitive_type: PrimitiveTypeEnumeration) -> IResult<&[u8], Self> {
    match primitive_type {
      PrimitiveTypeEnumeration::Boolean => map(Boolean::parse, Self::Boolean)(input),
      PrimitiveTypeEnumeration::Byte => map(Byte::parse, Self::Byte)(input),
      PrimitiveTypeEnumeration::Char => map(Char::parse, Self::Char)(input),
      PrimitiveTypeEnumeration::Decimal => map(Decimal::parse, Self::Decimal)(input),
      PrimitiveTypeEnumeration::Double => map(Double::parse, Self::Double)(input),
      PrimitiveTypeEnumeration::Int16 => map(Int16::parse, Self::Int16)(input),
      PrimitiveTypeEnumeration::Int32 => map(Int32::parse, Self::Int32)(input),
      PrimitiveTypeEnumeration::Int64 => map(Int64::parse, Self::Int64)(input),
      PrimitiveTypeEnumeration::SByte => map(Int8::parse, Self::SByte)(input),
      PrimitiveTypeEnumeration::Single => map(Single::parse, Self::Single)(input),
      PrimitiveTypeEnumeration::TimeSpan => map(TimeSpan::parse, Self::TimeSpan)(input),
      PrimitiveTypeEnumeration::DateTime => map(DateTime::parse, Self::DateTime)(input),
      PrimitiveTypeEnumeration::UInt16 => map(UInt16::parse, Self::UInt16)(input),
      PrimitiveTypeEnumeration::UInt32 => map(UInt32::parse, Self::UInt32)(input),
      PrimitiveTypeEnumeration::UInt64 => map(UInt64::parse, Self::UInt64)(input),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueWithCode {
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
  Null,
}

impl ValueWithCode {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    alt((
      map(MemberPrimitiveTyped::parse, |primitive_typed| match primitive_typed {
        MemberPrimitiveTyped::Boolean(v) => Self::Boolean(v),
        MemberPrimitiveTyped::Byte(v) => Self::Byte(v),
        MemberPrimitiveTyped::Char(v) => Self::Char(v),
        MemberPrimitiveTyped::Decimal(v) => Self::Decimal(v),
        MemberPrimitiveTyped::Double(v) => Self::Double(v),
        MemberPrimitiveTyped::Int16(v) => Self::Int16(v),
        MemberPrimitiveTyped::Int32(v) => Self::Int32(v),
        MemberPrimitiveTyped::Int64(v) => Self::Int64(v),
        MemberPrimitiveTyped::SByte(v) => Self::SByte(v),
        MemberPrimitiveTyped::Single(v) => Self::Single(v),
        MemberPrimitiveTyped::TimeSpan(v) => Self::TimeSpan(v),
        MemberPrimitiveTyped::DateTime(v) => Self::DateTime(v),
        MemberPrimitiveTyped::UInt16(v) => Self::UInt16(v),
        MemberPrimitiveTyped::UInt32(v) => Self::UInt32(v),
        MemberPrimitiveTyped::UInt64(v) => Self::UInt64(v),
      }),
      value(Self::Null, tag([17])),
    ))(input)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringValueWithCode<'i> {
  pub string_value: LengthPrefixedString<'i>,
}

impl<'i> StringValueWithCode<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, string_value) = preceded(tag([18]), LengthPrefixedString::parse)(input)?;

    Ok((input, Self { string_value }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayOfValueWithCode {
  pub list_of_value_with_code: Vec<ValueWithCode>,
}

impl ArrayOfValueWithCode {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, length) = le_i32(input)?;
    let length = usize::try_from(length).unwrap();
    let (input, list_of_value_with_code) = many_m_n(length, length, ValueWithCode::parse)(input)?;

    Ok((input, Self { list_of_value_with_code }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessageFlags(pub i32);

impl MessageFlags {
  pub const NO_ARGS: i32 = 0x00000001;
  pub const ARGS_INLINE: i32 = 0x00000002;
  pub const ARGS_IS_ARRAY: i32 = 0x00000004;
  pub const ARGS_IN_ARRAY: i32 = 0x00000008;
  pub const NO_CONTEXT: i32 = 0x00000010;
  pub const CONTEXT_INLINE: i32 = 0x00000020;
  pub const CONTEXT_IN_ARRAY: i32 = 0x00000040;
  pub const METHOD_SIGNATURE_IN_ARRAY: i32 = 0x00000080;
  pub const PROPERTIES_IN_ARRAY: i32 = 0x00000100;
  pub const NO_RETURN_VALUE: i32 = 0x00000200;
  pub const RETURN_VALUE_VOID: i32 = 0x00000400;
  pub const RETURN_VALUE_INLINE: i32 = 0x00000800;
  pub const RETURN_VALUE_IN_ARRAY: i32 = 0x00001000;
  pub const EXCEPTION_IN_ARRAY: i32 = 0x00002000;
  pub const GENERIC_METHOD: i32 = 0x00008000;

  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    map(le_i32, Self)(input)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryMethodCall<'i> {
  pub message_enum: MessageFlags,
  pub method_name: StringValueWithCode<'i>,
  pub type_name: StringValueWithCode<'i>,
  pub call_context: Option<StringValueWithCode<'i>>,
  pub args: Option<ArrayOfValueWithCode>,
}

impl<'i> BinaryMethodCall<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = tag([21])(input)?;

    let (input, message_enum) = MessageFlags::parse(input)?;
    let (input, method_name) = StringValueWithCode::parse(input)?;
    let (input, type_name) = StringValueWithCode::parse(input)?;
    let (input, call_context) =
      cond((message_enum.0 & MessageFlags::CONTEXT_INLINE) != 0, StringValueWithCode::parse)(input)?;
    let (input, args) = cond((message_enum.0 & MessageFlags::ARGS_INLINE) != 0, ArrayOfValueWithCode::parse)(input)?;

    Ok((input, Self { message_enum, method_name, type_name, call_context, args }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryMethodReturn<'i> {
  _todo: &'i (),
}

impl<'i> BinaryMethodReturn<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = tag([22])(input)?;

    Ok((input, todo!("BinaryMethodReturn::parse")))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AdditionalTypeInfo<'i> {
  Primitive(PrimitiveTypeEnumeration),
  SystemClass(LengthPrefixedString<'i>),
  Class(ClassTypeInfo<'i>),
}

impl<'i> AdditionalTypeInfo<'i> {
  pub fn parse_many(
    mut input: &'i [u8],
    binary_type_enums: &[BinaryTypeEnumeration],
  ) -> IResult<&'i [u8], Vec<Option<Self>>> {
    let mut additional_infos = vec![];

    for binary_type_enum in binary_type_enums {
      let additional_info;
      (input, additional_info) = Self::parse(input, *binary_type_enum)?;
      additional_infos.push(additional_info);
    }

    Ok((input, additional_infos))
  }

  pub fn parse(mut input: &'i [u8], binary_type_enum: BinaryTypeEnumeration) -> IResult<&'i [u8], Option<Self>> {
    let additional_info = match binary_type_enum {
      BinaryTypeEnumeration::Primitive => {
        let primitive_type;
        (input, primitive_type) = PrimitiveTypeEnumeration::parse(input)?;
        Some(Self::Primitive(primitive_type))
      },
      BinaryTypeEnumeration::String => None,
      BinaryTypeEnumeration::Object => None,
      BinaryTypeEnumeration::SystemClass => {
        let class_name;
        (input, class_name) = LengthPrefixedString::parse(input)?;
        Some(Self::SystemClass(class_name))
      },
      BinaryTypeEnumeration::Class => {
        let class_type_info;
        (input, class_type_info) = ClassTypeInfo::parse(input)?;
        Some(Self::Class(class_type_info))
      },
      BinaryTypeEnumeration::ObjectArray => None,
      BinaryTypeEnumeration::StringArray => None,
      BinaryTypeEnumeration::PrimitiveArray => {
        let primitive_type;
        (input, primitive_type) = PrimitiveTypeEnumeration::parse(input)?;
        Some(Self::Primitive(primitive_type))
      },
    };

    Ok((input, additional_info))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberTypeInfo<'i> {
  pub binary_type_enums: Vec<BinaryTypeEnumeration>,
  pub additional_infos: Vec<Option<AdditionalTypeInfo<'i>>>,
}

impl<'i> MemberTypeInfo<'i> {
  pub fn parse(input: &'i [u8], class_info: &ClassInfo<'_>) -> IResult<&'i [u8], Self> {
    let count = class_info.member_names.len();

    let (input, binary_type_enums) = many_m_n(count, count, BinaryTypeEnumeration::parse)(input)?;
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
  pub id_ref: i32,
}

impl MemberReference {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = tag([9])(input)?;

    let (input, id_ref) = le_i32(input)?;

    Ok((input, Self { id_ref }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayInfo {
  pub object_id: i32,
  pub length: i32,
}

impl ArrayInfo {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, object_id) = verify(le_i32, |&n| n > 0)(input)?;
    let (input, length) = verify(le_i32, |&n| n >= 0)(input)?;

    Ok((input, Self { object_id, length }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArraySingleObject {
  pub array_info: ArrayInfo,
}

impl ArraySingleObject {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = tag([16])(input)?;

    let (input, array_info) = ArrayInfo::parse(input)?;

    Ok((input, Self { array_info }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArraySinglePrimitive {
  pub array_info: ArrayInfo,
  pub primitive_type_enum: PrimitiveTypeEnumeration,
}

impl ArraySinglePrimitive {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = tag([15])(input)?;

    let (input, array_info) = ArrayInfo::parse(input)?;
    let (input, primitive_type_enum) = PrimitiveTypeEnumeration::parse(input)?;

    Ok((input, Self { array_info, primitive_type_enum }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArraySingleString {
  pub array_info: ArrayInfo,
}

impl ArraySingleString {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
    let (input, _) = tag([17])(input)?;

    let (input, array_info) = ArrayInfo::parse(input)?;

    Ok((input, Self { array_info }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryArrayTypeEnumeration {
  Single,
  Jagged,
  Rectangular,
  SingleOffset,
  JaggedOffset,
  RectangularOffset,
}

impl BinaryArrayTypeEnumeration {
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
  pub object_id: i32,
  pub binary_array_type_enum: BinaryArrayTypeEnumeration,
  pub rank: i32,
  pub lengths: Vec<i32>,
  pub lower_bounds: Option<Vec<i32>>,
  pub type_enum: BinaryTypeEnumeration,
  pub additional_type_info: Option<AdditionalTypeInfo<'i>>,
}

impl<'i> BinaryArray<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, _) = tag([7])(input)?;

    let (input, object_id) = verify(le_i32, |&n| n > 0)(input)?;
    let (input, binary_array_type_enum) = BinaryArrayTypeEnumeration::parse(input)?;
    let (input, rank) = verify(le_i32, |&n| n >= 0)(input)?;
    let rank_usize = usize::try_from(rank).unwrap();

    let (input, lengths) = many_m_n(rank_usize, rank_usize, verify(le_i32, |&n| n >= 0))(input)?;

    let (input, lower_bounds) = cond(
      matches!(
        binary_array_type_enum,
        BinaryArrayTypeEnumeration::SingleOffset
          | BinaryArrayTypeEnumeration::JaggedOffset
          | BinaryArrayTypeEnumeration::RectangularOffset
      ),
      many_m_n(rank_usize, rank_usize, verify(le_i32, |&n| n >= 0)),
    )(input)?;

    let (input, type_enum) = BinaryTypeEnumeration::parse(input)?;
    let (input, additional_type_info) = AdditionalTypeInfo::parse(input, type_enum)?;

    Ok((
      input,
      Self { object_id, binary_array_type_enum, rank, lengths, lower_bounds, type_enum, additional_type_info },
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
pub enum Referenceable<'i> {
  Classes(Classes<'i>),
  Arrays(Option<BinaryLibrary<'i>>, Vec<Record<'i>>),
  BinaryObjectString(BinaryObjectString<'i>),
}

impl<'i> Referenceable<'i> {
  fn parse_arrays(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;

    let (input, array) = alt((
      map(ArraySingleObject::parse, |_| todo!("ArraySingleObject")),
      map(ArraySinglePrimitive::parse, |_| todo!("ArraySinglePrimitive")),
      map(ArraySingleString::parse, |_| todo!("ArraySingleString")),
      map(BinaryArray::parse, |_| todo!("BinaryArray")),
    ))(input)?;

    Ok((input, Self::Arrays(binary_library, array)))
  }

  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    alt((
      map(Classes::parse, Self::Classes),
      Self::parse_arrays,
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
    member_type_info: &MemberTypeInfo<'_>,
  ) -> IResult<&'i [u8], Vec<MemberReference2<'i>>> {
    let mut member_references = vec![];

    for (binary_type_enum, additional_info) in
      member_type_info.binary_type_enums.iter().zip(member_type_info.additional_infos.iter())
    {
      member_references.push(match (binary_type_enum, additional_info) {
        (BinaryTypeEnumeration::Primitive, Some(AdditionalTypeInfo::Primitive(primitive_type))) => {
          let value;
          (input, value) = MemberPrimitiveUnTyped::parse(input, *primitive_type)?;

          MemberReference2 { binary_library: None, member_reference: MemberReference3::MemberPrimitiveUnTyped(value) }
        },
        (BinaryTypeEnumeration::String, None) => {
          let value;
          (input, value) = BinaryObjectString::parse(input)?;
          MemberReference2 { binary_library: None, member_reference: MemberReference3::BinaryObjectString(value) }
        },
        (BinaryTypeEnumeration::Object, None) => todo!("Object reference"),
        (BinaryTypeEnumeration::SystemClass, Some(class_name)) => todo!("SystemClass reference"),
        (BinaryTypeEnumeration::Class, Some(class_type_info)) => todo!("Class reference"),
        (BinaryTypeEnumeration::ObjectArray, None) => todo!("ObjectArray reference"),
        (BinaryTypeEnumeration::StringArray, None) => todo!("StringArray reference"),
        (BinaryTypeEnumeration::PrimitiveArray, Some(AdditionalTypeInfo::Primitive(primitive_type))) => {
          todo!("PrimitiveArray reference")
        },
        _ => unreachable!(),
      });
    }

    Ok((input, member_references))
  }

  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;

    let (mut input, class) = alt((
      map(ClassWithId::parse, Class::ClassWithId),
      map(ClassWithMembers::parse, Class::ClassWithMembers),
      map(ClassWithMembersAndTypes::parse, Class::ClassWithMembersAndTypes),
      map(SystemClassWithMembers::parse, Class::SystemClassWithMembers),
      map(SystemClassWithMembersAndTypes::parse, Class::SystemClassWithMembersAndTypes),
    ))(input)?;

    let member_references = match class {
      Class::ClassWithId(ref class) => todo!("ClassWithId"),
      Class::ClassWithMembers(ref class) => todo!("ClassWithMembers"),
      Class::ClassWithMembersAndTypes(ref class) => {
        let member_references;
        (input, member_references) = Self::parse_member_references(input, &class.member_type_info)?;
        member_references
      },
      Class::SystemClassWithMembers(ref class) => todo!("SystemClassWithMembers"),
      Class::SystemClassWithMembersAndTypes(ref class) => {
        let member_references;
        (input, member_references) = Self::parse_member_references(input, &class.member_type_info)?;
        member_references
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
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;

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
  pub call_array: ArraySingleObject,
  pub member_references: Vec<MemberReference2<'i>>,
}

impl<'i> CallArray<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;
    let (input, call_array) = ArraySingleObject::parse(input)?;
    dbg!(&call_array);
    let length = call_array.array_info.length as usize;
    dbg!(input);
    let (input, member_references) = many_m_n(length, length, MemberReference2::parse)(input)?;
    dbg!(&member_references);

    Ok((input, Self { binary_library, call_array, member_references }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Record<'i> {
  SerializationHeader(SerializationHeader),
  BinaryLibrary(BinaryLibrary<'i>),
  BinaryMethodReturn(BinaryMethodReturn<'i>),
  BinaryMethodCall(BinaryMethodCall<'i>, CallArray<'i>),
  MemberPrimitiveUnTyped(MemberPrimitiveUnTyped),
  MemberPrimitiveTyped(MemberPrimitiveTyped),
  BinaryObjectString(BinaryObjectString<'i>),
  MessageEnd(MessageEnd),
  Referenceable(Referenceable<'i>),
}

impl<'i> Record<'i> {
  fn parse_method_call(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;
    let (input, binary_method_call) = BinaryMethodCall::parse(input)?;
    dbg!(&binary_method_call);
    let (input, call_array) = CallArray::parse(input)?;
    dbg!(&call_array);

    Ok((input, Self::BinaryMethodCall(binary_method_call, call_array)))
  }

  fn parse_method_call_or_return(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    alt((Self::parse_method_call, map(BinaryMethodReturn::parse, Self::BinaryMethodReturn)))(input)
  }

  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Vec<Self>> {
    let (input, header) = SerializationHeader::parse(input)?;

    terminated(
      many0(alt((
        map(BinaryLibrary::parse, Self::BinaryLibrary),
        map(Referenceable::parse, Self::Referenceable),
        Self::parse_method_call_or_return,
      ))),
      MessageEnd::parse,
    )(input)
  }
}

pub fn parse<'i>(mut input: &'i [u8]) -> IResult<&'i [u8], Vec<Record<'i>>> {
  Record::parse(input)
}
