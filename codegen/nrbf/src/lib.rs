use nom::{
  branch::alt,
  bytes::complete::tag,
  combinator::{cond, fail, map, opt, value},
  multi::{many0, many_m_n},
  number::complete::le_i32,
  IResult, Parser, ToUsize,
};

pub mod data_type;
use data_type::*;
pub mod enumeration;
use enumeration::*;
pub mod method_invocation;
use method_invocation::*;
mod record;
pub use record::*;
mod grammar;
pub use grammar::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ClassInfo<'i> {
  pub object_id: Int32,
  pub name: LengthPrefixedString<'i>,
  pub member_names: Vec<LengthPrefixedString<'i>>,
}

impl<'i> ClassInfo<'i> {
  pub fn parse(input: &'i [u8]) -> IResult<&'i [u8], Self> {
    let (input, object_id) = Int32::parse_positive(input)?;
    let (input, name) = LengthPrefixedString::parse(input)?;
    let (input, member_count) = Int32::parse_positive_or_zero(input)?;

    let member_count = (i32::from(member_count) as u32).to_usize();

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
