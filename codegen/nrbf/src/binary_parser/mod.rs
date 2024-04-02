use std::collections::{BTreeMap, HashMap};

use nom::{
  branch::alt,
  combinator::{fail, map, map_opt, opt, verify},
  multi::many_m_n,
  IResult, ToUsize,
};

use crate::{
  common::{AdditionalTypeInfo, MemberTypeInfo},
  data_type::{Int32, LengthPrefixedString},
  enumeration::{BinaryArrayType, BinaryType},
  record::{
    ArraySingleObject, ArraySinglePrimitive, ArraySingleString, BinaryArray, BinaryLibrary, BinaryMethodCall,
    BinaryMethodReturn, BinaryObjectString, ClassWithId, ClassWithMembers, ClassWithMembersAndTypes,
    MemberPrimitiveTyped, MemberPrimitiveUnTyped, MemberReference, ObjectNull, ObjectNullMultiple,
    ObjectNullMultiple256, SystemClassWithMembers, SystemClassWithMembersAndTypes,
  },
  value::Object,
  Value,
};

mod class;
use class::Class;

#[derive(Debug, Default)]
pub struct BinaryParser<'i> {
  pub binary_libraries: BTreeMap<Int32, LengthPrefixedString<'i>>,
  pub classes: BTreeMap<Int32, Class<'i>>,
  pub objects: BTreeMap<Int32, Value<'i>>,
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

  fn parse_binary_object_string(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    let (input, s) = BinaryObjectString::parse(input)?;

    self.objects.insert(s.object_id(), Value::String(s.as_str()));

    Ok((input, ()))
  }

  /// 2.7 Binary Record Grammar - `memberReference`
  pub fn parse_member_reference(
    &mut self,
    input: &'i [u8],
    type_enum_and_additional_type_info: Option<(BinaryType, Option<&AdditionalTypeInfo<'i>>)>,
  ) -> IResult<&'i [u8], Value<'i>> {
    let (input, ()) = self.parse_binary_library(input)?;

    let (input, (object_id, object)) =
      if let Some((type_enum, additional_type_info)) = type_enum_and_additional_type_info {
        match (type_enum, additional_type_info) {
          (BinaryType::Primitive, Some(AdditionalTypeInfo::Primitive(primitive_type))) => map(
            |input| MemberPrimitiveUnTyped::parse(input, *primitive_type),
            |primitive| (None, primitive.into_value()),
          )(input)?,
          (BinaryType::String, None) => {
            map(BinaryObjectString::parse, |s| (Some(s.object_id()), Value::String(s.as_str())))(input)?
          },
          (BinaryType::Object, None) => return self.parse_member_reference(input, None),
          (BinaryType::SystemClass, Some(class_name)) => unimplemented!("{class_name:?}"),
          (BinaryType::Class, Some(class_type_info)) => {
            unimplemented!("{class_type_info:?}")
          },
          (BinaryType::ObjectArray, None) => return self.parse_member_reference(input, None),
          (BinaryType::StringArray, None) => alt((
            map(BinaryObjectString::parse, |s| (Some(s.object_id()), Value::String(s.as_str()))),
            map(MemberReference::parse, |member_reference| (None, Value::Ref(member_reference.id_ref))),
            map(|input| Self::parse_null_object(input), |null_object| (None, null_object)),
          ))(input)?,
          (BinaryType::PrimitiveArray, Some(AdditionalTypeInfo::Primitive(_primitive_type))) => {
            map(MemberReference::parse, |member_reference| (None, Value::Ref(member_reference.id_ref)))(input)?
          },
          _ => unreachable!(),
        }
      } else {
        alt((
          map(MemberPrimitiveTyped::parse, |primitive| (None, primitive.into_value())),
          map(MemberReference::parse, |member_reference| (None, Value::Ref(member_reference.id_ref))),
          map(BinaryObjectString::parse, |s| (Some(s.object_id()), Value::String(s.as_str()))),
          map(|input| Self::parse_null_object(input), |null_object| (None, null_object)),
          map(|input| self.parse_classes(input), |o| (None, o)),
        ))(input)?
      };

    let object = if let Some(object_id) = object_id {
      self.objects.insert(object_id, object);
      Value::Ref(object_id)
    } else {
      object
    };

    Ok((input, object))
  }

  fn parse_members_with_type_info(
    &mut self,
    mut input: &'i [u8],
    member_type_info: &MemberTypeInfo<'i>,
  ) -> IResult<&'i [u8], Vec<Value<'i>>> {
    let mut member_references = vec![];

    for (binary_type_enum, additional_info) in
      member_type_info.binary_type_enums.iter().zip(member_type_info.additional_infos.iter())
    {
      let member;
      (input, member) = self.parse_member_reference(input, Some((*binary_type_enum, additional_info.as_ref())))?;
      member_references.push(member);
    }

    Ok((input, member_references))
  }

  /// 2.7 Binary Record Grammar - `Classes`
  pub fn parse_classes(&mut self, input: &'i [u8]) -> IResult<&'i [u8], Value<'i>> {
    let (input, ()) = self.parse_binary_library(input)?;

    let (input, (object_id, class)) = verify(
      alt((
        map_opt(ClassWithId::parse, |class| {
          let object_id = class.object_id();
          self.classes.get(&class.metadata_id()).map(|class| (object_id, class.clone()))
        }),
        map(verify(ClassWithMembers::parse, |class| self.binary_libraries.contains_key(&class.library_id)), |class| {
          (class.object_id(), Class::ClassWithMembers(class))
        }),
        map(
          verify(ClassWithMembersAndTypes::parse, |class| self.binary_libraries.contains_key(&class.library_id)),
          |class| (class.object_id(), Class::ClassWithMembersAndTypes(class)),
        ),
        map(SystemClassWithMembers::parse, |class| (class.object_id(), Class::SystemClassWithMembers(class))),
        map(SystemClassWithMembersAndTypes::parse, |class| {
          (class.object_id(), Class::SystemClassWithMembersAndTypes(class))
        }),
      )),
      |(object_id, _)| !self.classes.contains_key(&object_id),
    )(input)?;

    let (input, (class_name, library, member_references)) = match class {
      Class::ClassWithMembers(ref class) => {
        let class_name = class.class_info().name.as_str();
        let library = self.binary_libraries[&class.library_id].as_str();

        let member_count = class.class_info().member_names.len();
        let (input, member_references) =
          many_m_n(member_count, member_count, |input| self.parse_member_reference(input, None))(input)?;

        (input, (class_name, Some(library), member_references))
      },
      Class::ClassWithMembersAndTypes(ref class) => {
        let class_name = class.class_info().name.as_str();
        let library = self.binary_libraries[&class.library_id].as_str();

        let (input, member_references) = self.parse_members_with_type_info(input, &class.member_type_info)?;

        (input, (class_name, Some(library), member_references))
      },
      Class::SystemClassWithMembers(ref class) => {
        let class_name = class.class_info().name.as_str();

        let member_count = class.class_info().member_names.len();
        let (input, member_references) =
          many_m_n(member_count, member_count, |input| self.parse_member_reference(input, None))(input)?;

        (input, (class_name, None, member_references))
      },
      Class::SystemClassWithMembersAndTypes(ref class) => {
        let class_name = class.class_info().name.as_str();

        let (input, member_references) = self.parse_members_with_type_info(input, &class.member_type_info)?;

        (input, (class_name, None, member_references))
      },
    };

    let class_info = class.class_info();
    let object = Value::Object(Object {
      class: class_name,
      library,
      members: HashMap::from_iter(
        class_info
          .member_names
          .iter()
          .zip(member_references.iter().cloned())
          .map(|(member_name, member_value)| (member_name.as_str(), member_value)),
      ),
    });

    self.classes.insert(object_id, class);
    self.objects.insert(object_id, object);

    Ok((input, Value::Ref(object_id)))
  }

  fn parse_array_single_object(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    let (mut input, array) = ArraySingleObject::parse(input)?;

    let mut members = vec![];

    let mut len_remaining = array.array_info.len();
    while len_remaining > 0 {
      let member;
      (input, member) = self.parse_member_reference(input, None)?;

      let count = match member {
        Value::Null(count) => count,
        _ => 1,
      };

      members.push(member);
      len_remaining -= count;
    }

    self.objects.insert(array.object_id(), Value::Array(members));

    Ok((input, ()))
  }

  fn parse_array_single_primitive(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    let (input, array) = ArraySinglePrimitive::parse(input)?;

    let length = array.array_info.len();
    let (input, members) = many_m_n(
      length,
      length,
      map(|input| MemberPrimitiveUnTyped::parse(input, array.primitive_type), |primitive| primitive.into_value()),
    )(input)?;

    self.objects.insert(array.object_id(), Value::Array(members));

    Ok((input, ()))
  }

  fn parse_array_single_string(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    let (mut input, array) = ArraySingleString::parse(input)?;

    let mut members = vec![];

    let mut len_remaining = array.array_info.len();
    while len_remaining > 0 {
      let member;
      (input, member) = self.parse_member_reference(input, Some((BinaryType::StringArray, None)))?;

      let count = match member {
        Value::Null(count) => count,
        _ => 1,
      };

      members.push(member);
      len_remaining -= count;
    }

    self.objects.insert(array.object_id(), Value::Array(members));

    Ok((input, ()))
  }

  fn parse_binary_array(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    let (input, array) = BinaryArray::parse(input)?;

    let member_count = match array.binary_array_type_enum {
      BinaryArrayType::Single | BinaryArrayType::SingleOffset => array.lengths.first().map(|n| i32::from(*n) as u32),
      BinaryArrayType::Rectangular | BinaryArrayType::RectangularOffset => {
        array.lengths.iter().try_fold(1u32, |acc, n| acc.checked_mul(i32::from(*n) as u32))
      },
      BinaryArrayType::Jagged | BinaryArrayType::JaggedOffset => array.lengths.first().map(|n| i32::from(*n) as u32),
    };
    let member_count = match member_count {
      Some(member_count) => member_count.to_usize(),
      None => return fail(input),
    };
    let (input, members) = many_m_n(member_count, member_count, |input| {
      self.parse_member_reference(input, Some((array.type_enum, array.additional_type_info.as_ref())))
    })(input)?;

    self.objects.insert(array.object_id(), Value::Array(members));

    Ok((input, ()))
  }

  /// 2.7 Binary Record Grammar - `Arrays`
  pub fn parse_arrays(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    let (input, ()) = self.parse_binary_library(input)?;

    if let Ok((input, ())) = self.parse_array_single_object(input) {
      return Ok((input, ()))
    }

    if let Ok((input, ())) = self.parse_array_single_primitive(input) {
      return Ok((input, ()))
    }

    if let Ok((input, ())) = self.parse_array_single_string(input) {
      return Ok((input, ()))
    }

    self.parse_binary_array(input)
  }

  /// 2.7 Binary Record Grammar - `referenceable`
  pub fn parse_referenceable(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    if let Ok((input, _)) = self.parse_classes(input) {
      return Ok((input, ()))
    }

    if let Ok((input, ())) = self.parse_arrays(input) {
      return Ok((input, ()))
    }

    self.parse_binary_object_string(input)
  }

  /// 2.7 Binary Record Grammar - `nullObject`
  pub fn parse_null_object(input: &'i [u8]) -> IResult<&'i [u8], Value<'i>> {
    alt((
      map(ObjectNull::parse, |n| Value::Null(n.null_count())),
      map(ObjectNullMultiple::parse, |n| Value::Null(n.null_count())),
      map(ObjectNullMultiple256::parse, |n| Value::Null(n.null_count())),
    ))(input)
  }

  pub fn parse_call_array(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    let (input, ()) = self.parse_binary_library(input)?;

    self.parse_array_single_object(input)
  }

  /// 2.7 Binary Record Grammar - `methodCall`
  pub fn parse_method_call(&mut self, input: &'i [u8]) -> IResult<&'i [u8], BinaryMethodCall<'i>> {
    let (input, ()) = self.parse_binary_library(input)?;

    let (input, binary_method_return) = BinaryMethodCall::parse(input)?;

    let (input, _) = opt(|input| self.parse_call_array(input))(input)?;

    Ok((input, binary_method_return))
  }

  /// 2.7 Binary Record Grammar - `methodReturn`
  pub fn parse_method_return(&mut self, input: &'i [u8]) -> IResult<&'i [u8], BinaryMethodReturn<'i>> {
    let (input, ()) = self.parse_binary_library(input)?;

    let (input, binary_method_return) = BinaryMethodReturn::parse(input)?;

    let (input, _) = opt(|input| self.parse_call_array(input))(input)?;

    Ok((input, binary_method_return))
  }
}
