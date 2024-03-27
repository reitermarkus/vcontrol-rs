use nom::IResult;

use crate::{
  data_type::{ClassTypeInfo, LengthPrefixedString},
  enumeration::{BinaryType, PrimitiveType},
};

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
