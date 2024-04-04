use std::collections::{BTreeMap, HashMap};

use nom::{
  branch::alt,
  combinator::{fail, map, map_opt, opt, verify},
  multi::many_m_n,
  IResult, ToUsize,
};

use crate::{
  common::{AdditionalTypeInfo, MemberTypeInfo},
  data_type::LengthPrefixedString,
  enumeration::{BinaryArrayType, BinaryType},
  record::{
    ArraySingleObject, ArraySinglePrimitive, ArraySingleString, BinaryArray, BinaryLibrary, BinaryMethodCall,
    BinaryMethodReturn, BinaryObjectString, ClassWithId, ClassWithMembers, ClassWithMembersAndTypes,
    MemberPrimitiveTyped, MemberPrimitiveUnTyped, MemberReference, MessageEnd, MessageFlags, ObjectNull,
    ObjectNullMultiple, ObjectNullMultiple256, SerializationHeader, SystemClassWithMembers,
    SystemClassWithMembersAndTypes,
  },
  value::Object,
  MethodCall, MethodReturn, RemotingMessage, Value,
};

#[derive(Debug, Clone)]
enum ValueOrRef<'i> {
  Value(Value<'i>),
  Ref(RefId),
}

#[derive(Debug, Clone, Copy)]
pub struct RefId(i32);

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq)]
pub enum Class<'i> {
  ClassWithMembers(ClassWithMembers<'i>),
  ClassWithMembersAndTypes(ClassWithMembersAndTypes<'i>),
  SystemClassWithMembers(SystemClassWithMembers<'i>),
  SystemClassWithMembersAndTypes(SystemClassWithMembersAndTypes<'i>),
}

#[derive(Debug, Clone, PartialEq)]
enum MethodCallOrReturn<'i> {
  MethodCall(MethodCall<'i>),
  MethodReturn(MethodReturn<'i>),
}

#[derive(Debug, Default)]
pub struct BinaryParser<'i> {
  binary_libraries: BTreeMap<i32, LengthPrefixedString<'i>>,
  classes: BTreeMap<i32, Class<'i>>,
  objects: BTreeMap<i32, Value<'i>>,
}

impl<'i> BinaryParser<'i> {
  fn parse_binary_library(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    let (input, binary_library) = opt(verify(BinaryLibrary::parse, |binary_library| {
      !self.binary_libraries.contains_key(&binary_library.library_id().into())
    }))(input)?;

    if let Some(binary_library) = binary_library {
      self.binary_libraries.insert(binary_library.library_id().into(), binary_library.library_name);
    }

    Ok((input, ()))
  }

  /// 2.7 Binary Record Grammar - `memberReference`
  fn parse_member_reference(
    &mut self,
    input: &'i [u8],
    type_enum_and_additional_type_info: Option<(BinaryType, Option<&AdditionalTypeInfo<'i>>)>,
  ) -> IResult<&'i [u8], ValueOrRef<'i>> {
    let (input, ()) = self.parse_binary_library(input)?;

    let (input, object) = if let Some((type_enum, additional_type_info)) = type_enum_and_additional_type_info {
      match (type_enum, additional_type_info) {
        (BinaryType::Primitive, Some(AdditionalTypeInfo::Primitive(primitive_type))) => map(
          |input| MemberPrimitiveUnTyped::parse(input, *primitive_type),
          |primitive| ValueOrRef::Value(primitive.into_value()),
        )(input)?,
        (BinaryType::String, None) => {
          map(BinaryObjectString::parse, |s| ValueOrRef::Value(Value::String(s.as_str())))(input)?
        },
        (BinaryType::Object, None) => return self.parse_member_reference(input, None),
        (BinaryType::SystemClass, Some(class_name)) => unimplemented!("{class_name:?}"),
        (BinaryType::Class, Some(class_type_info)) => {
          unimplemented!("{class_type_info:?}")
        },
        (BinaryType::ObjectArray, None) => return self.parse_member_reference(input, None),
        (BinaryType::StringArray, None) => alt((
          map(BinaryObjectString::parse, |s| ValueOrRef::Value(Value::String(s.as_str()))),
          map(MemberReference::parse, |member_reference| ValueOrRef::Ref(RefId(member_reference.id_ref.into()))),
          map(Self::parse_null_object, |null_object| ValueOrRef::Value(null_object)),
        ))(input)?,
        (BinaryType::PrimitiveArray, Some(AdditionalTypeInfo::Primitive(_primitive_type))) => {
          map(MemberReference::parse, |member_reference| ValueOrRef::Ref(RefId(member_reference.id_ref.into())))(input)?
        },
        _ => unreachable!(),
      }
    } else {
      alt((
        map(MemberPrimitiveTyped::parse, |primitive| ValueOrRef::Value(primitive.into_value())),
        map(MemberReference::parse, |member_reference| ValueOrRef::Ref(RefId(member_reference.id_ref.into()))),
        map(BinaryObjectString::parse, |s| ValueOrRef::Value(Value::String(s.as_str()))),
        map(Self::parse_null_object, |null_object| ValueOrRef::Value(null_object)),
        map(|input| self.parse_classes(input), |(_, object)| ValueOrRef::Value(Value::Object(object))),
      ))(input)?
    };

    Ok((input, object))
  }

  fn parse_members_with_type_info(
    &mut self,
    mut input: &'i [u8],
    member_type_info: &MemberTypeInfo<'i>,
  ) -> IResult<&'i [u8], Vec<ValueOrRef<'i>>> {
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

  /// Resolves members from already parsed objects or by parsing missing members.
  fn resolve_members(
    &mut self,
    mut input: &'i [u8],
    members: Vec<ValueOrRef<'i>>,
  ) -> IResult<&'i [u8], Vec<Value<'i>>> {
    let mut members2 = Vec::with_capacity(members.len());

    for member in members.into_iter() {
      members2.push(match member {
        ValueOrRef::Value(value) => value,
        ValueOrRef::Ref(id) => {
          if let Some(value) = self.objects.remove(&id.0) {
            value.clone()
          } else {
            let member2;
            (input, member2) = verify(|input| self.parse_referenceable(input), |id2| id2.0 == id.0)(input)?;

            if let Some(value) = self.objects.remove(&member2.0) {
              value.clone()
            } else {
              return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify)))
            }
          }
        },
      })
    }

    Ok((input, members2))
  }

  /// 2.7 Binary Record Grammar - `Classes`
  fn parse_classes(&mut self, input: &'i [u8]) -> IResult<&'i [u8], (RefId, Object<'i>)> {
    let (input, ()) = self.parse_binary_library(input)?;

    let (input, (object_id, class)) = verify(
      alt((
        map_opt(ClassWithId::parse, |class| {
          let object_id = class.object_id().into();
          self.classes.get(&class.metadata_id().into()).map(|class| (object_id, class.clone()))
        }),
        map(
          verify(ClassWithMembers::parse, |class| self.binary_libraries.contains_key(&class.library_id.into())),
          |class| (class.object_id().into(), Class::ClassWithMembers(class)),
        ),
        map(
          verify(ClassWithMembersAndTypes::parse, |class| self.binary_libraries.contains_key(&class.library_id.into())),
          |class| (class.object_id().into(), Class::ClassWithMembersAndTypes(class)),
        ),
        map(SystemClassWithMembers::parse, |class| (class.object_id().into(), Class::SystemClassWithMembers(class))),
        map(SystemClassWithMembersAndTypes::parse, |class| {
          (class.object_id().into(), Class::SystemClassWithMembersAndTypes(class))
        }),
      )),
      |(object_id, _)| !self.classes.contains_key(object_id),
    )(input)?;

    let (input, (class_info, library, member_references)) = match class {
      Class::ClassWithMembers(ref class) => {
        let library = self.binary_libraries[&class.library_id.into()].as_str();

        let member_count = class.class_info().member_names.len();
        let (input, member_references) =
          many_m_n(member_count, member_count, |input| self.parse_member_reference(input, None))(input)?;

        (input, (class.class_info(), Some(library), member_references))
      },
      Class::ClassWithMembersAndTypes(ref class) => {
        let library = self.binary_libraries[&class.library_id.into()].as_str();

        let (input, member_references) = self.parse_members_with_type_info(input, &class.member_type_info)?;

        (input, (class.class_info(), Some(library), member_references))
      },
      Class::SystemClassWithMembers(ref class) => {
        let member_count = class.class_info().member_names.len();
        let (input, member_references) =
          many_m_n(member_count, member_count, |input| self.parse_member_reference(input, None))(input)?;

        (input, (class.class_info(), None, member_references))
      },
      Class::SystemClassWithMembersAndTypes(ref class) => {
        let (input, member_references) = self.parse_members_with_type_info(input, &class.member_type_info)?;

        (input, (class.class_info(), None, member_references))
      },
    };

    let (input, member_references) = self.resolve_members(input, member_references)?;

    let members = HashMap::from_iter(
      class_info
        .member_names
        .iter()
        .zip(member_references.into_iter())
        .map(|(member_name, member)| (member_name.as_str(), { member })),
    );

    let class_name = class_info.name.as_str();

    Ok((input, (RefId(object_id), Object { class: class_name, library, members })))
  }

  /// 2.7 Binary Record Grammar - `ArraySingleObject *(memberReference)`
  fn parse_array_single_object(&mut self, input: &'i [u8]) -> IResult<&'i [u8], (RefId, Vec<Value<'i>>)> {
    let (mut input, array) = ArraySingleObject::parse(input)?;

    let mut members = vec![];

    let mut len_remaining = array.array_info.len();
    while len_remaining > 0 {
      let member;
      (input, member) = self.parse_member_reference(input, None)?;

      let count = match member {
        ValueOrRef::Value(Value::Null(count)) => count,
        _ => 1,
      };

      members.push(member);
      len_remaining -= count;
    }

    let (input, members) = self.resolve_members(input, members)?;

    let object_id = array.object_id().into();
    Ok((input, (RefId(object_id), members)))
  }

  /// 2.7 Binary Record Grammar - `ArraySinglePrimitive *(MemberPrimitiveUnTyped)`
  fn parse_array_single_primitive(&mut self, input: &'i [u8]) -> IResult<&'i [u8], (RefId, Vec<Value<'i>>)> {
    let (input, array) = ArraySinglePrimitive::parse(input)?;

    let length = array.array_info.len();
    let (input, members) = many_m_n(
      length,
      length,
      map(|input| MemberPrimitiveUnTyped::parse(input, array.primitive_type), |primitive| primitive.into_value()),
    )(input)?;

    let object_id = array.object_id().into();
    Ok((input, (RefId(object_id), members)))
  }

  /// 2.7 Binary Record Grammar - `ArraySingleString *(BinaryObjectString/MemberReference/nullObject)`
  fn parse_array_single_string(&mut self, input: &'i [u8]) -> IResult<&'i [u8], (RefId, Vec<Value<'i>>)> {
    let (mut input, array) = ArraySingleString::parse(input)?;

    let mut members = vec![];

    let mut len_remaining = array.array_info.len();
    while len_remaining > 0 {
      let member;
      (input, member) = self.parse_member_reference(input, Some((BinaryType::StringArray, None)))?;

      let count = match member {
        ValueOrRef::Value(Value::Null(count)) => count,
        _ => 1,
      };

      members.push(member);
      len_remaining -= count;
    }

    let (input, members) = self.resolve_members(input, members)?;

    let object_id = array.object_id().into();
    Ok((input, (RefId(object_id), members)))
  }

  /// 2.7 Binary Record Grammar - `BinaryArray *(memberReference)`
  fn parse_binary_array(&mut self, input: &'i [u8]) -> IResult<&'i [u8], (RefId, Vec<Value<'i>>)> {
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

    let (input, members) = self.resolve_members(input, members)?;

    let object_id = array.object_id().into();
    Ok((input, (RefId(object_id), members)))
  }

  /// 2.7 Binary Record Grammar - `Arrays`
  fn parse_arrays(&mut self, input: &'i [u8]) -> IResult<&'i [u8], (RefId, Vec<Value<'i>>)> {
    let (input, ()) = self.parse_binary_library(input)?;

    if let Ok((input, (object_id, array))) = self.parse_array_single_object(input) {
      return Ok((input, (object_id, array)))
    }

    if let Ok((input, (object_id, array))) = self.parse_array_single_primitive(input) {
      return Ok((input, (object_id, array)))
    }

    if let Ok((input, (object_id, array))) = self.parse_array_single_string(input) {
      return Ok((input, (object_id, array)))
    }

    self.parse_binary_array(input)
  }

  /// 2.7 Binary Record Grammar - `referenceable`
  fn parse_referenceable(&mut self, input: &'i [u8]) -> IResult<&'i [u8], RefId> {
    if let Ok((input, (object_id, object))) = self.parse_classes(input) {
      self.objects.insert(object_id.0, Value::Object(object));
      return Ok((input, object_id))
    }

    if let Ok((input, (object_id, array))) = self.parse_arrays(input) {
      self.objects.insert(object_id.0, Value::Array(array));
      return Ok((input, object_id))
    }

    let (input, s) = BinaryObjectString::parse(input)?;

    let object_id = s.object_id().into();
    self.objects.insert(object_id, Value::String(s.as_str()));

    Ok((input, RefId(object_id)))
  }

  /// 2.7 Binary Record Grammar - `nullObject`
  fn parse_null_object(input: &'i [u8]) -> IResult<&'i [u8], Value<'i>> {
    alt((
      map(ObjectNull::parse, |n| Value::Null(n.null_count())),
      map(ObjectNullMultiple::parse, |n| Value::Null(n.null_count())),
      map(ObjectNullMultiple256::parse, |n| Value::Null(n.null_count())),
    ))(input)
  }

  fn parse_call_array(&mut self, input: &'i [u8]) -> IResult<&'i [u8], (RefId, Vec<Value<'i>>)> {
    let (input, ()) = self.parse_binary_library(input)?;

    self.parse_array_single_object(input)
  }

  /// 2.7 Binary Record Grammar - `methodCall`
  fn parse_method_call(&mut self, input: &'i [u8], root_id: i32) -> IResult<&'i [u8], MethodCall<'i>> {
    let (input, ()) = self.parse_binary_library(input)?;

    let (input, binary_method_call) = BinaryMethodCall::parse(input)?;

    let (input, call_array) = opt(|input| self.parse_call_array(input))(input)?;

    let args = if binary_method_call.message_enum.intersects(MessageFlags::ARGS_IS_ARRAY) {
      if let Some((call_array_id, call_array)) = call_array {
        if call_array_id.0 == root_id {
          Some(call_array)
        } else {
          return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify)))
        }
      } else {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify)))
      }
    } else if binary_method_call.message_enum.intersects(MessageFlags::ARGS_IN_ARRAY) {
      if let Some((call_array_id, mut call_array)) = call_array {
        if call_array_id.0 == root_id && !call_array.is_empty() {
          if let Value::Array(args) = call_array.remove(0) {
            Some(args)
          } else {
            return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify)))
          }
        } else {
          return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify)))
        }
      } else {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify)))
      }
    } else {
      binary_method_call.args.map(|v| v.into_values())
    };

    let method_call = MethodCall {
      method_name: binary_method_call.method_name.as_str(),
      type_name: binary_method_call.type_name.as_str(),
      call_context: binary_method_call.call_context.map(|c| c.as_str()),
      args,
    };

    Ok((input, method_call))
  }

  /// 2.7 Binary Record Grammar - `methodReturn`
  fn parse_method_return(&mut self, input: &'i [u8], root_id: i32) -> IResult<&'i [u8], MethodReturn<'i>> {
    let (input, ()) = self.parse_binary_library(input)?;

    let (input, binary_method_return) = BinaryMethodReturn::parse(input)?;

    let (input, call_array) = opt(|input| self.parse_call_array(input))(input)?;

    let args = if binary_method_return.message_enum.intersects(MessageFlags::ARGS_IS_ARRAY) {
      if let Some((call_array_id, call_array)) = call_array {
        if call_array_id.0 == root_id {
          Some(call_array.clone())
        } else {
          return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify)))
        }
      } else {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify)))
      }
    } else if binary_method_return.message_enum.intersects(MessageFlags::ARGS_IN_ARRAY) {
      if let Some((call_array_id, mut call_array)) = call_array {
        if call_array_id.0 == root_id && !call_array.is_empty() {
          if let Value::Array(args) = call_array.remove(0) {
            Some(args)
          } else {
            return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify)))
          }
        } else {
          return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify)))
        }
      } else {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify)))
      }
    } else {
      binary_method_return.args.map(|v| v.into_values())
    };

    let method_return = MethodReturn {
      return_value: binary_method_return.return_value.map(|v| v.into_value()),
      call_context: binary_method_return.call_context.map(|c| c.as_str()),
      args,
    };

    Ok((input, method_return))
  }

  /// 2.7 Binary Record Grammar - `(methodCall/methodReturn)`
  fn parse_method_call_or_return(
    &mut self,
    input: &'i [u8],
    root_id: i32,
  ) -> IResult<&'i [u8], MethodCallOrReturn<'i>> {
    if let Ok(s) = map(|input| self.parse_method_call(input, root_id), MethodCallOrReturn::MethodCall)(input) {
      return Ok(s)
    }

    if let Ok(s) = map(|input| self.parse_method_return(input, root_id), MethodCallOrReturn::MethodReturn)(input) {
      return Ok(s)
    }

    Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Alt)))
  }

  /// 2.7 Binary Record Grammar - `remotingMessage`
  fn parse_remoting_message(&mut self, input: &'i [u8]) -> IResult<&'i [u8], RemotingMessage<'i>> {
    let (mut input, header) = SerializationHeader::parse(input)?;

    while let Ok((input2, _)) = self.parse_referenceable(input) {
      input = input2;
    }

    let (mut input, method_call_or_return) =
      opt(|input| self.parse_method_call_or_return(input, header.root_id.into()))(input)?;

    while let Ok((input2, _)) = self.parse_referenceable(input) {
      input = input2;
    }

    let (input, MessageEnd) = MessageEnd::parse(input)?;

    let remoting_message = match method_call_or_return {
      Some(MethodCallOrReturn::MethodCall(method_call)) => RemotingMessage::MethodCall(method_call),
      Some(MethodCallOrReturn::MethodReturn(method_return)) => RemotingMessage::MethodReturn(method_return),
      None => {
        let root_object = if let Some(root_object) = self.objects.remove(&header.root_id.into()) {
          root_object
        } else {
          return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify)))
        };

        RemotingMessage::Value(root_object)
      },
    };

    Ok((input, remoting_message))
  }

  /// Deserializes a [`RemotingMessage`] from bytes.
  pub fn deserialize(mut self, input: &'i [u8]) -> IResult<&'i [u8], RemotingMessage<'i>> {
    self.parse_remoting_message(input)
  }
}
